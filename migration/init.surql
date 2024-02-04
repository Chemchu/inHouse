OPTION IMPORT;

DEFINE TABLE producto SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD nombre ON producto TYPE string ASSERT $value != NONE;
DEFINE FIELD cantidad ON producto TYPE number ASSERT $value != NONE;
DEFINE FIELD barcode ON producto TYPE string;