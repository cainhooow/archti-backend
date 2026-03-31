use chrono::{Duration, NaiveDateTime};

use crate::domain::value_objects::document_vo::Document;

#[derive(Debug, Clone)]
pub struct Company {
    id: Option<String>,
    legal_name: String,
    trade_name: String,
    service_type: String,
    document: Document,
    contact_name: String,
    primary_phone: String,
    secondary_phone: Option<String>,
    operational_base: String,
    notes: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl Company {
    pub fn register(
        legal_name: String,
        trade_name: String,
        service_type: String,
        document: String,
        contact_name: String,
        primary_phone: String,
        secondary_phone: Option<String>,
        operational_base: String,
        notes: Option<String>,
    ) -> Result<Self, String> {
        if legal_name.is_empty() || trade_name.is_empty() {
            return Err("Legal name and trade name are required".to_string());
        }

        if service_type.is_empty() {
            return Err("Service type is required".to_string());
        }

        if contact_name.is_empty() {
            return Err("Contact name is required".to_string());
        }

        if document.is_empty() {
            return Err("Document is required".to_string());
        }

        let document = Document::parse(document)?;

        if primary_phone.is_empty() {
            return Err("Primary phone is required".to_string());
        }

        if let Some(secondary_phone) = secondary_phone.clone() {
            if secondary_phone.is_empty() {
                return Err("Secondary phone cannot be empty".to_string());
            }
        }

        if operational_base.is_empty() {
            return Err("Operational base is required".to_string());
        }

        if let Some(notes) = notes.clone() {
            if notes.is_empty() {
                return Err("Notes cannot be empty".to_string());
            }

            if notes.len() <= 1 {
                return Err("Notes must be longer than 1 character".to_string());
            }
        }

        Ok(Self {
            id: None,
            legal_name,
            trade_name,
            service_type,
            document,
            contact_name,
            primary_phone,
            secondary_phone,
            operational_base,
            notes,
            created_at: None,
            updated_at: None,
        })
    }

    pub fn restore(
        id: String,
        legal_name: String,
        trade_name: String,
        service_type: String,
        document: String,
        contact_name: String,
        primary_phone: String,
        secondary_phone: Option<String>,
        operational_base: String,
        notes: Option<String>,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Result<Self, String> {
        let document = Document::parse(document)?;
        Ok(Self {
            id: Some(id),
            legal_name,
            trade_name,
            service_type,
            document,
            contact_name,
            primary_phone,
            secondary_phone,
            operational_base,
            notes,
            created_at: created_at,
            updated_at: updated_at,
        })
    }

    pub fn change_legal_name(
        &mut self,
        legal_name: String,
        now: NaiveDateTime,
    ) -> Result<bool, String> {
        // if self
        //     .updated_at
        //     .map_or(true, |ts| ts < now - Duration::days(30))
        // {
        //     self.legal_name = legal_name;
        //     self.updated_at = Some(now);
        //     return Ok(true);
        // } else {
        //     return Err("You can only change your legal name every 30 days.".to_string());
        // }
        //
        self.legal_name = legal_name;
        self.updated_at = Some(now);
        Ok(true)
    }

    pub fn change_trade_name(
        &mut self,
        trade_name: String,
        now: NaiveDateTime,
    ) -> Result<bool, String> {
        self.trade_name = trade_name;
        self.updated_at = Some(now);
        Ok(true)
    }

    pub fn full_name(&self) -> String {
        format!("{} - {}", self.legal_name, self.trade_name)
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn legal_name(&self) -> &str {
        &self.legal_name
    }

    pub fn trade_name(&self) -> &str {
        &self.trade_name
    }

    pub fn service_type(&self) -> &str {
        &self.service_type
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn contact_name(&self) -> &str {
        &self.contact_name
    }

    pub fn primary_phone(&self) -> &str {
        &self.primary_phone
    }

    pub fn secondary_phone(&self) -> Option<&str> {
        self.secondary_phone.as_deref()
    }

    pub fn operational_base(&self) -> &str {
        &self.operational_base
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    pub fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }
}
