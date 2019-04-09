#[derive(Queryable, Debug)]
pub struct Art {
    pub id: String,
    /// Use local time
    pub date: DateTime<Local>,
    pub version: String,
    pub name: String,
    pub category: Option<Category>,
    pub ticket: Option<String>,
    pub slot: Option<String>,
    pub price: Option<String>,
    pub reference: Option<String>,
    pub description: Option<String>,
    pub customer: Option<Customer>,
}

#[derive(Queryable, Debug)]
pub struct Customer {
    pub name: String,
    pub contact: String,
    /// Payment information (paypal, crypto, ect)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<String>,
}

impl Customer {
    pub fn new() -> Customer {
        Customer {
            name: "".to_string(),
            contact: "".to_string(),
            payment: None,
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Customer
        where
            S: Into<String>,
    {
        self.name = name.into();
        self
    }

    pub fn contact<S: Into<String>>(mut self, cont: S) -> Customer
        where
            S: Into<String>,
    {
        self.contact = cont.into();
        self
    }

    pub fn payment<S: Into<String>>(mut self, pay: S) -> Customer
        where
            S: Into<String>,
    {
        self.payment = Some(pay.into());
        self
    }
}


impl Art {

}