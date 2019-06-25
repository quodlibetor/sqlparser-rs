// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

///! This module defines
/// 1) a list of constants for every keyword that
/// can appear in SQLWord::keyword:
///    pub const KEYWORD = "KEYWORD"
/// 2) an `ALL_KEYWORDS` array with every keyword in it
///     This is not a list of *reserved* keywords: some of these can be
///     parsed as identifiers if the parser decides so. This means that
///     new keywords can be added here without affecting the parse result.
///
///     As a matter of fact, most of these keywords are not used at all
///     and could be removed.
/// 3) a `RESERVED_FOR_TABLE_ALIAS` array with keywords reserved in a
/// "table alias" context.

/// Defines a string constant for a single keyword: `kw_def!(SELECT);`
/// expands to `pub const SELECT = "SELECT";`
macro_rules! kw_def {
    ($ident:ident = $string_keyword:expr) => {
        pub const $ident: &'static str = $string_keyword;
    };
    ($ident:ident) => {
        kw_def!($ident = stringify!($ident));
    };
}

/// Expands to a list of `kw_def!()` invocations for each keyword
/// and defines an ALL_KEYWORDS array of the defined constants.
macro_rules! define_keywords {
    ($(
        $ident:ident $(= $string_keyword:expr)?
    ),*) => {
        $(kw_def!($ident $(= $string_keyword)?);)*

        pub const ALL_KEYWORDS: &[&str] = &[
            $($ident),*
        ];
    }
}

define_keywords!(
    ABS,
    ADD,
    ASC,
    ALL,
    ALLOCATE,
    ALTER,
    AND,
    ANY,
    APPLY,
    ARE,
    ARRAY,
    ARRAY_AGG,
    ARRAY_MAX_CARDINALITY,
    AS,
    ASENSITIVE,
    ASYMMETRIC,
    AT,
    ATOMIC,
    AUTHORIZATION,
    AVG,
    BEGIN,
    BEGIN_FRAME,
    BEGIN_PARTITION,
    BETWEEN,
    BIGINT,
    BINARY,
    BLOB,
    BOOLEAN,
    BOTH,
    BY,
    BYTEA,
    CALL,
    CALLED,
    CARDINALITY,
    CASCADE,
    CASCADED,
    CASE,
    CAST,
    CEIL,
    CEILING,
    CHAIN,
    CHAR,
    CHAR_LENGTH,
    CHARACTER,
    CHARACTER_LENGTH,
    CHECK,
    CLOB,
    CLOSE,
    COALESCE,
    COLLATE,
    COLLECT,
    COLUMN,
    COMMIT,
    COMMITTED,
    CONDITION,
    CONNECT,
    CONSTRAINT,
    CONTAINS,
    CONVERT,
    COPY,
    CORR,
    CORRESPONDING,
    COUNT,
    COVAR_POP,
    COVAR_SAMP,
    CREATE,
    CROSS,
    CSV,
    CUBE,
    CUME_DIST,
    CURRENT,
    CURRENT_CATALOG,
    CURRENT_DATE,
    CURRENT_DEFAULT_TRANSFORM_GROUP,
    CURRENT_PATH,
    CURRENT_ROLE,
    CURRENT_ROW,
    CURRENT_SCHEMA,
    CURRENT_TIME,
    CURRENT_TIMESTAMP,
    CURRENT_TRANSFORM_GROUP_FOR_TYPE,
    CURRENT_USER,
    CURSOR,
    CYCLE,
    DATE,
    DAY,
    DEALLOCATE,
    DEC,
    DECIMAL,
    DECLARE,
    DEFAULT,
    DELETE,
    DENSE_RANK,
    DEREF,
    DESC,
    DESCRIBE,
    DETERMINISTIC,
    DISCONNECT,
    DISTINCT,
    DOUBLE,
    DROP,
    DYNAMIC,
    EACH,
    ELEMENT,
    ELSE,
    END,
    END_FRAME,
    END_PARTITION,
    EQUALS,
    ESCAPE,
    EVERY,
    EXCEPT,
    EXEC,
    EXECUTE,
    EXISTS,
    EXP,
    EXTERNAL,
    EXTRACT,
    FALSE,
    FETCH,
    FIRST,
    FILTER,
    FIRST_VALUE,
    FLOAT,
    FLOOR,
    FOLLOWING,
    FOR,
    FOREIGN,
    FRAME_ROW,
    FREE,
    FROM,
    FULL,
    FUNCTION,
    FUSION,
    GET,
    GLOBAL,
    GRANT,
    GROUP,
    GROUPING,
    GROUPS,
    HAVING,
    HEADER,
    HOLD,
    HOUR,
    IDENTITY,
    IF,
    IN,
    INDICATOR,
    INNER,
    INOUT,
    INSENSITIVE,
    INSERT,
    INT,
    INTEGER,
    INTERSECT,
    INTERSECTION,
    INTERVAL,
    INTO,
    IS,
    ISOLATION,
    JOIN,
    KEY,
    LAG,
    LANGUAGE,
    LARGE,
    LAST_VALUE,
    LATERAL,
    LEAD,
    LEADING,
    LEFT,
    LEVEL,
    LIKE,
    LIKE_REGEX,
    LIMIT,
    LN,
    LOCAL,
    LOCALTIME,
    LOCALTIMESTAMP,
    LOCATION,
    LOWER,
    MATCH,
    MATERIALIZED,
    MAX,
    MEMBER,
    MERGE,
    METHOD,
    MIN,
    MINUTE,
    MOD,
    MODIFIES,
    MODULE,
    MONTH,
    MULTISET,
    NATIONAL,
    NATURAL,
    NCHAR,
    NCLOB,
    NEXT,
    NEW,
    NO,
    NONE,
    NORMALIZE,
    NOT,
    NTH_VALUE,
    NTILE,
    NULL,
    NULLIF,
    NUMERIC,
    OBJECT,
    OCTET_LENGTH,
    OCCURRENCES_REGEX,
    OF,
    OFFSET,
    OLD,
    ON,
    ONLY,
    OPEN,
    OR,
    ORDER,
    OUT,
    OUTER,
    OVER,
    OVERLAPS,
    OVERLAY,
    PARAMETER,
    PARTITION,
    PARQUET,
    PEEK,
    PERCENT,
    PERCENT_RANK,
    PERCENTILE_CONT,
    PERCENTILE_DISC,
    PERIOD,
    PORTION,
    POSITION,
    POSITION_REGEX,
    POWER,
    PRECEDES,
    PRECEDING,
    PRECISION,
    PREPARE,
    PRIMARY,
    PROCEDURE,
    RANGE,
    RANK,
    READ,
    READS,
    REAL,
    RECURSIVE,
    REF,
    REFERENCES,
    REFERENCING,
    REGCLASS,
    REGISTRY,
    REGR_AVGX,
    REGR_AVGY,
    REGR_COUNT,
    REGR_INTERCEPT,
    REGR_R2,
    REGR_SLOPE,
    REGR_SXX,
    REGR_SXY,
    REGR_SYY,
    RELEASE,
    REPEATABLE,
    RESTRICT,
    RESULT,
    RETURN,
    RETURNS,
    REVOKE,
    RIGHT,
    ROLLBACK,
    ROLLUP,
    ROW,
    ROW_NUMBER,
    ROWS,
    SAVEPOINT,
    SCHEMA,
    SCOPE,
    SCROLL,
    SEARCH,
    SECOND,
    SELECT,
    SENSITIVE,
    SERIALIZABLE,
    SESSION_USER,
    SET,
    SIMILAR,
    SINK,
    SMALLINT,
    SOME,
    SOURCE,
    SPECIFIC,
    SPECIFICTYPE,
    SQL,
    SQLEXCEPTION,
    SQLSTATE,
    SQLWARNING,
    SQRT,
    START,
    STATIC,
    STDDEV_POP,
    STDDEV_SAMP,
    STDIN,
    STORED,
    SUBMULTISET,
    SUBSTRING,
    SUBSTRING_REGEX,
    SUCCEEDS,
    SUM,
    SYMMETRIC,
    SYSTEM,
    SYSTEM_TIME,
    SYSTEM_USER,
    TABLE,
    TABLESAMPLE,
    TAIL,
    TEXT,
    THEN,
    TIES,
    TIME,
    TIMESTAMP,
    TIMEZONE_HOUR,
    TIMEZONE_MINUTE,
    TO,
    TRAILING,
    TRANSACTION,
    TRANSLATE,
    TRANSLATE_REGEX,
    TRANSLATION,
    TREAT,
    TRIGGER,
    TRUNCATE,
    TRIM,
    TRIM_ARRAY,
    TRUE,
    UESCAPE,
    UNBOUNDED,
    UNCOMMITTED,
    UNION,
    UNIQUE,
    UNKNOWN,
    UNNEST,
    UPDATE,
    UPPER,
    USER,
    USING,
    UUID,
    VALUE,
    VALUES,
    VALUE_OF,
    VAR_POP,
    VAR_SAMP,
    VARBINARY,
    VARCHAR,
    VARYING,
    VERSIONING,
    VIEW,
    WHEN,
    WHENEVER,
    WHERE,
    WIDTH_BUCKET,
    WINDOW,
    WITH,
    WITHIN,
    WITHOUT,
    WRITE,
    WORK,
    YEAR,
    ZONE,
    END_EXEC = "END-EXEC"
);

/// These keywords can't be used as a table alias, so that `FROM table_name alias`
/// can be parsed unambiguously without looking ahead.
pub const RESERVED_FOR_TABLE_ALIAS: &[&str] = &[
    // Reserved as both a table and a column alias:
    WITH, SELECT, WHERE, GROUP, HAVING, ORDER, LIMIT, OFFSET, FETCH, UNION, EXCEPT, INTERSECT,
    // Reserved only as a table alias in the `FROM`/`JOIN` clauses:
    ON, JOIN, INNER, CROSS, FULL, LEFT, RIGHT, NATURAL, USING,
    // for MSSQL-specific OUTER APPLY (seems reserved in most dialects)
    OUTER,
];

/// Can't be used as a column alias, so that `SELECT <expr> alias`
/// can be parsed unambiguously without looking ahead.
pub const RESERVED_FOR_COLUMN_ALIAS: &[&str] = &[
    // Reserved as both a table and a column alias:
    WITH, SELECT, WHERE, GROUP, HAVING, ORDER, LIMIT, OFFSET, FETCH, UNION, EXCEPT, INTERSECT,
    // Reserved only as a column alias in the `SELECT` clause:
    FROM,
];
