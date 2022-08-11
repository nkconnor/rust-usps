/// See [213 Secondary Address Unit Designators](https://pe.usps.com/text/pub28/28c2_003.htm)
/// and [Appendix C: Secondary Unit Designators](https://pe.usps.com/text/pub28/28apc_003.htm#ep538629)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnitDesignator {
    Blank,
    Apartment,
    Basement,
    Building,
    Department,
    Floor,
    Front,
    Hanger,
    Key,
    Lobby,
    Lot,
    Lower,
    Office,
    Penthouse,
    Pier,
    Rear,
    Room,
    Side,
    Slip,
    Space,
    Stop,
    Suite,
    Trailer,
    Unit,
    Upper,
}

impl UnitDesignator {
    /// The USPS approved abbreviation for the designator, for
    /// example **APT**, or **DEPT*.
    pub fn abbreviation(&self) -> &'static str {
        match self {
            UnitDesignator::Apartment => "APT",
            UnitDesignator::Basement => "BSMT",
            UnitDesignator::Blank => "",
            UnitDesignator::Building => "BLDG",
            UnitDesignator::Department => "DEPT",
            UnitDesignator::Floor => "FL",
            UnitDesignator::Front => "FRNT",
            UnitDesignator::Hanger => "HNGR",
            UnitDesignator::Key => "KEY",
            UnitDesignator::Lobby => "LBBY",
            UnitDesignator::Lot => "LOT",
            UnitDesignator::Lower => "LOWR",
            UnitDesignator::Office => "OFC",
            UnitDesignator::Penthouse => "PH",
            UnitDesignator::Pier => "PIER",
            UnitDesignator::Rear => "REAR",
            UnitDesignator::Room => "RM",
            UnitDesignator::Side => "SIDE",
            UnitDesignator::Slip => "SLIP",
            UnitDesignator::Space => "SPC",
            UnitDesignator::Stop => "STOP",
            UnitDesignator::Suite => "STE",
            UnitDesignator::Trailer => "TRLR",
            UnitDesignator::Unit => "UNIT",
            UnitDesignator::Upper => "UPPR",
        }
    }

    /// Whether the pound sign (#) should be used with this UnitDesignator.
    pub fn requires_pound_sign(&self) -> bool {
        match self {
            Self::Blank => true,
            _ => false,
        }
    }

    /// Whether a unit number, like the seven (7) in **LOT 7**,
    /// should accompany this abbreviation. Some designators, e.g. **Front** or
    /// **Basement** do not warrant this.
    pub fn requires_unit_number(&self) -> bool {
        match self {
            Self::Basement
            | Self::Front
            | Self::Lobby
            | Self::Lower
            | Self::Office
            | Self::Penthouse
            | Self::Rear
            | Self::Side
            | Self::Upper => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
