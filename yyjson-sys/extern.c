#include "yyjson/src/yyjson.h"

// Static wrappers

__uint16_t __bswap_16__extern(__uint16_t __bsx) { return __bswap_16(__bsx); }
__uint32_t __bswap_32__extern(__uint32_t __bsx) { return __bswap_32(__bsx); }
__uint64_t __bswap_64__extern(__uint64_t __bsx) { return __bswap_64(__bsx); }
__uint16_t __uint16_identity__extern(__uint16_t __x) { return __uint16_identity(__x); }
__uint32_t __uint32_identity__extern(__uint32_t __x) { return __uint32_identity(__x); }
__uint64_t __uint64_identity__extern(__uint64_t __x) { return __uint64_identity(__x); }
yyjson_doc * yyjson_read__extern(const char *dat, size_t len, yyjson_read_flag flg) { return yyjson_read(dat, len, flg); }
size_t yyjson_read_max_memory_usage__extern(size_t len, yyjson_read_flag flg) { return yyjson_read_max_memory_usage(len, flg); }
const char * yyjson_mut_read_number__extern(const char *dat, yyjson_mut_val *val, yyjson_read_flag flg, const yyjson_alc *alc, yyjson_read_err *err) { return yyjson_mut_read_number(dat, val, flg, alc, err); }
char * yyjson_write__extern(const yyjson_doc *doc, yyjson_write_flag flg, size_t *len) { return yyjson_write(doc, flg, len); }
char * yyjson_mut_write__extern(const yyjson_mut_doc *doc, yyjson_write_flag flg, size_t *len) { return yyjson_mut_write(doc, flg, len); }
char * yyjson_val_write__extern(const yyjson_val *val, yyjson_write_flag flg, size_t *len) { return yyjson_val_write(val, flg, len); }
char * yyjson_mut_val_write__extern(const yyjson_mut_val *val, yyjson_write_flag flg, size_t *len) { return yyjson_mut_val_write(val, flg, len); }
yyjson_val * yyjson_doc_get_root__extern(yyjson_doc *doc) { return yyjson_doc_get_root(doc); }
size_t yyjson_doc_get_read_size__extern(yyjson_doc *doc) { return yyjson_doc_get_read_size(doc); }
size_t yyjson_doc_get_val_count__extern(yyjson_doc *doc) { return yyjson_doc_get_val_count(doc); }
void yyjson_doc_free__extern(yyjson_doc *doc) { yyjson_doc_free(doc); }
bool yyjson_is_raw__extern(yyjson_val *val) { return yyjson_is_raw(val); }
bool yyjson_is_null__extern(yyjson_val *val) { return yyjson_is_null(val); }
bool yyjson_is_true__extern(yyjson_val *val) { return yyjson_is_true(val); }
bool yyjson_is_false__extern(yyjson_val *val) { return yyjson_is_false(val); }
bool yyjson_is_bool__extern(yyjson_val *val) { return yyjson_is_bool(val); }
bool yyjson_is_uint__extern(yyjson_val *val) { return yyjson_is_uint(val); }
bool yyjson_is_sint__extern(yyjson_val *val) { return yyjson_is_sint(val); }
bool yyjson_is_int__extern(yyjson_val *val) { return yyjson_is_int(val); }
bool yyjson_is_real__extern(yyjson_val *val) { return yyjson_is_real(val); }
bool yyjson_is_num__extern(yyjson_val *val) { return yyjson_is_num(val); }
bool yyjson_is_str__extern(yyjson_val *val) { return yyjson_is_str(val); }
bool yyjson_is_arr__extern(yyjson_val *val) { return yyjson_is_arr(val); }
bool yyjson_is_obj__extern(yyjson_val *val) { return yyjson_is_obj(val); }
bool yyjson_is_ctn__extern(yyjson_val *val) { return yyjson_is_ctn(val); }
yyjson_type yyjson_get_type__extern(yyjson_val *val) { return yyjson_get_type(val); }
yyjson_subtype yyjson_get_subtype__extern(yyjson_val *val) { return yyjson_get_subtype(val); }
uint8_t yyjson_get_tag__extern(yyjson_val *val) { return yyjson_get_tag(val); }
const char * yyjson_get_type_desc__extern(yyjson_val *val) { return yyjson_get_type_desc(val); }
const char * yyjson_get_raw__extern(yyjson_val *val) { return yyjson_get_raw(val); }
bool yyjson_get_bool__extern(yyjson_val *val) { return yyjson_get_bool(val); }
uint64_t yyjson_get_uint__extern(yyjson_val *val) { return yyjson_get_uint(val); }
int64_t yyjson_get_sint__extern(yyjson_val *val) { return yyjson_get_sint(val); }
int yyjson_get_int__extern(yyjson_val *val) { return yyjson_get_int(val); }
double yyjson_get_real__extern(yyjson_val *val) { return yyjson_get_real(val); }
double yyjson_get_num__extern(yyjson_val *val) { return yyjson_get_num(val); }
const char * yyjson_get_str__extern(yyjson_val *val) { return yyjson_get_str(val); }
size_t yyjson_get_len__extern(yyjson_val *val) { return yyjson_get_len(val); }
bool yyjson_equals_str__extern(yyjson_val *val, const char *str) { return yyjson_equals_str(val, str); }
bool yyjson_equals_strn__extern(yyjson_val *val, const char *str, size_t len) { return yyjson_equals_strn(val, str, len); }
bool yyjson_equals__extern(yyjson_val *lhs, yyjson_val *rhs) { return yyjson_equals(lhs, rhs); }
bool yyjson_set_raw__extern(yyjson_val *val, const char *raw, size_t len) { return yyjson_set_raw(val, raw, len); }
bool yyjson_set_null__extern(yyjson_val *val) { return yyjson_set_null(val); }
bool yyjson_set_bool__extern(yyjson_val *val, bool num) { return yyjson_set_bool(val, num); }
bool yyjson_set_uint__extern(yyjson_val *val, uint64_t num) { return yyjson_set_uint(val, num); }
bool yyjson_set_sint__extern(yyjson_val *val, int64_t num) { return yyjson_set_sint(val, num); }
bool yyjson_set_int__extern(yyjson_val *val, int num) { return yyjson_set_int(val, num); }
bool yyjson_set_float__extern(yyjson_val *val, float num) { return yyjson_set_float(val, num); }
bool yyjson_set_double__extern(yyjson_val *val, double num) { return yyjson_set_double(val, num); }
bool yyjson_set_real__extern(yyjson_val *val, double num) { return yyjson_set_real(val, num); }
bool yyjson_set_fp_to_fixed__extern(yyjson_val *val, int prec) { return yyjson_set_fp_to_fixed(val, prec); }
bool yyjson_set_fp_to_float__extern(yyjson_val *val, bool flt) { return yyjson_set_fp_to_float(val, flt); }
bool yyjson_set_str__extern(yyjson_val *val, const char *str) { return yyjson_set_str(val, str); }
bool yyjson_set_strn__extern(yyjson_val *val, const char *str, size_t len) { return yyjson_set_strn(val, str, len); }
bool yyjson_set_str_noesc__extern(yyjson_val *val, bool noesc) { return yyjson_set_str_noesc(val, noesc); }
size_t yyjson_arr_size__extern(yyjson_val *arr) { return yyjson_arr_size(arr); }
yyjson_val * yyjson_arr_get__extern(yyjson_val *arr, size_t idx) { return yyjson_arr_get(arr, idx); }
yyjson_val * yyjson_arr_get_first__extern(yyjson_val *arr) { return yyjson_arr_get_first(arr); }
yyjson_val * yyjson_arr_get_last__extern(yyjson_val *arr) { return yyjson_arr_get_last(arr); }
bool yyjson_arr_iter_init__extern(yyjson_val *arr, yyjson_arr_iter *iter) { return yyjson_arr_iter_init(arr, iter); }
yyjson_arr_iter yyjson_arr_iter_with__extern(yyjson_val *arr) { return yyjson_arr_iter_with(arr); }
bool yyjson_arr_iter_has_next__extern(yyjson_arr_iter *iter) { return yyjson_arr_iter_has_next(iter); }
yyjson_val * yyjson_arr_iter_next__extern(yyjson_arr_iter *iter) { return yyjson_arr_iter_next(iter); }
size_t yyjson_obj_size__extern(yyjson_val *obj) { return yyjson_obj_size(obj); }
yyjson_val * yyjson_obj_get__extern(yyjson_val *obj, const char *key) { return yyjson_obj_get(obj, key); }
yyjson_val * yyjson_obj_getn__extern(yyjson_val *obj, const char *key, size_t key_len) { return yyjson_obj_getn(obj, key, key_len); }
bool yyjson_obj_iter_init__extern(yyjson_val *obj, yyjson_obj_iter *iter) { return yyjson_obj_iter_init(obj, iter); }
yyjson_obj_iter yyjson_obj_iter_with__extern(yyjson_val *obj) { return yyjson_obj_iter_with(obj); }
bool yyjson_obj_iter_has_next__extern(yyjson_obj_iter *iter) { return yyjson_obj_iter_has_next(iter); }
yyjson_val * yyjson_obj_iter_next__extern(yyjson_obj_iter *iter) { return yyjson_obj_iter_next(iter); }
yyjson_val * yyjson_obj_iter_get_val__extern(yyjson_val *key) { return yyjson_obj_iter_get_val(key); }
yyjson_val * yyjson_obj_iter_get__extern(yyjson_obj_iter *iter, const char *key) { return yyjson_obj_iter_get(iter, key); }
yyjson_val * yyjson_obj_iter_getn__extern(yyjson_obj_iter *iter, const char *key, size_t key_len) { return yyjson_obj_iter_getn(iter, key, key_len); }
yyjson_mut_val * yyjson_mut_doc_get_root__extern(yyjson_mut_doc *doc) { return yyjson_mut_doc_get_root(doc); }
void yyjson_mut_doc_set_root__extern(yyjson_mut_doc *doc, yyjson_mut_val *root) { yyjson_mut_doc_set_root(doc, root); }
bool yyjson_mut_is_raw__extern(yyjson_mut_val *val) { return yyjson_mut_is_raw(val); }
bool yyjson_mut_is_null__extern(yyjson_mut_val *val) { return yyjson_mut_is_null(val); }
bool yyjson_mut_is_true__extern(yyjson_mut_val *val) { return yyjson_mut_is_true(val); }
bool yyjson_mut_is_false__extern(yyjson_mut_val *val) { return yyjson_mut_is_false(val); }
bool yyjson_mut_is_bool__extern(yyjson_mut_val *val) { return yyjson_mut_is_bool(val); }
bool yyjson_mut_is_uint__extern(yyjson_mut_val *val) { return yyjson_mut_is_uint(val); }
bool yyjson_mut_is_sint__extern(yyjson_mut_val *val) { return yyjson_mut_is_sint(val); }
bool yyjson_mut_is_int__extern(yyjson_mut_val *val) { return yyjson_mut_is_int(val); }
bool yyjson_mut_is_real__extern(yyjson_mut_val *val) { return yyjson_mut_is_real(val); }
bool yyjson_mut_is_num__extern(yyjson_mut_val *val) { return yyjson_mut_is_num(val); }
bool yyjson_mut_is_str__extern(yyjson_mut_val *val) { return yyjson_mut_is_str(val); }
bool yyjson_mut_is_arr__extern(yyjson_mut_val *val) { return yyjson_mut_is_arr(val); }
bool yyjson_mut_is_obj__extern(yyjson_mut_val *val) { return yyjson_mut_is_obj(val); }
bool yyjson_mut_is_ctn__extern(yyjson_mut_val *val) { return yyjson_mut_is_ctn(val); }
yyjson_type yyjson_mut_get_type__extern(yyjson_mut_val *val) { return yyjson_mut_get_type(val); }
yyjson_subtype yyjson_mut_get_subtype__extern(yyjson_mut_val *val) { return yyjson_mut_get_subtype(val); }
uint8_t yyjson_mut_get_tag__extern(yyjson_mut_val *val) { return yyjson_mut_get_tag(val); }
const char * yyjson_mut_get_type_desc__extern(yyjson_mut_val *val) { return yyjson_mut_get_type_desc(val); }
const char * yyjson_mut_get_raw__extern(yyjson_mut_val *val) { return yyjson_mut_get_raw(val); }
bool yyjson_mut_get_bool__extern(yyjson_mut_val *val) { return yyjson_mut_get_bool(val); }
uint64_t yyjson_mut_get_uint__extern(yyjson_mut_val *val) { return yyjson_mut_get_uint(val); }
int64_t yyjson_mut_get_sint__extern(yyjson_mut_val *val) { return yyjson_mut_get_sint(val); }
int yyjson_mut_get_int__extern(yyjson_mut_val *val) { return yyjson_mut_get_int(val); }
double yyjson_mut_get_real__extern(yyjson_mut_val *val) { return yyjson_mut_get_real(val); }
double yyjson_mut_get_num__extern(yyjson_mut_val *val) { return yyjson_mut_get_num(val); }
const char * yyjson_mut_get_str__extern(yyjson_mut_val *val) { return yyjson_mut_get_str(val); }
size_t yyjson_mut_get_len__extern(yyjson_mut_val *val) { return yyjson_mut_get_len(val); }
bool yyjson_mut_equals_str__extern(yyjson_mut_val *val, const char *str) { return yyjson_mut_equals_str(val, str); }
bool yyjson_mut_equals_strn__extern(yyjson_mut_val *val, const char *str, size_t len) { return yyjson_mut_equals_strn(val, str, len); }
bool yyjson_mut_equals__extern(yyjson_mut_val *lhs, yyjson_mut_val *rhs) { return yyjson_mut_equals(lhs, rhs); }
bool yyjson_mut_set_raw__extern(yyjson_mut_val *val, const char *raw, size_t len) { return yyjson_mut_set_raw(val, raw, len); }
bool yyjson_mut_set_null__extern(yyjson_mut_val *val) { return yyjson_mut_set_null(val); }
bool yyjson_mut_set_bool__extern(yyjson_mut_val *val, bool num) { return yyjson_mut_set_bool(val, num); }
bool yyjson_mut_set_uint__extern(yyjson_mut_val *val, uint64_t num) { return yyjson_mut_set_uint(val, num); }
bool yyjson_mut_set_sint__extern(yyjson_mut_val *val, int64_t num) { return yyjson_mut_set_sint(val, num); }
bool yyjson_mut_set_int__extern(yyjson_mut_val *val, int num) { return yyjson_mut_set_int(val, num); }
bool yyjson_mut_set_float__extern(yyjson_mut_val *val, float num) { return yyjson_mut_set_float(val, num); }
bool yyjson_mut_set_double__extern(yyjson_mut_val *val, double num) { return yyjson_mut_set_double(val, num); }
bool yyjson_mut_set_real__extern(yyjson_mut_val *val, double num) { return yyjson_mut_set_real(val, num); }
bool yyjson_mut_set_fp_to_fixed__extern(yyjson_mut_val *val, int prec) { return yyjson_mut_set_fp_to_fixed(val, prec); }
bool yyjson_mut_set_fp_to_float__extern(yyjson_mut_val *val, bool flt) { return yyjson_mut_set_fp_to_float(val, flt); }
bool yyjson_mut_set_str__extern(yyjson_mut_val *val, const char *str) { return yyjson_mut_set_str(val, str); }
bool yyjson_mut_set_strn__extern(yyjson_mut_val *val, const char *str, size_t len) { return yyjson_mut_set_strn(val, str, len); }
bool yyjson_mut_set_str_noesc__extern(yyjson_mut_val *val, bool noesc) { return yyjson_mut_set_str_noesc(val, noesc); }
bool yyjson_mut_set_arr__extern(yyjson_mut_val *val) { return yyjson_mut_set_arr(val); }
bool yyjson_mut_set_obj__extern(yyjson_mut_val *val) { return yyjson_mut_set_obj(val); }
yyjson_mut_val * yyjson_mut_raw__extern(yyjson_mut_doc *doc, const char *str) { return yyjson_mut_raw(doc, str); }
yyjson_mut_val * yyjson_mut_rawn__extern(yyjson_mut_doc *doc, const char *str, size_t len) { return yyjson_mut_rawn(doc, str, len); }
yyjson_mut_val * yyjson_mut_rawcpy__extern(yyjson_mut_doc *doc, const char *str) { return yyjson_mut_rawcpy(doc, str); }
yyjson_mut_val * yyjson_mut_rawncpy__extern(yyjson_mut_doc *doc, const char *str, size_t len) { return yyjson_mut_rawncpy(doc, str, len); }
yyjson_mut_val * yyjson_mut_null__extern(yyjson_mut_doc *doc) { return yyjson_mut_null(doc); }
yyjson_mut_val * yyjson_mut_true__extern(yyjson_mut_doc *doc) { return yyjson_mut_true(doc); }
yyjson_mut_val * yyjson_mut_false__extern(yyjson_mut_doc *doc) { return yyjson_mut_false(doc); }
yyjson_mut_val * yyjson_mut_bool__extern(yyjson_mut_doc *doc, bool val) { return yyjson_mut_bool(doc, val); }
yyjson_mut_val * yyjson_mut_uint__extern(yyjson_mut_doc *doc, uint64_t num) { return yyjson_mut_uint(doc, num); }
yyjson_mut_val * yyjson_mut_sint__extern(yyjson_mut_doc *doc, int64_t num) { return yyjson_mut_sint(doc, num); }
yyjson_mut_val * yyjson_mut_int__extern(yyjson_mut_doc *doc, int64_t num) { return yyjson_mut_int(doc, num); }
yyjson_mut_val * yyjson_mut_float__extern(yyjson_mut_doc *doc, float num) { return yyjson_mut_float(doc, num); }
yyjson_mut_val * yyjson_mut_double__extern(yyjson_mut_doc *doc, double num) { return yyjson_mut_double(doc, num); }
yyjson_mut_val * yyjson_mut_real__extern(yyjson_mut_doc *doc, double num) { return yyjson_mut_real(doc, num); }
yyjson_mut_val * yyjson_mut_str__extern(yyjson_mut_doc *doc, const char *str) { return yyjson_mut_str(doc, str); }
yyjson_mut_val * yyjson_mut_strn__extern(yyjson_mut_doc *doc, const char *str, size_t len) { return yyjson_mut_strn(doc, str, len); }
yyjson_mut_val * yyjson_mut_strcpy__extern(yyjson_mut_doc *doc, const char *str) { return yyjson_mut_strcpy(doc, str); }
yyjson_mut_val * yyjson_mut_strncpy__extern(yyjson_mut_doc *doc, const char *str, size_t len) { return yyjson_mut_strncpy(doc, str, len); }
size_t yyjson_mut_arr_size__extern(yyjson_mut_val *arr) { return yyjson_mut_arr_size(arr); }
yyjson_mut_val * yyjson_mut_arr_get__extern(yyjson_mut_val *arr, size_t idx) { return yyjson_mut_arr_get(arr, idx); }
yyjson_mut_val * yyjson_mut_arr_get_first__extern(yyjson_mut_val *arr) { return yyjson_mut_arr_get_first(arr); }
yyjson_mut_val * yyjson_mut_arr_get_last__extern(yyjson_mut_val *arr) { return yyjson_mut_arr_get_last(arr); }
bool yyjson_mut_arr_iter_init__extern(yyjson_mut_val *arr, yyjson_mut_arr_iter *iter) { return yyjson_mut_arr_iter_init(arr, iter); }
yyjson_mut_arr_iter yyjson_mut_arr_iter_with__extern(yyjson_mut_val *arr) { return yyjson_mut_arr_iter_with(arr); }
bool yyjson_mut_arr_iter_has_next__extern(yyjson_mut_arr_iter *iter) { return yyjson_mut_arr_iter_has_next(iter); }
yyjson_mut_val * yyjson_mut_arr_iter_next__extern(yyjson_mut_arr_iter *iter) { return yyjson_mut_arr_iter_next(iter); }
yyjson_mut_val * yyjson_mut_arr_iter_remove__extern(yyjson_mut_arr_iter *iter) { return yyjson_mut_arr_iter_remove(iter); }
yyjson_mut_val * yyjson_mut_arr__extern(yyjson_mut_doc *doc) { return yyjson_mut_arr(doc); }
yyjson_mut_val * yyjson_mut_arr_with_bool__extern(yyjson_mut_doc *doc, const bool *vals, size_t count) { return yyjson_mut_arr_with_bool(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_sint__extern(yyjson_mut_doc *doc, const int64_t *vals, size_t count) { return yyjson_mut_arr_with_sint(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_uint__extern(yyjson_mut_doc *doc, const uint64_t *vals, size_t count) { return yyjson_mut_arr_with_uint(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_real__extern(yyjson_mut_doc *doc, const double *vals, size_t count) { return yyjson_mut_arr_with_real(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_sint8__extern(yyjson_mut_doc *doc, const int8_t *vals, size_t count) { return yyjson_mut_arr_with_sint8(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_sint16__extern(yyjson_mut_doc *doc, const int16_t *vals, size_t count) { return yyjson_mut_arr_with_sint16(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_sint32__extern(yyjson_mut_doc *doc, const int32_t *vals, size_t count) { return yyjson_mut_arr_with_sint32(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_sint64__extern(yyjson_mut_doc *doc, const int64_t *vals, size_t count) { return yyjson_mut_arr_with_sint64(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_uint8__extern(yyjson_mut_doc *doc, const uint8_t *vals, size_t count) { return yyjson_mut_arr_with_uint8(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_uint16__extern(yyjson_mut_doc *doc, const uint16_t *vals, size_t count) { return yyjson_mut_arr_with_uint16(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_uint32__extern(yyjson_mut_doc *doc, const uint32_t *vals, size_t count) { return yyjson_mut_arr_with_uint32(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_uint64__extern(yyjson_mut_doc *doc, const uint64_t *vals, size_t count) { return yyjson_mut_arr_with_uint64(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_float__extern(yyjson_mut_doc *doc, const float *vals, size_t count) { return yyjson_mut_arr_with_float(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_double__extern(yyjson_mut_doc *doc, const double *vals, size_t count) { return yyjson_mut_arr_with_double(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_str__extern(yyjson_mut_doc *doc, const char **vals, size_t count) { return yyjson_mut_arr_with_str(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_strn__extern(yyjson_mut_doc *doc, const char **vals, const size_t *lens, size_t count) { return yyjson_mut_arr_with_strn(doc, vals, lens, count); }
yyjson_mut_val * yyjson_mut_arr_with_strcpy__extern(yyjson_mut_doc *doc, const char **vals, size_t count) { return yyjson_mut_arr_with_strcpy(doc, vals, count); }
yyjson_mut_val * yyjson_mut_arr_with_strncpy__extern(yyjson_mut_doc *doc, const char **vals, const size_t *lens, size_t count) { return yyjson_mut_arr_with_strncpy(doc, vals, lens, count); }
bool yyjson_mut_arr_insert__extern(yyjson_mut_val *arr, yyjson_mut_val *val, size_t idx) { return yyjson_mut_arr_insert(arr, val, idx); }
bool yyjson_mut_arr_append__extern(yyjson_mut_val *arr, yyjson_mut_val *val) { return yyjson_mut_arr_append(arr, val); }
bool yyjson_mut_arr_prepend__extern(yyjson_mut_val *arr, yyjson_mut_val *val) { return yyjson_mut_arr_prepend(arr, val); }
yyjson_mut_val * yyjson_mut_arr_replace__extern(yyjson_mut_val *arr, size_t idx, yyjson_mut_val *val) { return yyjson_mut_arr_replace(arr, idx, val); }
yyjson_mut_val * yyjson_mut_arr_remove__extern(yyjson_mut_val *arr, size_t idx) { return yyjson_mut_arr_remove(arr, idx); }
yyjson_mut_val * yyjson_mut_arr_remove_first__extern(yyjson_mut_val *arr) { return yyjson_mut_arr_remove_first(arr); }
yyjson_mut_val * yyjson_mut_arr_remove_last__extern(yyjson_mut_val *arr) { return yyjson_mut_arr_remove_last(arr); }
bool yyjson_mut_arr_remove_range__extern(yyjson_mut_val *arr, size_t idx, size_t len) { return yyjson_mut_arr_remove_range(arr, idx, len); }
bool yyjson_mut_arr_clear__extern(yyjson_mut_val *arr) { return yyjson_mut_arr_clear(arr); }
bool yyjson_mut_arr_rotate__extern(yyjson_mut_val *arr, size_t idx) { return yyjson_mut_arr_rotate(arr, idx); }
bool yyjson_mut_arr_add_val__extern(yyjson_mut_val *arr, yyjson_mut_val *val) { return yyjson_mut_arr_add_val(arr, val); }
bool yyjson_mut_arr_add_null__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr) { return yyjson_mut_arr_add_null(doc, arr); }
bool yyjson_mut_arr_add_true__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr) { return yyjson_mut_arr_add_true(doc, arr); }
bool yyjson_mut_arr_add_false__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr) { return yyjson_mut_arr_add_false(doc, arr); }
bool yyjson_mut_arr_add_bool__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, bool val) { return yyjson_mut_arr_add_bool(doc, arr, val); }
bool yyjson_mut_arr_add_uint__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, uint64_t num) { return yyjson_mut_arr_add_uint(doc, arr, num); }
bool yyjson_mut_arr_add_sint__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, int64_t num) { return yyjson_mut_arr_add_sint(doc, arr, num); }
bool yyjson_mut_arr_add_int__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, int64_t num) { return yyjson_mut_arr_add_int(doc, arr, num); }
bool yyjson_mut_arr_add_float__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, float num) { return yyjson_mut_arr_add_float(doc, arr, num); }
bool yyjson_mut_arr_add_double__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, double num) { return yyjson_mut_arr_add_double(doc, arr, num); }
bool yyjson_mut_arr_add_real__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, double num) { return yyjson_mut_arr_add_real(doc, arr, num); }
bool yyjson_mut_arr_add_str__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, const char *str) { return yyjson_mut_arr_add_str(doc, arr, str); }
bool yyjson_mut_arr_add_strn__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, const char *str, size_t len) { return yyjson_mut_arr_add_strn(doc, arr, str, len); }
bool yyjson_mut_arr_add_strcpy__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, const char *str) { return yyjson_mut_arr_add_strcpy(doc, arr, str); }
bool yyjson_mut_arr_add_strncpy__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr, const char *str, size_t len) { return yyjson_mut_arr_add_strncpy(doc, arr, str, len); }
yyjson_mut_val * yyjson_mut_arr_add_arr__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr) { return yyjson_mut_arr_add_arr(doc, arr); }
yyjson_mut_val * yyjson_mut_arr_add_obj__extern(yyjson_mut_doc *doc, yyjson_mut_val *arr) { return yyjson_mut_arr_add_obj(doc, arr); }
size_t yyjson_mut_obj_size__extern(yyjson_mut_val *obj) { return yyjson_mut_obj_size(obj); }
yyjson_mut_val * yyjson_mut_obj_get__extern(yyjson_mut_val *obj, const char *key) { return yyjson_mut_obj_get(obj, key); }
yyjson_mut_val * yyjson_mut_obj_getn__extern(yyjson_mut_val *obj, const char *key, size_t key_len) { return yyjson_mut_obj_getn(obj, key, key_len); }
bool yyjson_mut_obj_iter_init__extern(yyjson_mut_val *obj, yyjson_mut_obj_iter *iter) { return yyjson_mut_obj_iter_init(obj, iter); }
yyjson_mut_obj_iter yyjson_mut_obj_iter_with__extern(yyjson_mut_val *obj) { return yyjson_mut_obj_iter_with(obj); }
bool yyjson_mut_obj_iter_has_next__extern(yyjson_mut_obj_iter *iter) { return yyjson_mut_obj_iter_has_next(iter); }
yyjson_mut_val * yyjson_mut_obj_iter_next__extern(yyjson_mut_obj_iter *iter) { return yyjson_mut_obj_iter_next(iter); }
yyjson_mut_val * yyjson_mut_obj_iter_get_val__extern(yyjson_mut_val *key) { return yyjson_mut_obj_iter_get_val(key); }
yyjson_mut_val * yyjson_mut_obj_iter_remove__extern(yyjson_mut_obj_iter *iter) { return yyjson_mut_obj_iter_remove(iter); }
yyjson_mut_val * yyjson_mut_obj_iter_get__extern(yyjson_mut_obj_iter *iter, const char *key) { return yyjson_mut_obj_iter_get(iter, key); }
yyjson_mut_val * yyjson_mut_obj_iter_getn__extern(yyjson_mut_obj_iter *iter, const char *key, size_t key_len) { return yyjson_mut_obj_iter_getn(iter, key, key_len); }
yyjson_mut_val * yyjson_mut_obj__extern(yyjson_mut_doc *doc) { return yyjson_mut_obj(doc); }
yyjson_mut_val * yyjson_mut_obj_with_str__extern(yyjson_mut_doc *doc, const char **keys, const char **vals, size_t count) { return yyjson_mut_obj_with_str(doc, keys, vals, count); }
yyjson_mut_val * yyjson_mut_obj_with_kv__extern(yyjson_mut_doc *doc, const char **kv_pairs, size_t pair_count) { return yyjson_mut_obj_with_kv(doc, kv_pairs, pair_count); }
bool yyjson_mut_obj_add__extern(yyjson_mut_val *obj, yyjson_mut_val *key, yyjson_mut_val *val) { return yyjson_mut_obj_add(obj, key, val); }
bool yyjson_mut_obj_put__extern(yyjson_mut_val *obj, yyjson_mut_val *key, yyjson_mut_val *val) { return yyjson_mut_obj_put(obj, key, val); }
bool yyjson_mut_obj_insert__extern(yyjson_mut_val *obj, yyjson_mut_val *key, yyjson_mut_val *val, size_t idx) { return yyjson_mut_obj_insert(obj, key, val, idx); }
yyjson_mut_val * yyjson_mut_obj_remove__extern(yyjson_mut_val *obj, yyjson_mut_val *key) { return yyjson_mut_obj_remove(obj, key); }
yyjson_mut_val * yyjson_mut_obj_remove_key__extern(yyjson_mut_val *obj, const char *key) { return yyjson_mut_obj_remove_key(obj, key); }
yyjson_mut_val * yyjson_mut_obj_remove_keyn__extern(yyjson_mut_val *obj, const char *key, size_t key_len) { return yyjson_mut_obj_remove_keyn(obj, key, key_len); }
bool yyjson_mut_obj_clear__extern(yyjson_mut_val *obj) { return yyjson_mut_obj_clear(obj); }
bool yyjson_mut_obj_replace__extern(yyjson_mut_val *obj, yyjson_mut_val *key, yyjson_mut_val *val) { return yyjson_mut_obj_replace(obj, key, val); }
bool yyjson_mut_obj_rotate__extern(yyjson_mut_val *obj, size_t idx) { return yyjson_mut_obj_rotate(obj, idx); }
bool yyjson_mut_obj_add_null__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key) { return yyjson_mut_obj_add_null(doc, obj, key); }
bool yyjson_mut_obj_add_true__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key) { return yyjson_mut_obj_add_true(doc, obj, key); }
bool yyjson_mut_obj_add_false__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key) { return yyjson_mut_obj_add_false(doc, obj, key); }
bool yyjson_mut_obj_add_bool__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, bool val) { return yyjson_mut_obj_add_bool(doc, obj, key, val); }
bool yyjson_mut_obj_add_uint__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, uint64_t val) { return yyjson_mut_obj_add_uint(doc, obj, key, val); }
bool yyjson_mut_obj_add_sint__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, int64_t val) { return yyjson_mut_obj_add_sint(doc, obj, key, val); }
bool yyjson_mut_obj_add_int__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, int64_t val) { return yyjson_mut_obj_add_int(doc, obj, key, val); }
bool yyjson_mut_obj_add_float__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, float val) { return yyjson_mut_obj_add_float(doc, obj, key, val); }
bool yyjson_mut_obj_add_double__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, double val) { return yyjson_mut_obj_add_double(doc, obj, key, val); }
bool yyjson_mut_obj_add_real__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, double val) { return yyjson_mut_obj_add_real(doc, obj, key, val); }
bool yyjson_mut_obj_add_str__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, const char *val) { return yyjson_mut_obj_add_str(doc, obj, key, val); }
bool yyjson_mut_obj_add_strn__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, const char *val, size_t len) { return yyjson_mut_obj_add_strn(doc, obj, key, val, len); }
bool yyjson_mut_obj_add_strcpy__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, const char *val) { return yyjson_mut_obj_add_strcpy(doc, obj, key, val); }
bool yyjson_mut_obj_add_strncpy__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, const char *val, size_t len) { return yyjson_mut_obj_add_strncpy(doc, obj, key, val, len); }
yyjson_mut_val * yyjson_mut_obj_add_arr__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key) { return yyjson_mut_obj_add_arr(doc, obj, key); }
yyjson_mut_val * yyjson_mut_obj_add_obj__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key) { return yyjson_mut_obj_add_obj(doc, obj, key); }
bool yyjson_mut_obj_add_val__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, yyjson_mut_val *val) { return yyjson_mut_obj_add_val(doc, obj, key, val); }
yyjson_mut_val * yyjson_mut_obj_remove_str__extern(yyjson_mut_val *obj, const char *key) { return yyjson_mut_obj_remove_str(obj, key); }
yyjson_mut_val * yyjson_mut_obj_remove_strn__extern(yyjson_mut_val *obj, const char *key, size_t len) { return yyjson_mut_obj_remove_strn(obj, key, len); }
bool yyjson_mut_obj_rename_key__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, const char *new_key) { return yyjson_mut_obj_rename_key(doc, obj, key, new_key); }
bool yyjson_mut_obj_rename_keyn__extern(yyjson_mut_doc *doc, yyjson_mut_val *obj, const char *key, size_t len, const char *new_key, size_t new_len) { return yyjson_mut_obj_rename_keyn(doc, obj, key, len, new_key, new_len); }
bool unsafe_yyjson_is_str_noesc__extern(const char *str, size_t len) { return unsafe_yyjson_is_str_noesc(str, len); }
double unsafe_yyjson_u64_to_f64__extern(uint64_t num) { return unsafe_yyjson_u64_to_f64(num); }
yyjson_type unsafe_yyjson_get_type__extern(void *val) { return unsafe_yyjson_get_type(val); }
yyjson_subtype unsafe_yyjson_get_subtype__extern(void *val) { return unsafe_yyjson_get_subtype(val); }
uint8_t unsafe_yyjson_get_tag__extern(void *val) { return unsafe_yyjson_get_tag(val); }
bool unsafe_yyjson_is_raw__extern(void *val) { return unsafe_yyjson_is_raw(val); }
bool unsafe_yyjson_is_null__extern(void *val) { return unsafe_yyjson_is_null(val); }
bool unsafe_yyjson_is_bool__extern(void *val) { return unsafe_yyjson_is_bool(val); }
bool unsafe_yyjson_is_num__extern(void *val) { return unsafe_yyjson_is_num(val); }
bool unsafe_yyjson_is_str__extern(void *val) { return unsafe_yyjson_is_str(val); }
bool unsafe_yyjson_is_arr__extern(void *val) { return unsafe_yyjson_is_arr(val); }
bool unsafe_yyjson_is_obj__extern(void *val) { return unsafe_yyjson_is_obj(val); }
bool unsafe_yyjson_is_ctn__extern(void *val) { return unsafe_yyjson_is_ctn(val); }
bool unsafe_yyjson_is_uint__extern(void *val) { return unsafe_yyjson_is_uint(val); }
bool unsafe_yyjson_is_sint__extern(void *val) { return unsafe_yyjson_is_sint(val); }
bool unsafe_yyjson_is_int__extern(void *val) { return unsafe_yyjson_is_int(val); }
bool unsafe_yyjson_is_real__extern(void *val) { return unsafe_yyjson_is_real(val); }
bool unsafe_yyjson_is_true__extern(void *val) { return unsafe_yyjson_is_true(val); }
bool unsafe_yyjson_is_false__extern(void *val) { return unsafe_yyjson_is_false(val); }
bool unsafe_yyjson_arr_is_flat__extern(yyjson_val *val) { return unsafe_yyjson_arr_is_flat(val); }
const char * unsafe_yyjson_get_raw__extern(void *val) { return unsafe_yyjson_get_raw(val); }
bool unsafe_yyjson_get_bool__extern(void *val) { return unsafe_yyjson_get_bool(val); }
uint64_t unsafe_yyjson_get_uint__extern(void *val) { return unsafe_yyjson_get_uint(val); }
int64_t unsafe_yyjson_get_sint__extern(void *val) { return unsafe_yyjson_get_sint(val); }
int unsafe_yyjson_get_int__extern(void *val) { return unsafe_yyjson_get_int(val); }
double unsafe_yyjson_get_real__extern(void *val) { return unsafe_yyjson_get_real(val); }
double unsafe_yyjson_get_num__extern(void *val) { return unsafe_yyjson_get_num(val); }
const char * unsafe_yyjson_get_str__extern(void *val) { return unsafe_yyjson_get_str(val); }
size_t unsafe_yyjson_get_len__extern(void *val) { return unsafe_yyjson_get_len(val); }
yyjson_val * unsafe_yyjson_get_first__extern(yyjson_val *ctn) { return unsafe_yyjson_get_first(ctn); }
yyjson_val * unsafe_yyjson_get_next__extern(yyjson_val *val) { return unsafe_yyjson_get_next(val); }
bool unsafe_yyjson_equals_strn__extern(void *val, const char *str, size_t len) { return unsafe_yyjson_equals_strn(val, str, len); }
bool unsafe_yyjson_equals_str__extern(void *val, const char *str) { return unsafe_yyjson_equals_str(val, str); }
void unsafe_yyjson_set_type__extern(void *val, yyjson_type type, yyjson_subtype subtype) { unsafe_yyjson_set_type(val, type, subtype); }
void unsafe_yyjson_set_len__extern(void *val, size_t len) { unsafe_yyjson_set_len(val, len); }
void unsafe_yyjson_set_tag__extern(void *val, yyjson_type type, yyjson_subtype subtype, size_t len) { unsafe_yyjson_set_tag(val, type, subtype, len); }
void unsafe_yyjson_inc_len__extern(void *val) { unsafe_yyjson_inc_len(val); }
void unsafe_yyjson_set_raw__extern(void *val, const char *raw, size_t len) { unsafe_yyjson_set_raw(val, raw, len); }
void unsafe_yyjson_set_null__extern(void *val) { unsafe_yyjson_set_null(val); }
void unsafe_yyjson_set_bool__extern(void *val, bool num) { unsafe_yyjson_set_bool(val, num); }
void unsafe_yyjson_set_uint__extern(void *val, uint64_t num) { unsafe_yyjson_set_uint(val, num); }
void unsafe_yyjson_set_sint__extern(void *val, int64_t num) { unsafe_yyjson_set_sint(val, num); }
void unsafe_yyjson_set_fp_to_fixed__extern(void *val, int prec) { unsafe_yyjson_set_fp_to_fixed(val, prec); }
void unsafe_yyjson_set_fp_to_float__extern(void *val, bool flt) { unsafe_yyjson_set_fp_to_float(val, flt); }
void unsafe_yyjson_set_float__extern(void *val, float num) { unsafe_yyjson_set_float(val, num); }
void unsafe_yyjson_set_double__extern(void *val, double num) { unsafe_yyjson_set_double(val, num); }
void unsafe_yyjson_set_real__extern(void *val, double num) { unsafe_yyjson_set_real(val, num); }
void unsafe_yyjson_set_str_noesc__extern(void *val, bool noesc) { unsafe_yyjson_set_str_noesc(val, noesc); }
void unsafe_yyjson_set_strn__extern(void *val, const char *str, size_t len) { unsafe_yyjson_set_strn(val, str, len); }
void unsafe_yyjson_set_str__extern(void *val, const char *str) { unsafe_yyjson_set_str(val, str); }
void unsafe_yyjson_set_arr__extern(void *val, size_t size) { unsafe_yyjson_set_arr(val, size); }
void unsafe_yyjson_set_obj__extern(void *val, size_t size) { unsafe_yyjson_set_obj(val, size); }
char * unsafe_yyjson_mut_str_alc__extern(yyjson_mut_doc *doc, size_t len) { return unsafe_yyjson_mut_str_alc(doc, len); }
char * unsafe_yyjson_mut_strncpy__extern(yyjson_mut_doc *doc, const char *str, size_t len) { return unsafe_yyjson_mut_strncpy(doc, str, len); }
yyjson_mut_val * unsafe_yyjson_mut_val__extern(yyjson_mut_doc *doc, size_t count) { return unsafe_yyjson_mut_val(doc, count); }
void unsafe_yyjson_mut_obj_add__extern(yyjson_mut_val *obj, yyjson_mut_val *key, yyjson_mut_val *val, size_t len) { unsafe_yyjson_mut_obj_add(obj, key, val, len); }
yyjson_mut_val * unsafe_yyjson_mut_obj_remove__extern(yyjson_mut_val *obj, const char *key, size_t key_len) { return unsafe_yyjson_mut_obj_remove(obj, key, key_len); }
bool unsafe_yyjson_mut_obj_replace__extern(yyjson_mut_val *obj, yyjson_mut_val *key, yyjson_mut_val *val) { return unsafe_yyjson_mut_obj_replace(obj, key, val); }
void unsafe_yyjson_mut_obj_rotate__extern(yyjson_mut_val *obj, size_t idx) { unsafe_yyjson_mut_obj_rotate(obj, idx); }
