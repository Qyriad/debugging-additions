impl UnitFactor {
    pub fn to_debug(&self) -> String {
        let kind_string = match &self.unit_id.kind {
            UnitKind::Base => format!("Base unit {}", self.unit_id.name),
            UnitKind::Derived(conversion_factor, unit) => {
                let factor_kinds = unit.factors
                    .iter()
                    .map(|factor| factor.to_debug())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Derived unit {} = {} * f[{}]", self.unit_id.name, conversion_factor.0, factor_kinds)
            },
        };
        let prefix_string = match self.prefix {
            Prefix::Metric(0) | Prefix::Binary(0) => format!(""),
            Prefix::Metric(val) => format!(" * metric {}", val),
            Prefix::Binary(val) => format!(" * binary {}", val),
        };
        let exponent_string = if self.exponent != Exponent::from_integer(1) {
            format!(" to the {}", pretty_exponent(&self.exponent))
        } else {
            String::new()
        };
        format!("{kind_string}{prefix_string}{exponent_string}")
    }
}

impl Unit {
    pub fn to_debug(&self) -> String {
        self.factors
            .iter()
            .map(|factor| factor.to_debug())
            .collect::<Vec<_>>()
            .join(" ×f ")
    }
}
