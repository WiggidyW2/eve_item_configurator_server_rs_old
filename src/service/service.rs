use super::{
    authorization::{Kind as AuthKind, Scope as AuthScope},
};

use crate::{
    accessors::{Accessor, AccessorWrapper},
    validator::Validator,
    error::Error,
    pb,
};

use eve_character_validator::Response as ValidatorResponse;

pub struct Service<A> {
    pub accessor: AccessorWrapper<A>,
    pub validator: Validator,
}

impl<A: Accessor> Service<A> {
    pub async fn authorize(
        &self,
        name: &str,
        token: &str,
        kind: AuthKind,
        scope: AuthScope,
    ) -> Result<ValidatorResponse, Error> {
        // Format the name to match the authorization endpoint
        let name = get_auth_name(name, kind, scope);

        // Get the list of characters that are authorized for the endpoint
        let characters = self.accessor.get_characters(&name).await?;

        // Get the validator response
        let rep = self.validator.validate(token, characters.iter()).await?;

        // Return the validator response
        Ok(rep)
    }

    pub async fn list_items(
        &self,
        req: pb::ListReq,
    ) -> Result<pb::ListRep, Error> {
        let auth_rep = self.authorize(
            &req.name,
            &req.refresh_token,
            AuthKind::Read,
            AuthScope::Items,
        )
            .await?;
        if auth_rep.valid {
            self.list_items_authorized(req, auth_rep.refresh_token).await
        } else {
            Ok(self.list_items_unauthorized(auth_rep.refresh_token))
        }
    }

    pub async fn update_items(
        &self,
        req: pb::UpdateReq,
    ) -> Result<pb::UpdateRep, Error> {
        let auth_rep = self.authorize(
            &req.name,
            &req.refresh_token,
            AuthKind::Write,
            AuthScope::Items,
        )
            .await?;
        if auth_rep.valid {
            self.update_items_authorized(req, auth_rep.refresh_token).await
        } else {
            Ok(self.update_items_unauthorized(auth_rep.refresh_token))
        }
    }

    pub async fn list_characters(
        &self,
        req: pb::ListCharactersReq,
    ) -> Result<pb::ListCharactersRep, Error> {
        let auth_rep = self.authorize(
            &req.name,
            &req.refresh_token,
            AuthKind::Read,
            AuthScope::Characters,
        )
            .await?;
        if auth_rep.valid {
            self.list_characters_authorized(req, auth_rep.refresh_token).await
        } else {
            Ok(self.list_characters_unauthorized(auth_rep.refresh_token))
        }
    }

    pub async fn del_characters(
        &self,
        req: pb::DelCharactersReq,
    ) -> Result<pb::DelCharactersRep, Error> {
        let auth_rep = self.authorize(
            &req.name,
            &req.refresh_token,
            AuthKind::Write,
            AuthScope::Characters,
        )
            .await?;
        if auth_rep.valid {
            self.del_characters_authorized(req, auth_rep.refresh_token).await
        } else {
            Ok(self.del_characters_unauthorized(auth_rep.refresh_token))
        }
    }

    pub async fn add_characters(
        &self,
        req: pb::AddCharactersReq,
    ) -> Result<pb::AddCharactersRep, Error> {
        let auth_rep = self.authorize(
            &req.name,
            &req.refresh_token,
            AuthKind::Write,
            AuthScope::Characters,
        )
            .await?;
        if auth_rep.valid {
            self.add_characters_authorized(req, auth_rep.refresh_token).await
        } else {
            Ok(self.add_characters_unauthorized(auth_rep.refresh_token))
        }
    }
}

// Format the name to match the authorization endpoint
pub fn get_auth_name(
    name: &str,
    kind: AuthKind,
    scope: AuthScope,
) -> String {
    format!("{}_{}_{}", name, kind.as_str(), scope.as_str())
}
