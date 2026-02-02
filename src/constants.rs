pub const API_BASE_URL: &str = "https://wpsapi.ahe.lodz.pl";
pub const API_LOGIN_PATH: &str = "/api/Profil/zaloguj";
pub const API_STUDENT_PATH: &str = "/api/Student/GetDaneStudenta";
pub const API_STUDENT_INDEXES_PATH: &str = "/api/Indeks/GETPobierzListeIndeksowDlaStudenta";
pub const API_PLAN_PATH: &str = "/api/PlanyZajec/GETPlanSzczegolowy";
pub const API_EXAM_PROTOCOL_PATH: &str =
  "/api/ProtokolyEgzaminacyjne/GetProtokolEgzaminacyjnySzczegolowy";
pub const API_EXAM_PROTOCOL_INTERMEDIATE_PATH: &str =
  "/api/ProtokolyEgzaminacyjne/GetProtokolEgzaminacyjnyPosredni";
pub const API_EXAM_FILTER_PATH: &str = "/api/Egzaminy/GETEgazminFiltr";
pub const API_CURRENT_ACADEMIC_YEAR_PATH: &str = "/api/Slowniki/GETPobierzAktualnyRokAkademicki";

pub const LOGIN_ROLE_ID: &str = "2";
pub const LOGIN_GRANT_TYPE: &str = "password";

pub const PLAN_INACTIVE_PARAM: &str = "CzyNieaktywnePlany=0";
pub const PLAN_LOADER_PARAM: &str = "loader=none";

pub const USER_AGENT: &str = concat!("ahe-ics/", env!("CARGO_PKG_VERSION"));

pub const DEFAULT_BIND_ADDR: &str = "0.0.0.0:8080";
pub const DEFAULT_CAL_PAST_DAYS: i64 = 60;
pub const DEFAULT_CAL_FUTURE_DAYS: i64 = 60;
pub const DEFAULT_CAL_LANG: &str = "pl";
pub const DEFAULT_EXAMS_ENABLED: bool = true;
pub const DEFAULT_JSON_ENABLED: bool = true;
pub const DEFAULT_OPENAPI_ENABLED: bool = true;

pub const TOKEN_REFRESH_GRACE_SECONDS: u64 = 30;
pub const ICS_CACHE_TTL_SECONDS: u64 = 600;
pub const EXAM_SETTLEMENT_NAME: &str = "egzamin";

pub const CALENDAR_TZ: &str = "Europe/Warsaw";
pub const ICS_CONTENT_TYPE: &str = "text/calendar; charset=utf-8";
pub const JSON_CONTENT_TYPE: &str = "application/json; charset=utf-8";
