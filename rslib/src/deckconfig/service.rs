// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
use anki_proto::generic;

use crate::collection::Collection;
use crate::deckconfig::DeckConfSchema11;
use crate::deckconfig::DeckConfig;
use crate::deckconfig::DeckConfigId;
use crate::deckconfig::UpdateDeckConfigsRequest;
use crate::error;

impl crate::services::DeckConfigService for Collection {
    fn add_or_update_deck_config_legacy(
        &mut self,
        input: generic::Json,
    ) -> error::Result<anki_proto::deckconfig::DeckConfigId> {
        let conf: DeckConfSchema11 = serde_json::from_slice(&input.json)?;
        let mut conf: DeckConfig = conf.into();

        self.transact_no_undo(|col| {
            col.add_or_update_deck_config_legacy(&mut conf)?;
            Ok(anki_proto::deckconfig::DeckConfigId { dcid: conf.id.0 })
        })
        .map(Into::into)
    }

    fn all_deck_config_legacy(&mut self) -> error::Result<generic::Json> {
        let conf: Vec<DeckConfSchema11> = self
            .storage
            .all_deck_config()?
            .into_iter()
            .map(Into::into)
            .collect();
        serde_json::to_vec(&conf)
            .map_err(Into::into)
            .map(Into::into)
    }

    fn get_deck_config(
        &mut self,
        input: anki_proto::deckconfig::DeckConfigId,
    ) -> error::Result<anki_proto::deckconfig::DeckConfig> {
        Ok(Collection::get_deck_config(self, input.into(), true)?
            .unwrap()
            .into())
    }

    fn get_deck_config_legacy(
        &mut self,
        input: anki_proto::deckconfig::DeckConfigId,
    ) -> error::Result<generic::Json> {
        let conf = Collection::get_deck_config(self, input.into(), true)?.unwrap();
        let conf: DeckConfSchema11 = conf.into();
        Ok(serde_json::to_vec(&conf)?).map(Into::into)
    }

    fn new_deck_config_legacy(&mut self) -> error::Result<generic::Json> {
        serde_json::to_vec(&DeckConfSchema11::default())
            .map_err(Into::into)
            .map(Into::into)
    }

    fn remove_deck_config(
        &mut self,
        input: anki_proto::deckconfig::DeckConfigId,
    ) -> error::Result<()> {
        self.transact_no_undo(|col| col.remove_deck_config_inner(input.into()))
            .map(Into::into)
    }

    fn get_deck_configs_for_update(
        &mut self,
        input: anki_proto::decks::DeckId,
    ) -> error::Result<anki_proto::deckconfig::DeckConfigsForUpdate> {
        self.get_deck_configs_for_update(input.did.into())
    }

    fn update_deck_configs(
        &mut self,
        input: anki_proto::deckconfig::UpdateDeckConfigsRequest,
    ) -> error::Result<anki_proto::collection::OpChanges> {
        self.update_deck_configs(input.into()).map(Into::into)
    }
}

impl From<DeckConfig> for anki_proto::deckconfig::DeckConfig {
    fn from(c: DeckConfig) -> Self {
        anki_proto::deckconfig::DeckConfig {
            id: c.id.0,
            name: c.name,
            mtime_secs: c.mtime_secs.0,
            usn: c.usn.0,
            config: Some(c.inner),
        }
    }
}

impl From<anki_proto::deckconfig::UpdateDeckConfigsRequest> for UpdateDeckConfigsRequest {
    fn from(c: anki_proto::deckconfig::UpdateDeckConfigsRequest) -> Self {
        UpdateDeckConfigsRequest {
            target_deck_id: c.target_deck_id.into(),
            configs: c.configs.into_iter().map(Into::into).collect(),
            removed_config_ids: c.removed_config_ids.into_iter().map(Into::into).collect(),
            apply_to_children: c.apply_to_children,
            card_state_customizer: c.card_state_customizer,
            limits: c.limits.unwrap_or_default(),
            new_cards_ignore_review_limit: c.new_cards_ignore_review_limit,
        }
    }
}

impl From<anki_proto::deckconfig::DeckConfig> for DeckConfig {
    fn from(c: anki_proto::deckconfig::DeckConfig) -> Self {
        DeckConfig {
            id: c.id.into(),
            name: c.name,
            mtime_secs: c.mtime_secs.into(),
            usn: c.usn.into(),
            inner: c.config.unwrap_or_default(),
        }
    }
}

impl From<anki_proto::deckconfig::DeckConfigId> for DeckConfigId {
    fn from(dcid: anki_proto::deckconfig::DeckConfigId) -> Self {
        DeckConfigId(dcid.dcid)
    }
}
