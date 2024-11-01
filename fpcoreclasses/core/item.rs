use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents the type of an item in a receipt.
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ItemType {
    Sale = 0,
    Comment = 1,
    FooterComment = 2,
    SurchargeAmount = 3,
    DiscountAmount = 4,
}

/// Price Modifier Types
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PriceModifierType {
    /// There is no Price Modifier, so Price Modifier Value must be 0.
    None = 0,

    /// The Price Modifier Value represents the discount in percents.
    DiscountPercent = 1,

    /// The Price Modifier Value represents the discount amount.
    DiscountAmount = 2,

    /// The Price Modifier Value represents the surcharge in percents.
    SurchargePercent = 3,

    /// The Price Modifier Value represents the surcharge amount.
    SurchargeAmount = 4,
}

/// Represents the tax groups available for items.
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TaxGroup {
    Unspecified = 0,
    TaxGroup1 = 1,
    TaxGroup2 = 2,
    TaxGroup3 = 3,
    TaxGroup4 = 4,
    TaxGroup5 = 5,
    TaxGroup6 = 6,
    TaxGroup7 = 7,
    TaxGroup8 = 8,
}

/// Represents one line in a receipt.
/// Can be either a comment or a fiscal line.
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// The item code.
    pub item_code: i32,

    /// The type of the item row.
    #[serde(rename = "type")]
    pub item_type: ItemType,

    /// The text of the line.
    pub text: String,

    /// The tax group.
    pub tax_group: TaxGroup,

    /// The department. Department 0 means no department.
    pub department: i32,

    /// The quantity.
    pub quantity: f64,

    /// The unit price.
    pub unit_price: f64,

    /// The amount used in discount and surcharge amount types.
    pub amount: f64,

    /// The discounts, surcharges.
    pub price_modifier_value: f64,

    /// The PriceModifierType, None is default.
    pub price_modifier_type: PriceModifierType,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            item_code: 999,
            item_type: ItemType::Sale,
            text: String::new(),
            tax_group: TaxGroup::Unspecified,
            department: 0,
            quantity: 0.0,
            unit_price: 0.0,
            amount: 0.0,
            price_modifier_value: 0.0,
            price_modifier_type: PriceModifierType::None,
        }
    }
}
