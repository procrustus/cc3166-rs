#![warn(missing_docs)]

//! # Country Codes (ISO 3166)
//! The purpose of [ISO 3166][1] is to define internationally recognized codes 
//! of letters and/or numbers that we can use when we refer to countries and 
//! their subdivisions. However, it does not define the names of countries – 
//! this information comes from United Nations sources (Terminology Bulletin 
//! Country Names and the Country and Region Codes for Statistical Use 
//! maintained by the United Nations Statistics Divisions).
//! 
//! ISO 3166 is divided into 3 parts, where:
//! * [ISO 3166-1][3] refers to "Codes for the representation of names of 
//!   countries and their subdivisions – Part 1: Country codes".
//! * [ISO 3166-2][4] refers to "Codes for the representation of names of 
//!   countries and their subdivisions – Part 2: Country subdivision code"
//! * [ISO 3166-3][5] refers to "Codes for the representation of names of 
//!   countries and their subdivisions – Part 3: Code for formerly used names 
//!   of countries".
//! 
//! # Crate layout
//! The main purpose of this crate is to establish lightweight country entities 
//! based on the Alpha-2 codes defined in ISO 3166-1. Secondary purpose is to 
//! provide reference data for each defined country, this data is limited to 
//! Alpha-2 code, Alpha-3 code, Numeric code by default. 
//! 
//! Data is derived from open public sources only, as such data quality is not 
//! guaranteed.
//! 
//! pub enum [`Country`] where each country is defined by its alpha-2 code.
//! 
//! [`Country`] defines three constructors:
//!
//! * [`Country::from_alpha2()`] which takes a two-letter `&str` as input.
//! * [`Country::from_alpha3()`] which takes a three-letter `&str` as input.
//! * [`Country::from_numeric()`] which takes a `u32` as input.
//! 
//! [`Country`] has four attributes:
//! * [`Country.alpha2()`] which yields the country alpha-2 code as `&str`.
//! * [`Country.alpha3()`] which yields the country alpha-3 code as `&str`.
//! * [`Country.numeric()`] which yields the country numeric code as `u32`.
//! * [`Country.country_data()`] which yields a &[`CountryData`] containing
//!   above named attributes.
//! 
//! ## Features
//! 
//! Since country names are not defined by ISO 3166 they are outside of the 
//! default scope of this crate. However, using optional <language> features it 
//! provides country name translated into <language> using `in_<language>()`
//! function.
//! 
//! Currently the following "short name" translations are provided behind 
//! features:
//! 
//! * `chinese` => [`in_chinese()`]
//! * `english` => [`in_english()`]
//! * `finnish` => [`in_finnish()`]
//! * `french` => [`in_french()`]
//! * `russian` => [`in_russian()`]
//! * `spanish` => [`in_spanish()`]
//! * `swedish` => [`in_swedish()`]
//! 
//! It is relatively simple to extend to other languages as well.
//! 
//! # Usage (Basic)
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! cc3166 = { version = "0.1", git = "https://github.com/procrustus/cc3166-rs" }
//! ```
//! 
//! ```rust
//! use cc3316::Country;
//! 
//! let a3_str = "USA";
//! 
//! match Country::from_a3(a3_str) {
//!     Ok(code) => {
//!         // `Display` is implemented, displaying the Alpha-2 code.
//!         
//!         // `Debug` is implementented, diplaying the Country enum value.
//!         println!("{:?}", code); // "Country::US"
//!         // `alpha2()` returns the recommended general-purpose alpha-2 code.
//!         println!("  Alpha-2 code: {}", code.alpha2()); // "US"
//!         // `alpha3()` returns the alpha-3 code which is more closely related 
//!         // to the country name.
//!         println!("  Alpha-3 code: {}", code.alpha3()); // "USA"
//!         // `numeric()` returns the numeric code which can be useful if you 
//!         // need to avoid using Latin script.
//!         println!("  Numeric code: {}", code.numeric()); // 840
//!         // `country_data()` returns a reference to the underlying basic 
//!         // `CountryData` which includes the Alpha-2/-3 & numeric code.
//!         //
//!         // CountryData("US", "USA", 840) for `code`.
//!         println!("  Country data: {:?}", code.country_data()); 
//!     }
//!     Err(err) => panic!("Unable to parse country code `{}`: {}.", 
//!         a3_str, err),
//! }
//! ```
//! 
//! # Usage (with feature = ["english", "swedish"])
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! cc3166 = { 
//!    version = "0.1", 
//!    git = "https://github.com/procrustus/cc3166-rs", 
//!    features = ["english", "swedish"],
//! }
//! ```
//! 
//! ```rust
//! use cc3316::{Country, in_english, in_swedish};
//! 
//! let cc = Couuntry::US;
//! 
//! println!("{:?} is called `{}` in english.", cc, in_english(cc));
//! println!("{:?} is called `{}` in swedish.", cc, in_swedish(cc));
//! ```
//! 
//! [1]: https://www.iso.org/iso-3166-country-codes.html
//! [2]: https://www.iso.org/glossary-for-iso-3166.html
//! [3]: https://en.wikipedia.org/wiki/ISO_3166-1
//! [4]: https://en.wikipedia.org/wiki/ISO_3166-2
//! [5]: https://en.wikipedia.org/wiki/ISO_3166-3

pub(crate) mod attrs;
pub(crate) mod lang;
#[macro_use]
pub(crate) mod macros;
mod error;

use std::{
    convert::TryInto, 
    fmt::{Debug, Display}, 
    str::from_utf8_unchecked
};
use paste::paste;

pub use error::CountryError;

/// Total number of countries as defined in ISO 3166-1 as of 2021-09-09.
pub const NUM_COUNTRIES: usize = 249;

// Generates a `pub fn in_<language>(input: Country) -> &str` for each language
// defined. 
// NOTE: To add support for a new language.
//   1. Create a <language>.rs file under the "src/lang/" folder. Add the
//      translations for each of the defined alpha-2 codes like:
//        pub const CH: &str = "Switzerland";
//      See "lang/english.rs" for example.
//   2. Add the following to "src/lang/mod.rs":
//        #[cfg(feature = "<language>")]
//        pub mod <language>;
//   3. Add the following to "Cargo.toml":
//        <language> = []
generate_language!(chinese, english, finnish, french, russian, spanish, swedish);

// Generates CountryData structs for all countries defined under ISO 3166-1.
generate_country_data!(
    AD, AE, AF, AG, AI, AL, AM, AO, AQ, AR, AS, AT, AU, AW, AX, AZ, BA, BB, BD, 
    BE, BF, BG, BH, BI, BJ, BL, BM, BN, BO, BQ, BR, BS, BT, BV, BW, BY, BZ, CA, 
    CC, CD, CF, CG, CH, CI, CK, CL, CM, CN, CO, CR, CU, CV, CW, CX, CY, CZ, DE, 
    DJ, DK, DM, DO, DZ, EC, EE, EG, EH, ER, ES, ET, FI, FJ, FK, FM, FO, FR, GA, 
    GB, GD, GE, GF, GG, GH, GI, GL, GM, GN, GP, GQ, GR, GS, GT, GU, GW, GY, HK, 
    HM, HN, HR, HT, HU, ID, IE, IL, IM, IN, IO, IQ, IR, IS, IT, JE, JM, JO, JP, 
    KE, KG, KH, KI, KM, KN, KP, KR, KW, KY, KZ, LA, LB, LC, LI, LK, LR, LS, LT, 
    LU, LV, LY, MA, MC, MD, ME, MF, MG, MH, MK, ML, MM, MN, MO, MP, MQ, MR, MS, 
    MT, MU, MV, MW, MX, MY, MZ, NA, NC, NE, NF, NG, NI, NL, NO, NP, NR, NU, NZ, 
    OM, PA, PE, PF, PG, PH, PK, PL, PM, PN, PR, PS, PT, PW, PY, QA, RE, RO, RS, 
    RU, RW, SA, SB, SC, SD, SE, SG, SH, SI, SJ, SK, SL, SM, SN, SO, SR, SS, ST, 
    SV, SX, SY, SZ, TC, TD, TF, TG, TH, TJ, TK, TL, TM, TN, TO, TR, TT, TV, TW, 
    TZ, UA, UG, UM, US, UY, UZ, VA, VC, VE, VG, VI, VN, VU, WF, WS, YE, YT, ZA, 
    ZM, ZW
);

/// Returns a pointer to [`CountryData`] for [`Country`]. For internal use.
fn as_country_data(input: Country) -> &'static CountryData {
    cd_match_stmt!(
        input,
        AD, AE, AF, AG, AI, AL, AM, AO, AQ, AR, AS, AT, AU, AW, AX, AZ, BA, BB, 
        BD, BE, BF, BG, BH, BI, BJ, BL, BM, BN, BO, BQ, BR, BS, BT, BV, BW, BY, 
        BZ, CA, CC, CD, CF, CG, CH, CI, CK, CL, CM, CN, CO, CR, CU, CV, CW, CX, 
        CY, CZ, DE, DJ, DK, DM, DO, DZ, EC, EE, EG, EH, ER, ES, ET, FI, FJ, FK, 
        FM, FO, FR, GA, GB, GD, GE, GF, GG, GH, GI, GL, GM, GN, GP, GQ, GR, GS, 
        GT, GU, GW, GY, HK, HM, HN, HR, HT, HU, ID, IE, IL, IM, IN, IO, IQ, IR, 
        IS, IT, JE, JM, JO, JP, KE, KG, KH, KI, KM, KN, KP, KR, KW, KY, KZ, LA, 
        LB, LC, LI, LK, LR, LS, LT, LU, LV, LY, MA, MC, MD, ME, MF, MG, MH, MK, 
        ML, MM, MN, MO, MP, MQ, MR, MS, MT, MU, MV, MW, MX, MY, MZ, NA, NC, NE, 
        NF, NG, NI, NL, NO, NP, NR, NU, NZ, OM, PA, PE, PF, PG, PH, PK, PL, PM, 
        PN, PR, PS, PT, PW, PY, QA, RE, RO, RS, RU, RW, SA, SB, SC, SD, SE, SG, 
        SH, SI, SJ, SK, SL, SM, SN, SO, SR, SS, ST, SV, SX, SY, SZ, TC, TD, TF, 
        TG, TH, TJ, TK, TL, TM, TN, TO, TR, TT, TV, TW, TZ, UA, UG, UM, US, UY, 
        UZ, VA, VC, VE, VG, VI, VN, VU, WF, WS, YE, YT, ZA, ZM, ZW        
    )
}



#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Country as defined by the alpha-2 code (most widely used).
pub enum Country {
    /// Andorra
    AD, 
    /// United Arab Emirates (the)
    AE, 
    /// Afghanistan
    AF, 
    /// Antigua and Barbuda
    AG, 
    /// Anguilla
    AI, 
    /// Albania
    AL, 
    /// Armenia
    AM, 
    /// Angola
    AO, 
    /// Antarctica
    AQ, 
    /// Argentina
    AR, 
    /// American Samoa
    AS, 
    /// Austria
    AT, 
    /// Australia
    AU, 
    /// Aruba
    AW, 
    /// Åland Islands
    AX, 
    /// Azerbaijan
    AZ, 
    /// Bosnia and Herzegovina
    BA, 
    /// Barbados
    BB, 
    /// Bangladesh
    BD, 
    /// Belgium
    BE, 
    /// Burkina Faso
    BF, 
    /// Bulgaria
    BG, 
    /// Bahrain
    BH, 
    /// Burundi
    BI, 
    /// Benin
    BJ, 
    /// Saint Barthélemy
    BL, 
    /// Bermuda
    BM, 
    /// Brunei Darussalam
    BN, 
    /// Bolivia (Plurinational State of)
    BO, 
    /// Bonaire, Sint Eustatius and Saba
    BQ, 
    /// Brazil
    BR, 
    /// Bahamas (the)
    BS, 
    /// Bhutan
    BT, 
    /// Bouvet Island
    BV, 
    /// Botswana
    BW, 
    /// Belarus
    BY, 
    /// Belize
    BZ, 
    /// Canada
    CA, 
    /// Cocos (Keeling) Islands (the)
    CC, 
    /// Congo (the Democratic Republic of the)
    CD, 
    /// Central African Republic (the)
    CF, 
    /// Congo (the)
    CG, 
    /// Switzerland
    CH, 
    /// Côte d'Ivoire
    CI, 
    /// Cook Islands (the)
    CK, 
    /// Chile
    CL, 
    /// Cameroon
    CM, 
    /// China
    CN, 
    /// Colombia
    CO, 
    /// Costa Rica
    CR, 
    /// Cuba
    CU, 
    /// Cabo Verde
    CV, 
    /// Curaçao
    CW, 
    /// Christmas Island
    CX, 
    /// Cyprus
    CY, 
    /// Czechia
    CZ, 
    /// Germany
    DE, 
    /// Djibouti
    DJ, 
    /// Denmark
    DK, 
    /// Dominica
    DM, 
    /// Dominican Republic (the)
    DO, 
    /// Algeria
    DZ, 
    /// Ecuador
    EC, 
    /// Estonia
    EE, 
    /// Egypt
    EG, 
    /// Western Sahara*
    EH, 
    /// Eritrea
    ER, 
    /// Spain
    ES, 
    /// Ethiopia
    ET, 
    /// Finland
    FI, 
    /// Fiji
    FJ, 
    /// Falkland Islands (the) [Malvinas]
    FK, 
    /// Micronesia (Federated States of)
    FM, 
    /// Faroe Islands (the)
    FO, 
    /// France
    FR, 
    /// Gabon
    GA, 
    /// United Kingdom of Great Britain and Northern Ireland (the)
    GB, 
    /// Grenada
    GD, 
    /// Georgia
    GE, 
    /// French Guiana
    GF, 
    /// Guernsey
    GG, 
    /// Ghana
    GH, 
    /// Gibraltar
    GI, 
    /// Greenland
    GL, 
    /// Gambia (the)
    GM, 
    /// Guinea
    GN, 
    /// Guadeloupe
    GP, 
    /// Equatorial Guinea
    GQ, 
    /// Greece
    GR, 
    /// South Georgia and the South Sandwich Islands
    GS, 
    /// Guatemala
    GT, 
    /// Guam
    GU, 
    /// Guinea-Bissau
    GW, 
    /// Guyana
    GY, 
    /// Hong Kong
    HK, 
    /// Heard Island and McDonald Islands
    HM, 
    /// Honduras
    HN, 
    /// Croatia
    HR, 
    /// Haiti
    HT, 
    /// Hungary
    HU, 
    /// Indonesia
    ID, 
    /// Ireland
    IE, 
    /// Israel
    IL, 
    /// Isle of Man
    IM, 
    /// India
    IN, 
    /// British Indian Ocean Territory (the)
    IO, 
    /// Iraq
    IQ, 
    /// Iran (Islamic Republic of)
    IR, 
    /// Iceland
    IS, 
    /// Italy
    IT, 
    /// Jersey
    JE, 
    /// Jamaica
    JM, 
    /// Jordan
    JO, 
    /// Japan
    JP, 
    /// Kenya
    KE, 
    /// Kyrgyzstan
    KG, 
    /// Cambodia
    KH, 
    /// Kiribati
    KI, 
    /// Comoros (the)
    KM, 
    /// Saint Kitts and Nevis
    KN, 
    /// Korea (the Democratic People's Republic of)
    KP, 
    /// Korea (the Republic of)
    KR, 
    /// Kuwait
    KW, 
    /// Cayman Islands (the)
    KY, 
    /// Kazakhstan
    KZ, 
    /// Lao People's Democratic Republic (the)
    LA, 
    /// Lebanon
    LB, 
    /// Saint Lucia
    LC, 
    /// Liechtenstein
    LI, 
    /// Sri Lanka
    LK, 
    /// Liberia
    LR, 
    /// Lesotho
    LS, 
    /// Lithuania
    LT, 
    /// Luxembourg
    LU, 
    /// Latvia
    LV, 
    /// Libya
    LY, 
    /// Morocco
    MA, 
    /// Monaco
    MC, 
    /// Moldova (the Republic of)
    MD, 
    /// Montenegro
    ME, 
    /// Saint Martin (French part)
    MF, 
    /// Madagascar
    MG, 
    /// Marshall Islands (the)
    MH, 
    /// North Macedonia
    MK, 
    /// Mali
    ML, 
    /// Myanmar
    MM, 
    /// Mongolia
    MN, 
    /// Macao
    MO, 
    /// Northern Mariana Islands (the)
    MP, 
    /// Martinique
    MQ, 
    /// Mauritania
    MR, 
    /// Montserrat
    MS, 
    /// Malta
    MT, 
    /// Mauritius
    MU, 
    /// Maldives
    MV, 
    /// Malawi
    MW, 
    /// Mexico
    MX, 
    /// Malaysia
    MY, 
    /// Mozambique
    MZ, 
    /// Namibia
    NA, 
    /// New Caledonia
    NC, 
    /// Niger (the)
    NE, 
    /// Norfolk Island
    NF, 
    /// Nigeria
    NG, 
    /// Nicaragua
    NI, 
    /// Netherlands (the)
    NL, 
    /// Norway
    NO, 
    /// Nepal
    NP, 
    /// Nauru
    NR, 
    /// Niue
    NU, 
    /// New Zealand
    NZ, 
    /// Oman
    OM, 
    /// Panama
    PA, 
    /// Peru
    PE, 
    /// French Polynesia
    PF, 
    /// Papua New Guinea
    PG, 
    /// Philippines (the)
    PH, 
    /// Pakistan
    PK, 
    /// Poland
    PL, 
    /// Saint Pierre and Miquelon
    PM, 
    /// Pitcairn
    PN, 
    /// Puerto Rico
    PR, 
    /// Palestine, State of
    PS, 
    /// Portugal
    PT, 
    /// Palau
    PW, 
    /// Paraguay
    PY, 
    /// Qatar
    QA, 
    /// Réunion
    RE, 
    /// Romania
    RO, 
    /// Serbia
    RS, 
    /// Russian Federation (the)
    RU, 
    /// Rwanda
    RW, 
    /// Saudi Arabia
    SA, 
    /// Solomon Islands
    SB, 
    /// Seychelles
    SC, 
    /// Sudan (the)
    SD, 
    /// Sweden
    SE, 
    /// Singapore
    SG, 
    /// Saint Helena, Ascension and Tristan da Cunha
    SH, 
    /// Slovenia
    SI, 
    /// Svalbard and Jan Mayen
    SJ, 
    /// Slovakia
    SK, 
    /// Sierra Leone
    SL, 
    /// San Marino
    SM, 
    /// Senegal
    SN, 
    /// Somalia
    SO, 
    /// Suriname
    SR, 
    /// South Sudan
    SS, 
    /// Sao Tome and Principe
    ST, 
    /// El Salvador
    SV, 
    /// Sint Maarten (Dutch part)
    SX, 
    /// Syrian Arab Republic (the)
    SY, 
    /// Eswatini
    SZ, 
    /// Turks and Caicos Islands (the)
    TC, 
    /// Chad
    TD, 
    /// French Southern Territories (the)
    TF, 
    /// Togo
    TG, 
    /// Thailand
    TH, 
    /// Tajikistan
    TJ, 
    /// Tokelau
    TK, 
    /// Timor-Leste
    TL, 
    /// Turkmenistan
    TM, 
    /// Tunisia
    TN, 
    /// Tonga
    TO, 
    /// Turkey
    TR, 
    /// Trinidad and Tobago
    TT, 
    /// Tuvalu
    TV, 
    /// Taiwan (Province of China)
    TW, 
    /// Tanzania, the United Republic of
    TZ, 
    /// Ukraine
    UA, 
    /// Uganda
    UG, 
    /// United States Minor Outlying Islands (the)
    UM, 
    /// United States of America (the)
    US, 
    /// Uruguay
    UY, 
    /// Uzbekistan
    UZ, 
    /// Holy See (the)
    VA, 
    /// Saint Vincent and the Grenadines
    VC, 
    /// Venezuela (Bolivarian Republic of)
    VE, 
    /// Virgin Islands (British)
    VG, 
    /// Virgin Islands (U.S.)
    VI, 
    /// Viet Nam
    VN, 
    /// Vanuatu
    VU, 
    /// Wallis and Futuna
    WF, 
    /// Samoa
    WS, 
    /// Yemen
    YE, 
    /// Mayotte
    YT, 
    /// South Africa
    ZA, 
    /// Zambia
    ZM, 
    /// Zimbabwe
    ZW, 
}

impl Country {
    /// Initialize a [`Country`] enum from an alpha-2 code as &str.
    ///
    /// # Examples
    /// ```
    /// use cc3166::Country;
    ///
    /// if let Ok(res) = Country::from_alpha2("CN") {
    ///     assert_eq!(res, Country::CN);
    /// }
    /// ```
    pub fn from_alpha2(input: &str) -> Result<Country, CountryError> {
        if input.as_bytes().len() != 2 { 
            return Err(CountryError::UnexpectedLen { 
                was: input.as_bytes().len(), 
                expected: 2
            }); 
        }
        let cc: &[u8; 2] = input.as_bytes().try_into().unwrap();
    
        from_match_stmt!(cc, a2);
    
        Err(CountryError::InvalidA2 { was: input.to_owned() })
    }
    /// Initialize a [`Country`] enum from an alpha-3 code as &str.
    ///
    /// # Examples
    /// ```
    /// use cc3166::Country;
    ///
    /// if let Ok(res) = Country::from_alpha3("JPN") {
    ///     assert_eq!(res, Country::JP);
    /// }
    /// ```
    pub fn from_alpha3(input: &str) -> Result<Country, CountryError> {
        if input.as_bytes().len() != 3 { 
            return Err(CountryError::UnexpectedLen { 
                was: input.as_bytes().len(), 
                expected: 3
            }); 
        }
        let cc: &[u8; 3] = input.as_bytes().try_into().unwrap();
    
        from_match_stmt!(cc, a3);
    
        Err(CountryError::InvalidA3 { was: input.to_owned() })    
    }
    /// Initialize a [`Country`] enum from a numeric code as u32.
    ///
    /// # Examples
    /// ```
    /// use cc3166::Country;
    /// 
    /// if let Ok(res) = Country::from_numeric(250) {
    ///     assert_eq!(res, Country::FR);
    /// }
    /// ```
    pub fn from_numeric(input: u32) -> Result<Country, CountryError> {
        from_match_stmt!(input, num);
    
        Err(CountryError::InvalidNum { was: input })    
    }
    /// Returns the alpha-2 code for [`Country`] as &str.
    ///
    /// # Examples
    /// ```
    /// use cc3166::Country;
    ///
    /// let cc = Country::ES;
    /// assert_eq!(cc.alpha2(), "ES");
    /// ```
    pub fn alpha2(&self) -> &str {
        as_country_data(*self).alpha2()
    }
    /// Returns the alpha-3 code for [`Country`] as &str.
    ///
    /// # Examples
    /// ```
    /// use cc3166::Country;
    ///
    /// let cc = Country::DE;
    /// assert_eq!(cc.alpha3(), "DEU");
    /// ```
    pub fn alpha3(&self) -> &str {
        as_country_data(*self).alpha3()
    }
    /// Returns the numeric code for [`Country`] as u32.
    ///
    /// # Examples
    /// ```
    /// use cc3166::Country;
    ///
    /// let cc = Country::AU;
    /// assert_eq!(cc.numeric(), 36);
    /// ```
    pub fn numeric(&self) -> u32 {
        as_country_data(*self).numeric()
    }
    /// Returns the &[`CountryData`] for [`Country`].
    ///
    /// # Examples
    /// ```
    /// use cc3166::{Country, AU};
    ///
    /// let cc = Country::AU;
    /// assert_eq!(cc.country_data(), AU);
    /// ```
    pub fn country_data(&self) -> &CountryData {
        as_country_data(*self)
    }
}

impl Display for Country {
    /// Two-letter code (alpha-2) which is recommended as the general-purpose 
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.alpha2())
    }
}

impl Debug for Country {
    /// Custom Debug implementation to make underlying data human readable.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Country::{}", &self.alpha2()))
    }
}

type A2 = &'static [u8; 2]; // Alpha-2 code
type A3 = &'static [u8; 3]; // Alpha-3 code 
type Num = u32; // Numeric code

#[derive(Clone, Copy, PartialEq, Eq)]
/// Storage for a countrys basic ISO 3166-1 data (Alpha-2, Alpha-3 & Numeric).
pub struct CountryData(A2, A3, Num);

impl CountryData {
    /// Returns the alpha-2 code as `&str`.
    pub fn alpha2(&self) -> &str {
        unsafe {
            // SAFETY:
            // This is safe since all Country -> CountryData are accounted for,
            // and all CountryData alpha-2 code fields are valid utf-8.
            from_utf8_unchecked(self.0)
        }
    }
    /// Returns the alpha-3 code as `&str`.
    pub fn alpha3(&self) -> &str {
        unsafe {
            // SAFETY:
            // This is safe since all Country -> CountryData are accounted for,
            // and all CountryData alpha-3 code fields are valid utf-8.
            from_utf8_unchecked(self.1)
        }
    }
    /// Returns the numeric code as `u32`.
    pub fn numeric(&self) -> u32 {
        self.2
    }
}

impl Debug for CountryData {
    /// Custom Debug implementation to make underlying data human readable.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CountryData")
            .field(&self.alpha2())
            .field(&self.alpha3())
            .field(&self.numeric())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn various_alpha2_tests() {
        // Too short
        assert_eq!(
            Country::from_alpha2("S").unwrap_err(), 
            error::CountryError::UnexpectedLen { was: 1, expected: 2 }
        );
        // Too long
        assert_eq!(
            Country::from_alpha2("SWE").unwrap_err(), 
            error::CountryError::UnexpectedLen { was: 3, expected: 2 }
        );
        // Unknown country
        assert_eq!(
            Country::from_alpha2("XY").unwrap_err(), 
            error::CountryError::InvalidA2 { was: "XY".to_string() }
        );
        // `input` must be uppercase
        assert_eq!(
            Country::from_alpha2("se").unwrap_err(), 
            error::CountryError::InvalidA2 { was: "se".to_string() }
        );
        // Valid country
        assert_eq!(Country::from_alpha2("SE").unwrap(), Country::SE);
    }

    #[test]
    fn various_alpha3_tests() {
        // Too short
        assert_eq!(
            Country::from_alpha3("SE").unwrap_err(), 
            error::CountryError::UnexpectedLen { was: 2, expected: 3 }
        );
        // Too long
        assert_eq!(
            Country::from_alpha3("SWED").unwrap_err(), 
            error::CountryError::UnexpectedLen { was: 4, expected: 3 }
        );
        // Unknown country
        assert_eq!(
            Country::from_alpha3("XYZ").unwrap_err(), 
            error::CountryError::InvalidA3 { was: "XYZ".to_string() }
        );
       // `input` must be uppercase
       assert_eq!(
            Country::from_alpha3("swe").unwrap_err(), 
            error::CountryError::InvalidA3 { was: "swe".to_string() }
        );
        // Valid country
        assert_eq!(Country::from_alpha3("SWE").unwrap(), Country::SE);
    }

    #[test]
    fn various_numeric_tests() {
        // Unknown country
        assert_eq!(
            Country::from_numeric(0).unwrap_err(), 
            error::CountryError::InvalidNum { was: 0 }
        );
        // Valid country
        assert_eq!(Country::from_numeric(752).unwrap(), Country::SE);
    }

    #[test]
    fn various_attribute_tests() {
        assert_eq!(Country::SE.alpha2(), "SE");
        assert_ne!(Country::SE.alpha2(), "US");

        assert_eq!(Country::SE.alpha3(), "SWE");
        assert_ne!(Country::SE.alpha3(), "USA");

        assert_eq!(Country::SE.numeric(), 752);
        assert_ne!(Country::SE.numeric(), 999);

        assert_eq!(Country::SE.country_data(), SE);
        assert_ne!(Country::SE.country_data(), US);
    }

    #[cfg(feature = "chinese")]
    #[test]
    fn in_chinese_test() {
        assert_eq!(in_chinese(Country::WF), "瓦利斯和富图纳");
    }

    #[cfg(feature = "english")]
    #[test]
    fn in_english_test() {
        assert_eq!(in_english(Country::WF), "Wallis and Futuna");
    }

    #[cfg(feature = "finnish")]
    #[test]
    fn in_finnish_test() {
        assert_eq!(in_finnish(Country::WF), "Wallis ja Futunasaaret");
    }

    #[cfg(feature = "french")]
    #[test]
    fn in_french_test() {
        assert_eq!(in_french(Country::WF), "Wallis-et-Futuna");
    }

    #[cfg(feature = "russian")]
    #[test]
    fn in_russian_test() {
        assert_eq!(in_russian(Country::WF), "Уоллис и Футуна");
    }

    #[cfg(feature = "spanish")]
    #[test]
    fn in_spanish_test() {
        assert_eq!(in_spanish(Country::WF), "Wallis y Futuna");
    }

    #[cfg(feature = "swedish")]
    #[test]
    fn in_swedish_test() {
        assert_eq!(in_swedish(Country::WF), "Wallis- och Futunaöarna");
    }
}
