// Create match statement for [`as_country_data()`] query.
macro_rules! cd_match_stmt {
    ($input:ident, $($res:ident),*) => {
        match $input {
            $(Country::$res => { crate::$res }, )*
        }
    };
}

// Create match statement for [`from_<x>()`] query.
macro_rules! from_match_stmt {
    ($input:ident, $suffix:ident) => {
        from_match_internal!(
            $input,
            $suffix,
            AD, AE, AF, AG, AI, AL, AM, AO, AQ, AR, AS, AT, AU, AW, AX, AZ, BA, 
            BB, BD, BE, BF, BG, BH, BI, BJ, BL, BM, BN, BO, BQ, BR, BS, BT, BV, 
            BW, BY, BZ, CA, CC, CD, CF, CG, CH, CI, CK, CL, CM, CN, CO, CR, CU, 
            CV, CW, CX, CY, CZ, DE, DJ, DK, DM, DO, DZ, EC, EE, EG, EH, ER, ES, 
            ET, FI, FJ, FK, FM, FO, FR, GA, GB, GD, GE, GF, GG, GH, GI, GL, GM, 
            GN, GP, GQ, GR, GS, GT, GU, GW, GY, HK, HM, HN, HR, HT, HU, ID, IE, 
            IL, IM, IN, IO, IQ, IR, IS, IT, JE, JM, JO, JP, KE, KG, KH, KI, KM, 
            KN, KP, KR, KW, KY, KZ, LA, LB, LC, LI, LK, LR, LS, LT, LU, LV, LY, 
            MA, MC, MD, ME, MF, MG, MH, MK, ML, MM, MN, MO, MP, MQ, MR, MS, MT, 
            MU, MV, MW, MX, MY, MZ, NA, NC, NE, NF, NG, NI, NL, NO, NP, NR, NU, 
            NZ, OM, PA, PE, PF, PG, PH, PK, PL, PM, PN, PR, PS, PT, PW, PY, QA, 
            RE, RO, RS, RU, RW, SA, SB, SC, SD, SE, SG, SH, SI, SJ, SK, SL, SM, 
            SN, SO, SR, SS, ST, SV, SX, SY, SZ, TC, TD, TF, TG, TH, TJ, TK, TL, 
            TM, TN, TO, TR, TT, TV, TW, TZ, UA, UG, UM, US, UY, UZ, VA, VC, VE, 
            VG, VI, VN, VU, WF, WS, YE, YT, ZA, ZM, ZW        

        )
    };
}

// internal match statement generator for macro `from_match_stmt!()`.
macro_rules! from_match_internal {
    ($input:ident, $suffix:ident, $($res:ident),*) => {
        match $input {
            $(attrs::$suffix::$res => { return Ok(Country::$res); }, )*
            _ => { },
        };

    };
}


// internal match statement generator for macro `generate_language!()`.
// NOTE: does not compile if under mod `genlang`.
macro_rules! lang_match_internal {
    ($input:ident, $lang:ident, $($res:ident),*) => {
        match $input {
            $(Country::$res => { lang::$lang::$res }, )*
        }
    };
}


pub(crate) mod genlang {
    //! NOTE: paste!() macro did not work well in multiple macros unless in 
    //! separate mods.

    /// Create in_<language>() functions for featured optional languages.
    #[macro_export]
    macro_rules! generate_language {
        ($($lang:tt),*) => {
            $(
                paste! {
                    #[cfg(any(feature = $lang:lower, doc))]
                    #[doc = "Returns Country name in `" $lang:camel "`."]
                    pub fn [<in_ $lang:lower>](input: Country) -> &'static str {
                        lang_match_internal!(
                            input,
                            $lang,
                            AD, AE, AF, AG, AI, AL, AM, AO, AQ, AR, AS, AT, AU, 
                            AW, AX, AZ, BA, BB, BD, BE, BF, BG, BH, BI, BJ, BL, 
                            BM, BN, BO, BQ, BR, BS, BT, BV, BW, BY, BZ, CA, CC, 
                            CD, CF, CG, CH, CI, CK, CL, CM, CN, CO, CR, CU, CV, 
                            CW, CX, CY, CZ, DE, DJ, DK, DM, DO, DZ, EC, EE, EG, 
                            EH, ER, ES, ET, FI, FJ, FK, FM, FO, FR, GA, GB, GD, 
                            GE, GF, GG, GH, GI, GL, GM, GN, GP, GQ, GR, GS, GT, 
                            GU, GW, GY, HK, HM, HN, HR, HT, HU, ID, IE, IL, IM, 
                            IN, IO, IQ, IR, IS, IT, JE, JM, JO, JP, KE, KG, KH, 
                            KI, KM, KN, KP, KR, KW, KY, KZ, LA, LB, LC, LI, LK, 
                            LR, LS, LT, LU, LV, LY, MA, MC, MD, ME, MF, MG, MH, 
                            MK, ML, MM, MN, MO, MP, MQ, MR, MS, MT, MU, MV, MW, 
                            MX, MY, MZ, NA, NC, NE, NF, NG, NI, NL, NO, NP, NR, 
                            NU, NZ, OM, PA, PE, PF, PG, PH, PK, PL, PM, PN, PR, 
                            PS, PT, PW, PY, QA, RE, RO, RS, RU, RW, SA, SB, SC, 
                            SD, SE, SG, SH, SI, SJ, SK, SL, SM, SN, SO, SR, SS, 
                            ST, SV, SX, SY, SZ, TC, TD, TF, TG, TH, TJ, TK, TL, 
                            TM, TN, TO, TR, TT, TV, TW, TZ, UA, UG, UM, US, UY, 
                            UZ, VA, VC, VE, VG, VI, VN, VU, WF, WS, YE, YT, ZA, 
                            ZM, ZW        
                        )
                    }
                }
            )*
        }
    }
}

pub(crate) mod gencd {
    //! NOTE: paste!() macro did not work well in multiple macros unless in 
    //! separate mods.

    /// Creates const &[`CountryData`] structs for for all defined countries.
    #[macro_export]
    macro_rules! generate_country_data {
        ($($c:ident),*) => {
            $(
                paste! {
                    #[doc = "Basic country data for `" $c:upper "`."]
                    pub const $c: &CountryData = &CountryData(
                        attrs::a2::$c, attrs::a3::$c, attrs::num::$c
                    );
                }
            )*
            
            #[doc = "List of all countries [`CountryData`]."]                
            pub const COUNTRY_DATA: &'static [&'static CountryData] = &[
                $($c, )*
            ];
        };
    }
}
