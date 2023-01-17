mod runner;

use runner::tzt::TZT;
use vm::Result;

#[test]
fn tzt_sub_timestamp_int_00() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_00.json")?.run()
}

#[test]
fn tzt_map_listint_05() -> Result<()> {
    TZT::load("tzt_map_listint_05.json")?.run()
}

#[test]
fn tzt_compare_bool_01() -> Result<()> {
    TZT::load("tzt_compare_bool_01.json")?.run()
}

#[test]
fn tzt_not_nat_05() -> Result<()> {
    TZT::load("tzt_not_nat_05.json")?.run()
}

#[test]
fn tzt_car_01() -> Result<()> {
    TZT::load("tzt_car_01.json")?.run()
}

#[test]
fn tzt_slice_bytes_00() -> Result<()> {
    TZT::load("tzt_slice_bytes_00.json")?.run()
}

#[test]
fn tzt_int_nat_00() -> Result<()> {
    TZT::load("tzt_int_nat_00.json")?.run()
}

#[test]
fn tzt_exec_00() -> Result<()> {
    TZT::load("tzt_exec_00.json")?.run()
}

#[test]
fn tzt_mem_bigmapstringnat_04() -> Result<()> {
    TZT::load("tzt_mem_bigmapstringnat_04.json")?.run()
}

#[test]
fn tzt_get_mapintint_00() -> Result<()> {
    TZT::load("tzt_get_mapintint_00.json")?.run()
}

#[test]
fn tzt_iter_listint_02() -> Result<()> {
    TZT::load("tzt_iter_listint_02.json")?.run()
}

#[test]
fn tzt_ifcons_listint_01() -> Result<()> {
    TZT::load("tzt_ifcons_listint_01.json")?.run()
}

#[test]
fn tzt_compare_string_03() -> Result<()> {
    TZT::load("tzt_compare_string_03.json")?.run()
}

#[test]
fn tzt_mem_mapnatnat_05() -> Result<()> {
    TZT::load("tzt_mem_mapnatnat_05.json")?.run()
}

#[test]
fn tzt_size_listint_00() -> Result<()> {
    TZT::load("tzt_size_listint_00.json")?.run()
}

#[test]
fn tzt_compare_timestamp_01() -> Result<()> {
    TZT::load("tzt_compare_timestamp_01.json")?.run()
}

#[test]
fn tzt_neg_int_02() -> Result<()> {
    TZT::load("tzt_neg_int_02.json")?.run()
}

#[test]
fn tzt_ifcons_listnat_00() -> Result<()> {
    TZT::load("tzt_ifcons_listnat_00.json")?.run()
}

#[test]
fn tzt_xor_nat_nat_04() -> Result<()> {
    TZT::load("tzt_xor_nat_nat_04.json")?.run()
}

#[test]
fn tzt_none_int_00() -> Result<()> {
    TZT::load("tzt_none_int_00.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_06() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_06.json")?.run()
}

#[test]
fn tzt_get_bigmapstringstring_01() -> Result<()> {
    TZT::load("tzt_get_bigmapstringstring_01.json")?.run()
}

#[test]
fn tzt_compare_nat_03() -> Result<()> {
    TZT::load("tzt_compare_nat_03.json")?.run()
}

#[test]
fn tzt_size_setint_01() -> Result<()> {
    TZT::load("tzt_size_setint_01.json")?.run()
}

#[test]
fn tzt_ifleft_orintstring_00() -> Result<()> {
    TZT::load("tzt_ifleft_orintstring_00.json")?.run()
}

#[test]
fn tzt_lsl_04() -> Result<()> {
    TZT::load("tzt_lsl_04.json")?.run()
}

#[test]
fn tzt_mem_mapintint_00() -> Result<()> {
    TZT::load("tzt_mem_mapintint_00.json")?.run()
}

#[test]
fn tzt_add_int_nat_00() -> Result<()> {
    TZT::load("tzt_add_int_nat_00.json")?.run()
}

#[test]
fn tzt_ifnone_optionint_00() -> Result<()> {
    TZT::load("tzt_ifnone_optionint_00.json")?.run()
}

#[test]
fn tzt_add_timestamp_int_02() -> Result<()> {
    TZT::load("tzt_add_timestamp_int_02.json")?.run()
}

#[test]
fn tzt_cons_int_01() -> Result<()> {
    TZT::load("tzt_cons_int_01.json")?.run()
}

#[test]
fn tzt_compare_pairintint_00() -> Result<()> {
    TZT::load("tzt_compare_pairintint_00.json")?.run()
}

#[test]
fn tzt_ediv_int_int_00() -> Result<()> {
    TZT::load("tzt_ediv_int_int_00.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_01() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_01.json")?.run()
}

#[test]
fn tzt_cons_int_00() -> Result<()> {
    TZT::load("tzt_cons_int_00.json")?.run()
}

#[test]
fn tzt_concat_liststring_00() -> Result<()> {
    TZT::load("tzt_concat_liststring_00.json")?.run()
}

#[test]
fn tzt_exec_02() -> Result<()> {
    TZT::load("tzt_exec_02.json")?.run()
}

#[test]
fn tzt_loopleft_02() -> Result<()> {
    TZT::load("tzt_loopleft_02.json")?.run()
}

#[test]
fn tzt_le_01() -> Result<()> {
    TZT::load("tzt_le_01.json")?.run()
}

#[test]
fn tzt_xor_nat_nat_05() -> Result<()> {
    TZT::load("tzt_xor_nat_nat_05.json")?.run()
}

#[test]
fn tzt_contract_05() -> Result<()> {
    TZT::load("tzt_contract_05.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_04() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_04.json")?.run()
}

#[test]
fn tzt_mem_bigmapstringnat_01() -> Result<()> {
    TZT::load("tzt_mem_bigmapstringnat_01.json")?.run()
}

#[test]
fn tzt_not_nat_01() -> Result<()> {
    TZT::load("tzt_not_nat_01.json")?.run()
}

#[test]
fn tzt_emptybigmap_nat_nat_00() -> Result<()> {
    TZT::load("tzt_emptybigmap_nat_nat_00.json")?.run()
}

#[test]
fn tzt_left_int_nat_00() -> Result<()> {
    TZT::load("tzt_left_int_nat_00.json")?.run()
}

#[test]
fn tzt_map_mapintstring_00() -> Result<()> {
    TZT::load("tzt_map_mapintstring_00.json")?.run()
}

#[test]
fn tzt_add_nat_int_00() -> Result<()> {
    TZT::load("tzt_add_nat_int_00.json")?.run()
}

#[test]
fn tzt_concat_string_02() -> Result<()> {
    TZT::load("tzt_concat_string_02.json")?.run()
}

#[test]
fn tzt_not_nat_02() -> Result<()> {
    TZT::load("tzt_not_nat_02.json")?.run()
}

#[test]
fn tzt_nil_nat_00() -> Result<()> {
    TZT::load("tzt_nil_nat_00.json")?.run()
}

#[test]
fn tzt_update_mapintint_00() -> Result<()> {
    TZT::load("tzt_update_mapintint_00.json")?.run()
}

#[test]
fn tzt_map_listint_01() -> Result<()> {
    TZT::load("tzt_map_listint_01.json")?.run()
}

#[test]
fn tzt_compare_pairintint_02() -> Result<()> {
    TZT::load("tzt_compare_pairintint_02.json")?.run()
}

#[test]
fn tzt_some_string_00() -> Result<()> {
    TZT::load("tzt_some_string_00.json")?.run()
}

#[test]
fn tzt_or_bool_bool_02() -> Result<()> {
    TZT::load("tzt_or_bool_bool_02.json")?.run()
}

#[test]
fn tzt_concat_liststring_02() -> Result<()> {
    TZT::load("tzt_concat_liststring_02.json")?.run()
}

#[test]
fn tzt_compare_bytes_04() -> Result<()> {
    TZT::load("tzt_compare_bytes_04.json")?.run()
}

#[test]
fn tzt_compare_nat_01() -> Result<()> {
    TZT::load("tzt_compare_nat_01.json")?.run()
}

#[test]
fn tzt_packunpack_nat_00() -> Result<()> {
    TZT::load("tzt_packunpack_nat_00.json")?.run()
}

#[test]
fn tzt_balance_00() -> Result<()> {
    TZT::load("tzt_balance_00.json")?.run()
}

#[test]
fn tzt_mul_nat_int_00() -> Result<()> {
    TZT::load("tzt_mul_nat_int_00.json")?.run()
}

#[test]
fn tzt_contract_02() -> Result<()> {
    TZT::load("tzt_contract_02.json")?.run()
}

#[test]
fn tzt_compare_int_03() -> Result<()> {
    TZT::load("tzt_compare_int_03.json")?.run()
}

#[test]
fn tzt_gt_03() -> Result<()> {
    TZT::load("tzt_gt_03.json")?.run()
}

#[test]
fn tzt_and_nat_nat_01() -> Result<()> {
    TZT::load("tzt_and_nat_nat_01.json")?.run()
}

#[test]
fn tzt_map_mapstringnat_01() -> Result<()> {
    TZT::load("tzt_map_mapstringnat_01.json")?.run()
}

#[test]
fn tzt_lsr_00() -> Result<()> {
    TZT::load("tzt_lsr_00.json")?.run()
}

#[test]
fn tzt_xor_nat_nat_00() -> Result<()> {
    TZT::load("tzt_xor_nat_nat_00.json")?.run()
}

#[test]
fn tzt_mem_mapstringnat_03() -> Result<()> {
    TZT::load("tzt_mem_mapstringnat_03.json")?.run()
}

#[test]
fn tzt_map_mapstringnat_00() -> Result<()> {
    TZT::load("tzt_map_mapstringnat_00.json")?.run()
}

#[test]
fn tzt_lsr_05() -> Result<()> {
    TZT::load("tzt_lsr_05.json")?.run()
}

#[test]
fn tzt_or_bool_bool_00() -> Result<()> {
    TZT::load("tzt_or_bool_bool_00.json")?.run()
}

#[test]
fn tzt_size_setint_03() -> Result<()> {
    TZT::load("tzt_size_setint_03.json")?.run()
}

#[test]
fn tzt_or_bool_bool_03() -> Result<()> {
    TZT::load("tzt_or_bool_bool_03.json")?.run()
}

#[test]
fn tzt_push_string_00() -> Result<()> {
    TZT::load("tzt_push_string_00.json")?.run()
}

#[test]
fn tzt_map_liststring_07() -> Result<()> {
    TZT::load("tzt_map_liststring_07.json")?.run()
}

#[test]
fn tzt_concat_liststring_01() -> Result<()> {
    TZT::load("tzt_concat_liststring_01.json")?.run()
}

#[test]
fn tzt_add_timestamp_int_00() -> Result<()> {
    TZT::load("tzt_add_timestamp_int_00.json")?.run()
}

#[test]
fn tzt_mem_bigmapstringnat_03() -> Result<()> {
    TZT::load("tzt_mem_bigmapstringnat_03.json")?.run()
}

#[test]
fn tzt_mem_mapnatnat_03() -> Result<()> {
    TZT::load("tzt_mem_mapnatnat_03.json")?.run()
}

#[test]
fn tzt_iter_setint_01() -> Result<()> {
    TZT::load("tzt_iter_setint_01.json")?.run()
}

#[test]
fn tzt_compare_timestamp_02() -> Result<()> {
    TZT::load("tzt_compare_timestamp_02.json")?.run()
}

#[test]
fn tzt_sub_int_int_00() -> Result<()> {
    TZT::load("tzt_sub_int_int_00.json")?.run()
}

#[test]
fn tzt_mem_bigmapnatnat_04() -> Result<()> {
    TZT::load("tzt_mem_bigmapnatnat_04.json")?.run()
}

#[test]
fn tzt_le_02() -> Result<()> {
    TZT::load("tzt_le_02.json")?.run()
}

#[test]
fn tzt_map_mapstringnat_02() -> Result<()> {
    TZT::load("tzt_map_mapstringnat_02.json")?.run()
}

#[test]
fn tzt_sub_timestamp_timestamp_01() -> Result<()> {
    TZT::load("tzt_sub_timestamp_timestamp_01.json")?.run()
}

#[test]
fn tzt_size_mapintint_00() -> Result<()> {
    TZT::load("tzt_size_mapintint_00.json")?.run()
}

#[test]
fn tzt_compare_mutez_01() -> Result<()> {
    TZT::load("tzt_compare_mutez_01.json")?.run()
}

#[test]
fn tzt_compare_bool_00() -> Result<()> {
    TZT::load("tzt_compare_bool_00.json")?.run()
}

#[test]
fn tzt_compare_mutez_02() -> Result<()> {
    TZT::load("tzt_compare_mutez_02.json")?.run()
}

#[test]
fn tzt_slice_bytes_03() -> Result<()> {
    TZT::load("tzt_slice_bytes_03.json")?.run()
}

#[test]
fn tzt_lsl_01() -> Result<()> {
    TZT::load("tzt_lsl_01.json")?.run()
}

#[test]
fn tzt_push_int_00() -> Result<()> {
    TZT::load("tzt_push_int_00.json")?.run()
}

#[test]
fn tzt_lsl_06() -> Result<()> {
    TZT::load("tzt_lsl_06.json")?.run()
}

#[test]
fn tzt_mul_int_nat_00() -> Result<()> {
    TZT::load("tzt_mul_int_nat_00.json")?.run()
}

#[test]
fn tzt_map_liststring_02() -> Result<()> {
    TZT::load("tzt_map_liststring_02.json")?.run()
}

#[test]
fn tzt_mem_mapstringnat_01() -> Result<()> {
    TZT::load("tzt_mem_mapstringnat_01.json")?.run()
}

#[test]
fn tzt_iter_setint_00() -> Result<()> {
    TZT::load("tzt_iter_setint_00.json")?.run()
}

#[test]
fn tzt_not_bool_01() -> Result<()> {
    TZT::load("tzt_not_bool_01.json")?.run()
}

#[test]
fn tzt_gt_02() -> Result<()> {
    TZT::load("tzt_gt_02.json")?.run()
}

#[test]
fn tzt_pair_int_int_00() -> Result<()> {
    TZT::load("tzt_pair_int_int_00.json")?.run()
}

#[test]
fn tzt_compare_timestamp_05() -> Result<()> {
    TZT::load("tzt_compare_timestamp_05.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_01() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_01.json")?.run()
}

#[test]
fn tzt_lsl_02() -> Result<()> {
    TZT::load("tzt_lsl_02.json")?.run()
}

#[test]
fn tzt_implicitaccount_00() -> Result<()> {
    TZT::load("tzt_implicitaccount_00.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_03() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_03.json")?.run()
}

#[test]
fn tzt_size_setint_02() -> Result<()> {
    TZT::load("tzt_size_setint_02.json")?.run()
}

#[test]
fn tzt_add_timestamp_int_01() -> Result<()> {
    TZT::load("tzt_add_timestamp_int_01.json")?.run()
}

#[test]
fn tzt_concat_bytes_00() -> Result<()> {
    TZT::load("tzt_concat_bytes_00.json")?.run()
}

#[test]
fn tzt_xor_bool_bool_02() -> Result<()> {
    TZT::load("tzt_xor_bool_bool_02.json")?.run()
}

#[test]
fn tzt_xor_nat_nat_06() -> Result<()> {
    TZT::load("tzt_xor_nat_nat_06.json")?.run()
}

#[test]
fn tzt_failwith_00() -> Result<()> {
    TZT::load("tzt_failwith_00.json")?.run()
}

#[test]
fn tzt_lsr_01() -> Result<()> {
    TZT::load("tzt_lsr_01.json")?.run()
}

#[test]
fn tzt_mem_bigmapnatnat_05() -> Result<()> {
    TZT::load("tzt_mem_bigmapnatnat_05.json")?.run()
}

#[test]
fn tzt_isnat_00() -> Result<()> {
    TZT::load("tzt_isnat_00.json")?.run()
}

#[test]
fn tzt_packunpack_int_00() -> Result<()> {
    TZT::load("tzt_packunpack_int_00.json")?.run()
}

#[test]
fn tzt_not_nat_03() -> Result<()> {
    TZT::load("tzt_not_nat_03.json")?.run()
}

#[test]
fn tzt_packunpack_timestamp_00() -> Result<()> {
    TZT::load("tzt_packunpack_timestamp_00.json")?.run()
}

#[test]
fn tzt_sub_mutez_mutez_01() -> Result<()> {
    TZT::load("tzt_sub_mutez_mutez_01.json")?.run()
}

#[test]
fn tzt_ge_03() -> Result<()> {
    TZT::load("tzt_ge_03.json")?.run()
}

#[test]
fn tzt_ediv_mutez_mutez_01() -> Result<()> {
    TZT::load("tzt_ediv_mutez_mutez_01.json")?.run()
}

#[test]
fn tzt_loop_01() -> Result<()> {
    TZT::load("tzt_loop_01.json")?.run()
}

#[test]
fn tzt_not_int_00() -> Result<()> {
    TZT::load("tzt_not_int_00.json")?.run()
}

#[test]
fn tzt_concat_liststring_03() -> Result<()> {
    TZT::load("tzt_concat_liststring_03.json")?.run()
}

#[test]
fn tzt_ifcons_listnat_01() -> Result<()> {
    TZT::load("tzt_ifcons_listnat_01.json")?.run()
}

#[test]
fn tzt_dig_04() -> Result<()> {
    TZT::load("tzt_dig_04.json")?.run()
}

#[test]
fn tzt_size_listint_01() -> Result<()> {
    TZT::load("tzt_size_listint_01.json")?.run()
}

#[test]
fn tzt_packunpack_keyhash_00() -> Result<()> {
    TZT::load("tzt_packunpack_keyhash_00.json")?.run()
}

#[test]
fn tzt_sub_mutez_mutez_00() -> Result<()> {
    TZT::load("tzt_sub_mutez_mutez_00.json")?.run()
}

#[test]
fn tzt_map_listint_03() -> Result<()> {
    TZT::load("tzt_map_listint_03.json")?.run()
}

#[test]
fn tzt_sub_timestamp_int_04() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_04.json")?.run()
}

#[test]
fn tzt_car_00() -> Result<()> {
    TZT::load("tzt_car_00.json")?.run()
}

#[test]
fn tzt_sub_timestamp_timestamp_00() -> Result<()> {
    TZT::load("tzt_sub_timestamp_timestamp_00.json")?.run()
}

#[test]
fn tzt_compare_nat_02() -> Result<()> {
    TZT::load("tzt_compare_nat_02.json")?.run()
}

#[test]
fn tzt_compare_bytes_02() -> Result<()> {
    TZT::load("tzt_compare_bytes_02.json")?.run()
}

#[test]
fn tzt_add_int_nat_01() -> Result<()> {
    TZT::load("tzt_add_int_nat_01.json")?.run()
}

#[test]
fn tzt_unit_00() -> Result<()> {
    TZT::load("tzt_unit_00.json")?.run()
}

#[test]
fn tzt_le_00() -> Result<()> {
    TZT::load("tzt_le_00.json")?.run()
}

#[test]
fn tzt_xor_bool_bool_01() -> Result<()> {
    TZT::load("tzt_xor_bool_bool_01.json")?.run()
}

#[test]
fn tzt_ge_01() -> Result<()> {
    TZT::load("tzt_ge_01.json")?.run()
}

#[test]
fn tzt_mul_mutez_nat_00() -> Result<()> {
    TZT::load("tzt_mul_mutez_nat_00.json")?.run()
}

#[test]
fn tzt_compare_timestamp_03() -> Result<()> {
    TZT::load("tzt_compare_timestamp_03.json")?.run()
}

#[test]
fn tzt_loop_00() -> Result<()> {
    TZT::load("tzt_loop_00.json")?.run()
}

#[test]
fn tzt_ifcons_listint_00() -> Result<()> {
    TZT::load("tzt_ifcons_listint_00.json")?.run()
}

#[test]
fn tzt_lsr_04() -> Result<()> {
    TZT::load("tzt_lsr_04.json")?.run()
}

#[test]
fn tzt_slice_string_02() -> Result<()> {
    TZT::load("tzt_slice_string_02.json")?.run()
}

#[test]
fn tzt_concat_string_00() -> Result<()> {
    TZT::load("tzt_concat_string_00.json")?.run()
}

#[test]
fn tzt_compare_keyhash_02() -> Result<()> {
    TZT::load("tzt_compare_keyhash_02.json")?.run()
}

#[test]
fn tzt_eq_01() -> Result<()> {
    TZT::load("tzt_eq_01.json")?.run()
}

#[test]
fn tzt_gt_00() -> Result<()> {
    TZT::load("tzt_gt_00.json")?.run()
}

#[test]
fn tzt_lt_04() -> Result<()> {
    TZT::load("tzt_lt_04.json")?.run()
}

#[test]
fn tzt_mul_int_int_00() -> Result<()> {
    TZT::load("tzt_mul_int_int_00.json")?.run()
}

#[test]
fn tzt_not_nat_07() -> Result<()> {
    TZT::load("tzt_not_nat_07.json")?.run()
}

#[test]
fn tzt_get_mapintint_01() -> Result<()> {
    TZT::load("tzt_get_mapintint_01.json")?.run()
}

#[test]
fn tzt_size_setstring_00() -> Result<()> {
    TZT::load("tzt_size_setstring_00.json")?.run()
}

#[test]
fn tzt_emptymap_nat_nat_00() -> Result<()> {
    TZT::load("tzt_emptymap_nat_nat_00.json")?.run()
}

#[test]
fn tzt_iter_setstring_01() -> Result<()> {
    TZT::load("tzt_iter_setstring_01.json")?.run()
}

#[test]
fn tzt_not_bool_00() -> Result<()> {
    TZT::load("tzt_not_bool_00.json")?.run()
}

#[test]
fn tzt_and_bool_bool_03() -> Result<()> {
    TZT::load("tzt_and_bool_bool_03.json")?.run()
}

#[test]
fn tzt_mem_mapnatnat_00() -> Result<()> {
    TZT::load("tzt_mem_mapnatnat_00.json")?.run()
}

#[test]
fn tzt_if_00() -> Result<()> {
    TZT::load("tzt_if_00.json")?.run()
}

#[test]
fn tzt_lsl_05() -> Result<()> {
    TZT::load("tzt_lsl_05.json")?.run()
}

#[test]
fn tzt_iter_mapstringstring_00() -> Result<()> {
    TZT::load("tzt_iter_mapstringstring_00.json")?.run()
}

#[test]
fn tzt_compare_mutez_03() -> Result<()> {
    TZT::load("tzt_compare_mutez_03.json")?.run()
}

#[test]
fn tzt_xor_bool_bool_00() -> Result<()> {
    TZT::load("tzt_xor_bool_bool_00.json")?.run()
}

#[test]
fn tzt_sub_timestamp_int_03() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_03.json")?.run()
}

#[test]
fn tzt_dig_02() -> Result<()> {
    TZT::load("tzt_dig_02.json")?.run()
}

#[test]
fn tzt_compare_timestamp_00() -> Result<()> {
    TZT::load("tzt_compare_timestamp_00.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_04() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_04.json")?.run()
}

#[test]
fn tzt_mul_nat_mutez_00() -> Result<()> {
    TZT::load("tzt_mul_nat_mutez_00.json")?.run()
}

#[test]
fn tzt_ediv_int_int_01() -> Result<()> {
    TZT::load("tzt_ediv_int_int_01.json")?.run()
}

#[test]
fn tzt_xor_nat_nat_03() -> Result<()> {
    TZT::load("tzt_xor_nat_nat_03.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_02() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_02.json")?.run()
}

#[test]
fn tzt_not_nat_06() -> Result<()> {
    TZT::load("tzt_not_nat_06.json")?.run()
}

#[test]
fn tzt_iter_liststring_01() -> Result<()> {
    TZT::load("tzt_iter_liststring_01.json")?.run()
}

#[test]
fn tzt_mem_mapstringnat_04() -> Result<()> {
    TZT::load("tzt_mem_mapstringnat_04.json")?.run()
}

#[test]
fn tzt_ge_00() -> Result<()> {
    TZT::load("tzt_ge_00.json")?.run()
}

#[test]
fn tzt_ediv_int_int_03() -> Result<()> {
    TZT::load("tzt_ediv_int_int_03.json")?.run()
}

#[test]
fn tzt_and_nat_nat_03() -> Result<()> {
    TZT::load("tzt_and_nat_nat_03.json")?.run()
}

#[test]
fn tzt_right_nat_int_00() -> Result<()> {
    TZT::load("tzt_right_nat_int_00.json")?.run()
}

#[test]
fn tzt_size_mapstringnat_01() -> Result<()> {
    TZT::load("tzt_size_mapstringnat_01.json")?.run()
}

#[test]
fn tzt_loop_02() -> Result<()> {
    TZT::load("tzt_loop_02.json")?.run()
}

#[test]
fn tzt_compare_mutez_05() -> Result<()> {
    TZT::load("tzt_compare_mutez_05.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_06() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_06.json")?.run()
}

#[test]
fn tzt_map_liststring_08() -> Result<()> {
    TZT::load("tzt_map_liststring_08.json")?.run()
}

#[test]
fn tzt_none_pair_nat_string() -> Result<()> {
    TZT::load("tzt_none_pair_nat_string.json")?.run()
}

#[test]
fn tzt_mul_nat_mutez_01() -> Result<()> {
    TZT::load("tzt_mul_nat_mutez_01.json")?.run()
}

#[test]
fn tzt_address_02() -> Result<()> {
    TZT::load("tzt_address_02.json")?.run()
}

#[test]
fn tzt_iter_listint_01() -> Result<()> {
    TZT::load("tzt_iter_listint_01.json")?.run()
}

#[test]
fn tzt_eq_02() -> Result<()> {
    TZT::load("tzt_eq_02.json")?.run()
}

#[test]
fn tzt_slice_string_00() -> Result<()> {
    TZT::load("tzt_slice_string_00.json")?.run()
}

#[test]
fn tzt_dig_00() -> Result<()> {
    TZT::load("tzt_dig_00.json")?.run()
}

#[test]
fn tzt_and_bool_bool_02() -> Result<()> {
    TZT::load("tzt_and_bool_bool_02.json")?.run()
}

#[test]
fn tzt_if_01() -> Result<()> {
    TZT::load("tzt_if_01.json")?.run()
}

#[test]
fn tzt_packunpack_address_00() -> Result<()> {
    TZT::load("tzt_packunpack_address_00.json")?.run()
}

#[test]
fn tzt_or_nat_nat_06() -> Result<()> {
    TZT::load("tzt_or_nat_nat_06.json")?.run()
}

#[test]
fn tzt_iter_mapintint_01() -> Result<()> {
    TZT::load("tzt_iter_mapintint_01.json")?.run()
}

#[test]
fn tzt_map_listint_06() -> Result<()> {
    TZT::load("tzt_map_listint_06.json")?.run()
}

#[test]
fn tzt_lt_01() -> Result<()> {
    TZT::load("tzt_lt_01.json")?.run()
}

#[test]
fn tzt_xor_nat_nat_01() -> Result<()> {
    TZT::load("tzt_xor_nat_nat_01.json")?.run()
}

#[test]
fn tzt_iter_listint_00() -> Result<()> {
    TZT::load("tzt_iter_listint_00.json")?.run()
}

#[test]
fn tzt_slice_string_01() -> Result<()> {
    TZT::load("tzt_slice_string_01.json")?.run()
}

#[test]
fn tzt_lsl_00() -> Result<()> {
    TZT::load("tzt_lsl_00.json")?.run()
}

#[test]
fn tzt_sender_00() -> Result<()> {
    TZT::load("tzt_sender_00.json")?.run()
}

#[test]
fn tzt_compare_string_02() -> Result<()> {
    TZT::load("tzt_compare_string_02.json")?.run()
}

#[test]
fn tzt_map_liststring_00() -> Result<()> {
    TZT::load("tzt_map_liststring_00.json")?.run()
}

#[test]
fn tzt_loopleft_03() -> Result<()> {
    TZT::load("tzt_loopleft_03.json")?.run()
}

#[test]
fn tzt_iter_setstring_00() -> Result<()> {
    TZT::load("tzt_iter_setstring_00.json")?.run()
}

#[test]
fn tzt_slice_string_03() -> Result<()> {
    TZT::load("tzt_slice_string_03.json")?.run()
}

#[test]
fn tzt_lt_02() -> Result<()> {
    TZT::load("tzt_lt_02.json")?.run()
}

#[test]
fn tzt_contract_03() -> Result<()> {
    TZT::load("tzt_contract_03.json")?.run()
}

#[test]
fn tzt_drop_00() -> Result<()> {
    TZT::load("tzt_drop_00.json")?.run()
}

#[test]
fn tzt_ge_04() -> Result<()> {
    TZT::load("tzt_ge_04.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_00() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_00.json")?.run()
}

#[test]
fn tzt_compare_nat_04() -> Result<()> {
    TZT::load("tzt_compare_nat_04.json")?.run()
}

#[test]
fn tzt_loopleft_00() -> Result<()> {
    TZT::load("tzt_loopleft_00.json")?.run()
}

#[test]
fn tzt_dig_03() -> Result<()> {
    TZT::load("tzt_dig_03.json")?.run()
}

#[test]
fn tzt_contract_00() -> Result<()> {
    TZT::load("tzt_contract_00.json")?.run()
}

#[test]
fn tzt_dropn_00() -> Result<()> {
    TZT::load("tzt_dropn_00.json")?.run()
}

#[test]
fn tzt_compare_bool_03() -> Result<()> {
    TZT::load("tzt_compare_bool_03.json")?.run()
}

#[test]
fn tzt_sub_timestamp_int_01() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_01.json")?.run()
}

#[test]
fn tzt_mul_nat_nat_00() -> Result<()> {
    TZT::load("tzt_mul_nat_nat_00.json")?.run()
}

#[test]
fn tzt_compare_keyhash_00() -> Result<()> {
    TZT::load("tzt_compare_keyhash_00.json")?.run()
}

#[test]
fn tzt_or_nat_nat_04() -> Result<()> {
    TZT::load("tzt_or_nat_nat_04.json")?.run()
}

#[test]
fn tzt_packunpack_string_00() -> Result<()> {
    TZT::load("tzt_packunpack_string_00.json")?.run()
}

#[test]
fn tzt_mem_mapnatnat_04() -> Result<()> {
    TZT::load("tzt_mem_mapnatnat_04.json")?.run()
}

#[test]
fn tzt_size_mapstringnat_00() -> Result<()> {
    TZT::load("tzt_size_mapstringnat_00.json")?.run()
}

#[test]
fn tzt_get_mapstringstring_02() -> Result<()> {
    TZT::load("tzt_get_mapstringstring_02.json")?.run()
}

#[test]
fn tzt_concat_liststring_04() -> Result<()> {
    TZT::load("tzt_concat_liststring_04.json")?.run()
}

#[test]
fn tzt_compare_int_04() -> Result<()> {
    TZT::load("tzt_compare_int_04.json")?.run()
}

#[test]
fn tzt_get_mapstringstring_01() -> Result<()> {
    TZT::load("tzt_get_mapstringstring_01.json")?.run()
}

#[test]
fn tzt_exec_03() -> Result<()> {
    TZT::load("tzt_exec_03.json")?.run()
}

#[test]
fn tzt_mem_bigmapnatnat_03() -> Result<()> {
    TZT::load("tzt_mem_bigmapnatnat_03.json")?.run()
}

#[test]
fn tzt_compare_string_01() -> Result<()> {
    TZT::load("tzt_compare_string_01.json")?.run()
}

#[test]
fn tzt_lt_00() -> Result<()> {
    TZT::load("tzt_lt_00.json")?.run()
}

#[test]
fn tzt_cdr_01() -> Result<()> {
    TZT::load("tzt_cdr_01.json")?.run()
}

#[test]
fn tzt_sub_timestamp_timestamp_02() -> Result<()> {
    TZT::load("tzt_sub_timestamp_timestamp_02.json")?.run()
}

#[test]
fn tzt_add_mutez_mutez_01() -> Result<()> {
    TZT::load("tzt_add_mutez_mutez_01.json")?.run()
}

#[test]
fn tzt_address_01() -> Result<()> {
    TZT::load("tzt_address_01.json")?.run()
}

#[test]
fn tzt_get_mapstringstring_00() -> Result<()> {
    TZT::load("tzt_get_mapstringstring_00.json")?.run()
}

#[test]
fn tzt_compare_int_01() -> Result<()> {
    TZT::load("tzt_compare_int_01.json")?.run()
}

#[test]
fn tzt_compare_string_00() -> Result<()> {
    TZT::load("tzt_compare_string_00.json")?.run()
}

#[test]
fn tzt_dropn_01() -> Result<()> {
    TZT::load("tzt_dropn_01.json")?.run()
}

#[test]
fn tzt_amount_00() -> Result<()> {
    TZT::load("tzt_amount_00.json")?.run()
}

#[test]
fn tzt_mem_setstring_01() -> Result<()> {
    TZT::load("tzt_mem_setstring_01.json")?.run()
}

#[test]
fn tzt_ge_02() -> Result<()> {
    TZT::load("tzt_ge_02.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_03() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_03.json")?.run()
}

#[test]
fn tzt_dugn_00() -> Result<()> {
    TZT::load("tzt_dugn_00.json")?.run()
}

#[test]
fn tzt_map_mapintstring_01() -> Result<()> {
    TZT::load("tzt_map_mapintstring_01.json")?.run()
}

#[test]
fn tzt_neq_04() -> Result<()> {
    TZT::load("tzt_neq_04.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_05() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_05.json")?.run()
}

#[test]
fn tzt_gt_01() -> Result<()> {
    TZT::load("tzt_gt_01.json")?.run()
}

#[test]
fn tzt_or_nat_nat_01() -> Result<()> {
    TZT::load("tzt_or_nat_nat_01.json")?.run()
}

#[test]
fn tzt_add_int_timestamp_00() -> Result<()> {
    TZT::load("tzt_add_int_timestamp_00.json")?.run()
}

#[test]
fn tzt_eq_03() -> Result<()> {
    TZT::load("tzt_eq_03.json")?.run()
}

#[test]
fn tzt_abs_00() -> Result<()> {
    TZT::load("tzt_abs_00.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_00() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_00.json")?.run()
}

#[test]
fn tzt_mem_setstring_00() -> Result<()> {
    TZT::load("tzt_mem_setstring_00.json")?.run()
}

#[test]
fn tzt_apply_00() -> Result<()> {
    TZT::load("tzt_apply_00.json")?.run()
}

#[test]
fn tzt_size_bytes_00() -> Result<()> {
    TZT::load("tzt_size_bytes_00.json")?.run()
}

#[test]
fn tzt_get_bigmapstringstring_00() -> Result<()> {
    TZT::load("tzt_get_bigmapstringstring_00.json")?.run()
}

#[test]
fn tzt_iter_setint_02() -> Result<()> {
    TZT::load("tzt_iter_setint_02.json")?.run()
}

#[test]
fn tzt_emptyset_nat_00() -> Result<()> {
    TZT::load("tzt_emptyset_nat_00.json")?.run()
}

#[test]
fn tzt_iter_mapintint_04() -> Result<()> {
    TZT::load("tzt_iter_mapintint_04.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_02() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_02.json")?.run()
}

#[test]
fn tzt_map_mapintint_00() -> Result<()> {
    TZT::load("tzt_map_mapintint_00.json")?.run()
}

#[test]
fn tzt_neg_int_01() -> Result<()> {
    TZT::load("tzt_neg_int_01.json")?.run()
}

#[test]
fn tzt_iter_mapintint_00() -> Result<()> {
    TZT::load("tzt_iter_mapintint_00.json")?.run()
}

#[test]
fn tzt_gt_04() -> Result<()> {
    TZT::load("tzt_gt_04.json")?.run()
}

#[test]
fn tzt_dip_02() -> Result<()> {
    TZT::load("tzt_dip_02.json")?.run()
}

#[test]
fn tzt_some_int_00() -> Result<()> {
    TZT::load("tzt_some_int_00.json")?.run()
}

#[test]
fn tzt_slice_bytes_02() -> Result<()> {
    TZT::load("tzt_slice_bytes_02.json")?.run()
}

#[test]
fn tzt_compare_nat_00() -> Result<()> {
    TZT::load("tzt_compare_nat_00.json")?.run()
}

#[test]
fn tzt_sub_timestamp_timestamp_03() -> Result<()> {
    TZT::load("tzt_sub_timestamp_timestamp_03.json")?.run()
}

#[test]
fn tzt_not_nat_04() -> Result<()> {
    TZT::load("tzt_not_nat_04.json")?.run()
}

#[test]
fn tzt_isnat_01() -> Result<()> {
    TZT::load("tzt_isnat_01.json")?.run()
}

#[test]
fn tzt_and_nat_nat_04() -> Result<()> {
    TZT::load("tzt_and_nat_nat_04.json")?.run()
}

#[test]
fn tzt_ediv_mutez_mutez_03() -> Result<()> {
    TZT::load("tzt_ediv_mutez_mutez_03.json")?.run()
}

#[test]
fn tzt_lsr_02() -> Result<()> {
    TZT::load("tzt_lsr_02.json")?.run()
}

#[test]
fn tzt_map_liststring_05() -> Result<()> {
    TZT::load("tzt_map_liststring_05.json")?.run()
}

#[test]
fn tzt_eq_00() -> Result<()> {
    TZT::load("tzt_eq_00.json")?.run()
}

#[test]
fn tzt_mem_setint_01() -> Result<()> {
    TZT::load("tzt_mem_setint_01.json")?.run()
}

#[test]
fn tzt_ifnone_optionnat_00() -> Result<()> {
    TZT::load("tzt_ifnone_optionnat_00.json")?.run()
}

#[test]
fn tzt_iter_listint_03() -> Result<()> {
    TZT::load("tzt_iter_listint_03.json")?.run()
}

#[test]
fn tzt_chain_id_01() -> Result<()> {
    TZT::load("tzt_chain_id_01.json")?.run()
}

#[test]
fn tzt_size_string_00() -> Result<()> {
    TZT::load("tzt_size_string_00.json")?.run()
}

#[test]
fn tzt_loopleft_04() -> Result<()> {
    TZT::load("tzt_loopleft_04.json")?.run()
}

#[test]
fn tzt_compare_timestamp_04() -> Result<()> {
    TZT::load("tzt_compare_timestamp_04.json")?.run()
}

#[test]
fn tzt_loopleft_01() -> Result<()> {
    TZT::load("tzt_loopleft_01.json")?.run()
}

#[test]
fn tzt_map_liststring_04() -> Result<()> {
    TZT::load("tzt_map_liststring_04.json")?.run()
}

#[test]
fn tzt_iter_mapintint_03() -> Result<()> {
    TZT::load("tzt_iter_mapintint_03.json")?.run()
}

#[test]
fn tzt_packunpack_bool_00() -> Result<()> {
    TZT::load("tzt_packunpack_bool_00.json")?.run()
}

#[test]
fn tzt_neq_02() -> Result<()> {
    TZT::load("tzt_neq_02.json")?.run()
}

#[test]
fn tzt_or_bool_bool_01() -> Result<()> {
    TZT::load("tzt_or_bool_bool_01.json")?.run()
}

#[test]
fn tzt_mem_bigmapnatnat_02() -> Result<()> {
    TZT::load("tzt_mem_bigmapnatnat_02.json")?.run()
}

#[test]
fn tzt_compare_nat_05() -> Result<()> {
    TZT::load("tzt_compare_nat_05.json")?.run()
}

#[test]
fn tzt_chain_id_00() -> Result<()> {
    TZT::load("tzt_chain_id_00.json")?.run()
}

#[test]
fn tzt_or_nat_nat_05() -> Result<()> {
    TZT::load("tzt_or_nat_nat_05.json")?.run()
}

#[test]
fn tzt_slice_string_05() -> Result<()> {
    TZT::load("tzt_slice_string_05.json")?.run()
}

#[test]
fn tzt_contract_01() -> Result<()> {
    TZT::load("tzt_contract_01.json")?.run()
}

#[test]
fn tzt_sub_timestamp_int_02() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_02.json")?.run()
}

#[test]
fn tzt_not_nat_00() -> Result<()> {
    TZT::load("tzt_not_nat_00.json")?.run()
}

#[test]
fn tzt_iter_mapintint_02() -> Result<()> {
    TZT::load("tzt_iter_mapintint_02.json")?.run()
}

#[test]
fn tzt_mem_bigmapstringnat_00() -> Result<()> {
    TZT::load("tzt_mem_bigmapstringnat_00.json")?.run()
}

#[test]
fn tzt_now_00() -> Result<()> {
    TZT::load("tzt_now_00.json")?.run()
}

#[test]
fn tzt_iter_setstring_02() -> Result<()> {
    TZT::load("tzt_iter_setstring_02.json")?.run()
}

#[test]
fn tzt_or_nat_nat_00() -> Result<()> {
    TZT::load("tzt_or_nat_nat_00.json")?.run()
}

#[test]
fn tzt_concat_listbytes_02() -> Result<()> {
    TZT::load("tzt_concat_listbytes_02.json")?.run()
}

#[test]
fn tzt_mem_bigmapstringnat_05() -> Result<()> {
    TZT::load("tzt_mem_bigmapstringnat_05.json")?.run()
}

#[test]
fn tzt_lsl_03() -> Result<()> {
    TZT::load("tzt_lsl_03.json")?.run()
}

#[test]
fn tzt_map_listint_00() -> Result<()> {
    TZT::load("tzt_map_listint_00.json")?.run()
}

#[test]
fn tzt_mem_setint_00() -> Result<()> {
    TZT::load("tzt_mem_setint_00.json")?.run()
}

#[test]
fn tzt_size_mapstringnat_02() -> Result<()> {
    TZT::load("tzt_size_mapstringnat_02.json")?.run()
}

#[test]
fn tzt_mem_bigmapstringnat_02() -> Result<()> {
    TZT::load("tzt_mem_bigmapstringnat_02.json")?.run()
}

#[test]
fn tzt_compare_int_02() -> Result<()> {
    TZT::load("tzt_compare_int_02.json")?.run()
}

#[test]
fn tzt_compare_bytes_01() -> Result<()> {
    TZT::load("tzt_compare_bytes_01.json")?.run()
}

#[test]
fn tzt_dip_00() -> Result<()> {
    TZT::load("tzt_dip_00.json")?.run()
}

#[test]
fn tzt_abs_01() -> Result<()> {
    TZT::load("tzt_abs_01.json")?.run()
}

#[test]
fn tzt_get_bigmapstringstring_02() -> Result<()> {
    TZT::load("tzt_get_bigmapstringstring_02.json")?.run()
}

#[test]
fn tzt_and_nat_nat_02() -> Result<()> {
    TZT::load("tzt_and_nat_nat_02.json")?.run()
}

#[test]
fn tzt_map_liststring_01() -> Result<()> {
    TZT::load("tzt_map_liststring_01.json")?.run()
}

#[test]
fn tzt_map_liststring_06() -> Result<()> {
    TZT::load("tzt_map_liststring_06.json")?.run()
}

#[test]
fn tzt_compare_pairintint_01() -> Result<()> {
    TZT::load("tzt_compare_pairintint_01.json")?.run()
}

#[test]
fn tzt_map_listint_02() -> Result<()> {
    TZT::load("tzt_map_listint_02.json")?.run()
}

#[test]
fn tzt_size_mapstringnat_03() -> Result<()> {
    TZT::load("tzt_size_mapstringnat_03.json")?.run()
}

#[test]
fn tzt_concat_listbytes_01() -> Result<()> {
    TZT::load("tzt_concat_listbytes_01.json")?.run()
}

#[test]
fn tzt_self_00() -> Result<()> {
    TZT::load("tzt_self_00.json")?.run()
}

#[test]
fn tzt_or_nat_nat_02() -> Result<()> {
    TZT::load("tzt_or_nat_nat_02.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_07() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_07.json")?.run()
}

#[test]
fn tzt_sub_int_int_01() -> Result<()> {
    TZT::load("tzt_sub_int_int_01.json")?.run()
}

#[test]
fn tzt_iter_liststring_00() -> Result<()> {
    TZT::load("tzt_iter_liststring_00.json")?.run()
}

#[test]
fn tzt_update_setint_02() -> Result<()> {
    TZT::load("tzt_update_setint_02.json")?.run()
}

#[test]
fn tzt_neg_nat_00() -> Result<()> {
    TZT::load("tzt_neg_nat_00.json")?.run()
}

#[test]
fn tzt_update_setint_00() -> Result<()> {
    TZT::load("tzt_update_setint_00.json")?.run()
}

#[test]
fn tzt_abs_02() -> Result<()> {
    TZT::load("tzt_abs_02.json")?.run()
}

#[test]
fn tzt_compare_bool_02() -> Result<()> {
    TZT::load("tzt_compare_bool_02.json")?.run()
}

#[test]
fn tzt_compare_pairintint_03() -> Result<()> {
    TZT::load("tzt_compare_pairintint_03.json")?.run()
}

#[test]
fn tzt_compare_string_04() -> Result<()> {
    TZT::load("tzt_compare_string_04.json")?.run()
}

#[test]
fn tzt_size_listint_02() -> Result<()> {
    TZT::load("tzt_size_listint_02.json")?.run()
}

#[test]
fn tzt_size_setint_00() -> Result<()> {
    TZT::load("tzt_size_setint_00.json")?.run()
}

#[test]
fn tzt_compare_mutez_00() -> Result<()> {
    TZT::load("tzt_compare_mutez_00.json")?.run()
}

#[test]
fn tzt_ediv_mutez_mutez_02() -> Result<()> {
    TZT::load("tzt_ediv_mutez_mutez_02.json")?.run()
}

#[test]
fn tzt_and_bool_bool_01() -> Result<()> {
    TZT::load("tzt_and_bool_bool_01.json")?.run()
}

#[test]
fn tzt_mem_mapstringnat_00() -> Result<()> {
    TZT::load("tzt_mem_mapstringnat_00.json")?.run()
}

#[test]
fn tzt_mem_mapstringnat_02() -> Result<()> {
    TZT::load("tzt_mem_mapstringnat_02.json")?.run()
}

#[test]
fn tzt_ediv_mutez_mutez_00() -> Result<()> {
    TZT::load("tzt_ediv_mutez_mutez_00.json")?.run()
}

#[test]
fn tzt_packunpack_bytes_00() -> Result<()> {
    TZT::load("tzt_packunpack_bytes_00.json")?.run()
}

#[test]
fn tzt_compare_keyhash_01() -> Result<()> {
    TZT::load("tzt_compare_keyhash_01.json")?.run()
}

#[test]
fn tzt_ifleft_orstringint_00() -> Result<()> {
    TZT::load("tzt_ifleft_orstringint_00.json")?.run()
}

#[test]
fn tzt_map_mapintint_01() -> Result<()> {
    TZT::load("tzt_map_mapintint_01.json")?.run()
}

#[test]
fn tzt_compare_bytes_00() -> Result<()> {
    TZT::load("tzt_compare_bytes_00.json")?.run()
}

#[test]
fn tzt_packunpack_mutez_00() -> Result<()> {
    TZT::load("tzt_packunpack_mutez_00.json")?.run()
}

#[test]
fn tzt_concat_listbytes_00() -> Result<()> {
    TZT::load("tzt_concat_listbytes_00.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_05() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_05.json")?.run()
}

#[test]
fn tzt_neg_int_00() -> Result<()> {
    TZT::load("tzt_neg_int_00.json")?.run()
}

#[test]
fn tzt_address_00() -> Result<()> {
    TZT::load("tzt_address_00.json")?.run()
}

#[test]
fn tzt_add_timestamp_int_03() -> Result<()> {
    TZT::load("tzt_add_timestamp_int_03.json")?.run()
}

#[test]
fn tzt_lsr_03() -> Result<()> {
    TZT::load("tzt_lsr_03.json")?.run()
}

#[test]
fn tzt_le_04() -> Result<()> {
    TZT::load("tzt_le_04.json")?.run()
}

#[test]
fn tzt_concat_string_01() -> Result<()> {
    TZT::load("tzt_concat_string_01.json")?.run()
}

#[test]
fn tzt_pair_pair_nat_string_pair_string_nat_00() -> Result<()> {
    TZT::load("tzt_pair_pair_nat_string_pair_string_nat_00.json")?.run()
}

#[test]
fn tzt_lt_03() -> Result<()> {
    TZT::load("tzt_lt_03.json")?.run()
}

#[test]
fn tzt_pair_nat_string_00() -> Result<()> {
    TZT::load("tzt_pair_nat_string_00.json")?.run()
}

#[test]
fn tzt_cons_string_00() -> Result<()> {
    TZT::load("tzt_cons_string_00.json")?.run()
}

#[test]
fn tzt_update_setint_01() -> Result<()> {
    TZT::load("tzt_update_setint_01.json")?.run()
}

#[test]
fn tzt_add_mutez_mutez_00() -> Result<()> {
    TZT::load("tzt_add_mutez_mutez_00.json")?.run()
}

#[test]
fn tzt_compare_int_00() -> Result<()> {
    TZT::load("tzt_compare_int_00.json")?.run()
}

#[test]
fn tzt_neq_03() -> Result<()> {
    TZT::load("tzt_neq_03.json")?.run()
}

#[test]
fn tzt_compare_mutez_04() -> Result<()> {
    TZT::load("tzt_compare_mutez_04.json")?.run()
}

#[test]
fn tzt_int_nat_01() -> Result<()> {
    TZT::load("tzt_int_nat_01.json")?.run()
}

#[test]
fn tzt_exec_01() -> Result<()> {
    TZT::load("tzt_exec_01.json")?.run()
}

#[test]
fn tzt_xor_bool_bool_03() -> Result<()> {
    TZT::load("tzt_xor_bool_bool_03.json")?.run()
}

#[test]
fn tzt_le_03() -> Result<()> {
    TZT::load("tzt_le_03.json")?.run()
}

#[test]
fn tzt_contract_04() -> Result<()> {
    TZT::load("tzt_contract_04.json")?.run()
}

#[test]
fn tzt_neq_01() -> Result<()> {
    TZT::load("tzt_neq_01.json")?.run()
}

#[test]
fn tzt_some_pairintint_00() -> Result<()> {
    TZT::load("tzt_some_pairintint_00.json")?.run()
}

#[test]
fn tzt_and_nat_nat_00() -> Result<()> {
    TZT::load("tzt_and_nat_nat_00.json")?.run()
}

#[test]
fn tzt_mem_bigmapnatnat_00() -> Result<()> {
    TZT::load("tzt_mem_bigmapnatnat_00.json")?.run()
}

#[test]
fn tzt_mem_bigmapnatnat_01() -> Result<()> {
    TZT::load("tzt_mem_bigmapnatnat_01.json")?.run()
}

#[test]
fn tzt_mem_mapstringnat_05() -> Result<()> {
    TZT::load("tzt_mem_mapstringnat_05.json")?.run()
}

#[test]
fn tzt_and_bool_bool_00() -> Result<()> {
    TZT::load("tzt_and_bool_bool_00.json")?.run()
}

#[test]
fn tzt_mem_setstring_02() -> Result<()> {
    TZT::load("tzt_mem_setstring_02.json")?.run()
}

#[test]
fn tzt_compare_bytes_03() -> Result<()> {
    TZT::load("tzt_compare_bytes_03.json")?.run()
}

#[test]
fn tzt_xor_nat_nat_02() -> Result<()> {
    TZT::load("tzt_xor_nat_nat_02.json")?.run()
}

#[test]
fn tzt_emptymap_string_string_00() -> Result<()> {
    TZT::load("tzt_emptymap_string_string_00.json")?.run()
}

#[test]
fn tzt_mem_mapnatnat_02() -> Result<()> {
    TZT::load("tzt_mem_mapnatnat_02.json")?.run()
}

#[test]
fn tzt_add_int_int_00() -> Result<()> {
    TZT::load("tzt_add_int_int_00.json")?.run()
}

#[test]
fn tzt_source_00() -> Result<()> {
    TZT::load("tzt_source_00.json")?.run()
}

#[test]
fn tzt_cons_int_02() -> Result<()> {
    TZT::load("tzt_cons_int_02.json")?.run()
}

#[test]
fn tzt_or_nat_nat_03() -> Result<()> {
    TZT::load("tzt_or_nat_nat_03.json")?.run()
}

#[test]
fn tzt_neq_00() -> Result<()> {
    TZT::load("tzt_neq_00.json")?.run()
}

#[test]
fn tzt_slice_string_04() -> Result<()> {
    TZT::load("tzt_slice_string_04.json")?.run()
}

#[test]
fn tzt_map_listint_04() -> Result<()> {
    TZT::load("tzt_map_listint_04.json")?.run()
}

#[test]
fn tzt_slice_bytes_04() -> Result<()> {
    TZT::load("tzt_slice_bytes_04.json")?.run()
}

#[test]
fn tzt_mul_mutez_nat_01() -> Result<()> {
    TZT::load("tzt_mul_mutez_nat_01.json")?.run()
}

#[test]
fn tzt_dipn_00() -> Result<()> {
    TZT::load("tzt_dipn_00.json")?.run()
}

#[test]
fn tzt_update_mapintint_01() -> Result<()> {
    TZT::load("tzt_update_mapintint_01.json")?.run()
}

#[test]
fn tzt_ediv_int_int_02() -> Result<()> {
    TZT::load("tzt_ediv_int_int_02.json")?.run()
}

#[test]
fn tzt_mem_mapnatnat_01() -> Result<()> {
    TZT::load("tzt_mem_mapnatnat_01.json")?.run()
}

#[test]
fn tzt_add_nat_nat_00() -> Result<()> {
    TZT::load("tzt_add_nat_nat_00.json")?.run()
}

#[test]
fn tzt_dip_01() -> Result<()> {
    TZT::load("tzt_dip_01.json")?.run()
}

#[test]
fn tzt_size_listint_03() -> Result<()> {
    TZT::load("tzt_size_listint_03.json")?.run()
}

#[test]
fn tzt_dig_01() -> Result<()> {
    TZT::load("tzt_dig_01.json")?.run()
}

#[test]
fn tzt_concat_bytes_01() -> Result<()> {
    TZT::load("tzt_concat_bytes_01.json")?.run()
}

#[test]
fn tzt_neg_nat_01() -> Result<()> {
    TZT::load("tzt_neg_nat_01.json")?.run()
}

#[test]
fn tzt_slice_bytes_01() -> Result<()> {
    TZT::load("tzt_slice_bytes_01.json")?.run()
}

#[test]
fn tzt_unpair_pairstringstring_00() -> Result<()> {
    TZT::load("tzt_unpair_pairstringstring_00.json")?.run()
}

#[test]
fn tzt_eq_04() -> Result<()> {
    TZT::load("tzt_eq_04.json")?.run()
}

#[test]
fn tzt_cdr_00() -> Result<()> {
    TZT::load("tzt_cdr_00.json")?.run()
}
