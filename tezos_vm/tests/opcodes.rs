mod runner;

use runner::e2e::E2E;
use tezos_vm::Result;

#[test]
fn e2e_cons_00() -> Result<()> {
    E2E::load("e2e_cons_00.json")?.run()
}

#[test]
fn e2e_cons_01() -> Result<()> {
    E2E::load("e2e_cons_01.json")?.run()
}

#[test]
fn e2e_cons_02() -> Result<()> {
    E2E::load("e2e_cons_02.json")?.run()
}

#[test]
fn e2e_none_00() -> Result<()> {
    E2E::load("e2e_none_00.json")?.run()
}

#[test]
fn e2e_ret_int_00() -> Result<()> {
    E2E::load("e2e_ret_int_00.json")?.run()
}

#[test]
fn e2e_list_map_block_00() -> Result<()> {
    E2E::load("e2e_list_map_block_00.json")?.run()
}

#[test]
fn e2e_list_map_block_01() -> Result<()> {
    E2E::load("e2e_list_map_block_01.json")?.run()
}

#[test]
fn e2e_list_map_block_02() -> Result<()> {
    E2E::load("e2e_list_map_block_02.json")?.run()
}

#[test]
fn e2e_reverse_00() -> Result<()> {
    E2E::load("e2e_reverse_00.json")?.run()
}

#[test]
fn e2e_reverse_01() -> Result<()> {
    E2E::load("e2e_reverse_01.json")?.run()
}

#[test]
fn e2e_loop_left_00() -> Result<()> {
    E2E::load("e2e_loop_left_00.json")?.run()
}

#[test]
fn e2e_loop_left_01() -> Result<()> {
    E2E::load("e2e_loop_left_01.json")?.run()
}

#[test]
fn e2e_str_id_00() -> Result<()> {
    E2E::load("e2e_str_id_00.json")?.run()
}

#[test]
fn e2e_str_id_01() -> Result<()> {
    E2E::load("e2e_str_id_01.json")?.run()
}

#[test]
fn e2e_slice_00() -> Result<()> {
    E2E::load("e2e_slice_00.json")?.run()
}

#[test]
fn e2e_slice_01() -> Result<()> {
    E2E::load("e2e_slice_01.json")?.run()
}

#[test]
fn e2e_slice_02() -> Result<()> {
    E2E::load("e2e_slice_02.json")?.run()
}

#[test]
fn e2e_slice_03() -> Result<()> {
    E2E::load("e2e_slice_03.json")?.run()
}

#[test]
fn e2e_slice_04() -> Result<()> {
    E2E::load("e2e_slice_04.json")?.run()
}

#[test]
fn e2e_slice_05() -> Result<()> {
    E2E::load("e2e_slice_05.json")?.run()
}

#[test]
fn e2e_slice_06() -> Result<()> {
    E2E::load("e2e_slice_06.json")?.run()
}

#[test]
fn e2e_slice_07() -> Result<()> {
    E2E::load("e2e_slice_07.json")?.run()
}

#[test]
fn e2e_slice_bytes_00() -> Result<()> {
    E2E::load("e2e_slice_bytes_00.json")?.run()
}

#[test]
fn e2e_slice_bytes_01() -> Result<()> {
    E2E::load("e2e_slice_bytes_01.json")?.run()
}

#[test]
fn e2e_slice_bytes_02() -> Result<()> {
    E2E::load("e2e_slice_bytes_02.json")?.run()
}

#[test]
fn e2e_slice_bytes_03() -> Result<()> {
    E2E::load("e2e_slice_bytes_03.json")?.run()
}

#[test]
fn e2e_slice_bytes_04() -> Result<()> {
    E2E::load("e2e_slice_bytes_04.json")?.run()
}

#[test]
fn e2e_slice_bytes_05() -> Result<()> {
    E2E::load("e2e_slice_bytes_05.json")?.run()
}

#[test]
fn e2e_slice_bytes_06() -> Result<()> {
    E2E::load("e2e_slice_bytes_06.json")?.run()
}

#[test]
fn e2e_slice_bytes_07() -> Result<()> {
    E2E::load("e2e_slice_bytes_07.json")?.run()
}

#[test]
fn e2e_pair_id_00() -> Result<()> {
    E2E::load("e2e_pair_id_00.json")?.run()
}

#[test]
fn e2e_pair_id_01() -> Result<()> {
    E2E::load("e2e_pair_id_01.json")?.run()
}

#[test]
fn e2e_pair_id_02() -> Result<()> {
    E2E::load("e2e_pair_id_02.json")?.run()
}

#[test]
fn e2e_pair_id_03() -> Result<()> {
    E2E::load("e2e_pair_id_03.json")?.run()
}

#[test]
fn e2e_car_00() -> Result<()> {
    E2E::load("e2e_car_00.json")?.run()
}

#[test]
fn e2e_cdr_00() -> Result<()> {
    E2E::load("e2e_cdr_00.json")?.run()
}

#[test]
fn e2e_not_00() -> Result<()> {
    E2E::load("e2e_not_00.json")?.run()
}

#[test]
fn e2e_not_01() -> Result<()> {
    E2E::load("e2e_not_01.json")?.run()
}

#[test]
fn e2e_and_00() -> Result<()> {
    E2E::load("e2e_and_00.json")?.run()
}

#[test]
fn e2e_and_01() -> Result<()> {
    E2E::load("e2e_and_01.json")?.run()
}

#[test]
fn e2e_and_02() -> Result<()> {
    E2E::load("e2e_and_02.json")?.run()
}

#[test]
fn e2e_and_03() -> Result<()> {
    E2E::load("e2e_and_03.json")?.run()
}

#[test]
fn e2e_or_00() -> Result<()> {
    E2E::load("e2e_or_00.json")?.run()
}

#[test]
fn e2e_or_01() -> Result<()> {
    E2E::load("e2e_or_01.json")?.run()
}

#[test]
fn e2e_or_02() -> Result<()> {
    E2E::load("e2e_or_02.json")?.run()
}

#[test]
fn e2e_or_03() -> Result<()> {
    E2E::load("e2e_or_03.json")?.run()
}

#[test]
fn e2e_and_logical_1_00() -> Result<()> {
    E2E::load("e2e_and_logical_1_00.json")?.run()
}

#[test]
fn e2e_and_logical_1_01() -> Result<()> {
    E2E::load("e2e_and_logical_1_01.json")?.run()
}

#[test]
fn e2e_and_logical_1_02() -> Result<()> {
    E2E::load("e2e_and_logical_1_02.json")?.run()
}

#[test]
fn e2e_and_logical_1_03() -> Result<()> {
    E2E::load("e2e_and_logical_1_03.json")?.run()
}

#[test]
fn e2e_and_binary_00() -> Result<()> {
    E2E::load("e2e_and_binary_00.json")?.run()
}

#[test]
fn e2e_or_binary_00() -> Result<()> {
    E2E::load("e2e_or_binary_00.json")?.run()
}

#[test]
fn e2e_or_binary_01() -> Result<()> {
    E2E::load("e2e_or_binary_01.json")?.run()
}

#[test]
fn e2e_or_binary_02() -> Result<()> {
    E2E::load("e2e_or_binary_02.json")?.run()
}

#[test]
fn e2e_or_binary_03() -> Result<()> {
    E2E::load("e2e_or_binary_03.json")?.run()
}

#[test]
fn e2e_or_binary_04() -> Result<()> {
    E2E::load("e2e_or_binary_04.json")?.run()
}

#[test]
fn e2e_or_binary_05() -> Result<()> {
    E2E::load("e2e_or_binary_05.json")?.run()
}

#[test]
fn e2e_not_binary_00() -> Result<()> {
    E2E::load("e2e_not_binary_00.json")?.run()
}

#[test]
fn e2e_not_binary_01() -> Result<()> {
    E2E::load("e2e_not_binary_01.json")?.run()
}

#[test]
fn e2e_not_binary_02() -> Result<()> {
    E2E::load("e2e_not_binary_02.json")?.run()
}

#[test]
fn e2e_not_binary_03() -> Result<()> {
    E2E::load("e2e_not_binary_03.json")?.run()
}

#[test]
fn e2e_not_binary_04() -> Result<()> {
    E2E::load("e2e_not_binary_04.json")?.run()
}

#[test]
fn e2e_not_binary_05() -> Result<()> {
    E2E::load("e2e_not_binary_05.json")?.run()
}

#[test]
fn e2e_not_binary_06() -> Result<()> {
    E2E::load("e2e_not_binary_06.json")?.run()
}

#[test]
fn e2e_not_binary_07() -> Result<()> {
    E2E::load("e2e_not_binary_07.json")?.run()
}

#[test]
fn e2e_xor_00() -> Result<()> {
    E2E::load("e2e_xor_00.json")?.run()
}

#[test]
fn e2e_xor_01() -> Result<()> {
    E2E::load("e2e_xor_01.json")?.run()
}

#[test]
fn e2e_xor_02() -> Result<()> {
    E2E::load("e2e_xor_02.json")?.run()
}

#[test]
fn e2e_xor_03() -> Result<()> {
    E2E::load("e2e_xor_03.json")?.run()
}

#[test]
fn e2e_xor_04() -> Result<()> {
    E2E::load("e2e_xor_04.json")?.run()
}

#[test]
fn e2e_xor_05() -> Result<()> {
    E2E::load("e2e_xor_05.json")?.run()
}

#[test]
fn e2e_xor_06() -> Result<()> {
    E2E::load("e2e_xor_06.json")?.run()
}

#[test]
fn e2e_xor_07() -> Result<()> {
    E2E::load("e2e_xor_07.json")?.run()
}

#[test]
fn e2e_xor_08() -> Result<()> {
    E2E::load("e2e_xor_08.json")?.run()
}

#[test]
fn e2e_xor_09() -> Result<()> {
    E2E::load("e2e_xor_09.json")?.run()
}

#[test]
fn e2e_shifts_00() -> Result<()> {
    E2E::load("e2e_shifts_00.json")?.run()
}

#[test]
fn e2e_shifts_01() -> Result<()> {
    E2E::load("e2e_shifts_01.json")?.run()
}

#[test]
fn e2e_shifts_02() -> Result<()> {
    E2E::load("e2e_shifts_02.json")?.run()
}

#[test]
fn e2e_shifts_03() -> Result<()> {
    E2E::load("e2e_shifts_03.json")?.run()
}

#[test]
fn e2e_shifts_04() -> Result<()> {
    E2E::load("e2e_shifts_04.json")?.run()
}

#[test]
fn e2e_shifts_05() -> Result<()> {
    E2E::load("e2e_shifts_05.json")?.run()
}

#[test]
fn e2e_shifts_06() -> Result<()> {
    E2E::load("e2e_shifts_06.json")?.run()
}

#[test]
fn e2e_shifts_07() -> Result<()> {
    E2E::load("e2e_shifts_07.json")?.run()
}

#[test]
fn e2e_shifts_08() -> Result<()> {
    E2E::load("e2e_shifts_08.json")?.run()
}

#[test]
fn e2e_shifts_09() -> Result<()> {
    E2E::load("e2e_shifts_09.json")?.run()
}

#[test]
fn e2e_concat_list_00() -> Result<()> {
    E2E::load("e2e_concat_list_00.json")?.run()
}

#[test]
fn e2e_concat_list_01() -> Result<()> {
    E2E::load("e2e_concat_list_01.json")?.run()
}

#[test]
fn e2e_concat_list_02() -> Result<()> {
    E2E::load("e2e_concat_list_02.json")?.run()
}

#[test]
fn e2e_concat_hello_bytes_00() -> Result<()> {
    E2E::load("e2e_concat_hello_bytes_00.json")?.run()
}

#[test]
fn e2e_concat_hello_bytes_01() -> Result<()> {
    E2E::load("e2e_concat_hello_bytes_01.json")?.run()
}

#[test]
fn e2e_concat_hello_bytes_02() -> Result<()> {
    E2E::load("e2e_concat_hello_bytes_02.json")?.run()
}

#[test]
fn e2e_list_id_00() -> Result<()> {
    E2E::load("e2e_list_id_00.json")?.run()
}

#[test]
fn e2e_list_id_01() -> Result<()> {
    E2E::load("e2e_list_id_01.json")?.run()
}

#[test]
fn e2e_list_id_02() -> Result<()> {
    E2E::load("e2e_list_id_02.json")?.run()
}

#[test]
fn e2e_list_id_map_00() -> Result<()> {
    E2E::load("e2e_list_id_map_00.json")?.run()
}

#[test]
fn e2e_list_id_map_01() -> Result<()> {
    E2E::load("e2e_list_id_map_01.json")?.run()
}

#[test]
fn e2e_list_id_map_02() -> Result<()> {
    E2E::load("e2e_list_id_map_02.json")?.run()
}

#[test]
fn e2e_map_id_00() -> Result<()> {
    E2E::load("e2e_map_id_00.json")?.run()
}

#[test]
fn e2e_map_id_01() -> Result<()> {
    E2E::load("e2e_map_id_01.json")?.run()
}

#[test]
fn e2e_map_id_02() -> Result<()> {
    E2E::load("e2e_map_id_02.json")?.run()
}

#[test]
fn e2e_map_mem_nat_00() -> Result<()> {
    E2E::load("e2e_map_mem_nat_00.json")?.run()
}

#[test]
fn e2e_map_mem_nat_01() -> Result<()> {
    E2E::load("e2e_map_mem_nat_01.json")?.run()
}

#[test]
fn e2e_map_mem_nat_02() -> Result<()> {
    E2E::load("e2e_map_mem_nat_02.json")?.run()
}

#[test]
fn e2e_map_mem_nat_03() -> Result<()> {
    E2E::load("e2e_map_mem_nat_03.json")?.run()
}

#[test]
fn e2e_map_mem_nat_04() -> Result<()> {
    E2E::load("e2e_map_mem_nat_04.json")?.run()
}

#[test]
fn e2e_map_mem_nat_05() -> Result<()> {
    E2E::load("e2e_map_mem_nat_05.json")?.run()
}

#[test]
fn e2e_map_mem_string_00() -> Result<()> {
    E2E::load("e2e_map_mem_string_00.json")?.run()
}

#[test]
fn e2e_map_mem_string_01() -> Result<()> {
    E2E::load("e2e_map_mem_string_01.json")?.run()
}

#[test]
fn e2e_map_mem_string_02() -> Result<()> {
    E2E::load("e2e_map_mem_string_02.json")?.run()
}

#[test]
fn e2e_map_mem_string_03() -> Result<()> {
    E2E::load("e2e_map_mem_string_03.json")?.run()
}

#[test]
fn e2e_map_mem_string_04() -> Result<()> {
    E2E::load("e2e_map_mem_string_04.json")?.run()
}

#[test]
fn e2e_map_mem_string_05() -> Result<()> {
    E2E::load("e2e_map_mem_string_05.json")?.run()
}

#[test]
fn e2e_map_map_00() -> Result<()> {
    E2E::load("e2e_map_map_00.json")?.run()
}

#[test]
fn e2e_map_map_01() -> Result<()> {
    E2E::load("e2e_map_map_01.json")?.run()
}

#[test]
fn e2e_map_map_02() -> Result<()> {
    E2E::load("e2e_map_map_02.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_00() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_00.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_01() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_01.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_02() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_02.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_03() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_03.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_04() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_04.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_05() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_05.json")?.run()
}

#[test]
fn e2e_big_map_mem_string_00() -> Result<()> {
    E2E::load("e2e_big_map_mem_string_00.json")?.run()
}

#[test]
fn e2e_big_map_mem_string_01() -> Result<()> {
    E2E::load("e2e_big_map_mem_string_01.json")?.run()
}

#[test]
fn e2e_big_map_mem_string_02() -> Result<()> {
    E2E::load("e2e_big_map_mem_string_02.json")?.run()
}

#[test]
fn e2e_big_map_mem_string_03() -> Result<()> {
    E2E::load("e2e_big_map_mem_string_03.json")?.run()
}

#[test]
fn e2e_big_map_mem_string_04() -> Result<()> {
    E2E::load("e2e_big_map_mem_string_04.json")?.run()
}

#[test]
fn e2e_big_map_mem_string_05() -> Result<()> {
    E2E::load("e2e_big_map_mem_string_05.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_06() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_06.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_07() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_07.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_08() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_08.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_09() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_09.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_10() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_10.json")?.run()
}

#[test]
fn e2e_big_map_mem_nat_11() -> Result<()> {
    E2E::load("e2e_big_map_mem_nat_11.json")?.run()
}

#[test]
fn e2e_set_id_00() -> Result<()> {
    E2E::load("e2e_set_id_00.json")?.run()
}

#[test]
fn e2e_set_id_01() -> Result<()> {
    E2E::load("e2e_set_id_01.json")?.run()
}

#[test]
fn e2e_set_id_02() -> Result<()> {
    E2E::load("e2e_set_id_02.json")?.run()
}

#[test]
fn e2e_list_concat_00() -> Result<()> {
    E2E::load("e2e_list_concat_00.json")?.run()
}

#[test]
fn e2e_list_concat_01() -> Result<()> {
    E2E::load("e2e_list_concat_01.json")?.run()
}

#[test]
fn e2e_list_concat_bytes_00() -> Result<()> {
    E2E::load("e2e_list_concat_bytes_00.json")?.run()
}

#[test]
fn e2e_list_concat_bytes_01() -> Result<()> {
    E2E::load("e2e_list_concat_bytes_01.json")?.run()
}

#[test]
fn e2e_list_concat_bytes_02() -> Result<()> {
    E2E::load("e2e_list_concat_bytes_02.json")?.run()
}

#[test]
fn e2e_list_concat_bytes_03() -> Result<()> {
    E2E::load("e2e_list_concat_bytes_03.json")?.run()
}

#[test]
fn e2e_list_iter_00() -> Result<()> {
    E2E::load("e2e_list_iter_00.json")?.run()
}

#[test]
fn e2e_list_iter_01() -> Result<()> {
    E2E::load("e2e_list_iter_01.json")?.run()
}

#[test]
fn e2e_list_size_00() -> Result<()> {
    E2E::load("e2e_list_size_00.json")?.run()
}

#[test]
fn e2e_list_size_01() -> Result<()> {
    E2E::load("e2e_list_size_01.json")?.run()
}

#[test]
fn e2e_list_size_02() -> Result<()> {
    E2E::load("e2e_list_size_02.json")?.run()
}

#[test]
fn e2e_list_size_03() -> Result<()> {
    E2E::load("e2e_list_size_03.json")?.run()
}

#[test]
fn e2e_set_member_00() -> Result<()> {
    E2E::load("e2e_set_member_00.json")?.run()
}

#[test]
fn e2e_set_member_01() -> Result<()> {
    E2E::load("e2e_set_member_01.json")?.run()
}

#[test]
fn e2e_set_member_02() -> Result<()> {
    E2E::load("e2e_set_member_02.json")?.run()
}

#[test]
fn e2e_set_size_00() -> Result<()> {
    E2E::load("e2e_set_size_00.json")?.run()
}

#[test]
fn e2e_set_size_01() -> Result<()> {
    E2E::load("e2e_set_size_01.json")?.run()
}

#[test]
fn e2e_set_size_02() -> Result<()> {
    E2E::load("e2e_set_size_02.json")?.run()
}

#[test]
fn e2e_set_size_03() -> Result<()> {
    E2E::load("e2e_set_size_03.json")?.run()
}

#[test]
fn e2e_set_iter_00() -> Result<()> {
    E2E::load("e2e_set_iter_00.json")?.run()
}

#[test]
fn e2e_set_iter_01() -> Result<()> {
    E2E::load("e2e_set_iter_01.json")?.run()
}

#[test]
fn e2e_set_iter_02() -> Result<()> {
    E2E::load("e2e_set_iter_02.json")?.run()
}

#[test]
fn e2e_map_size_00() -> Result<()> {
    E2E::load("e2e_map_size_00.json")?.run()
}

#[test]
fn e2e_map_size_01() -> Result<()> {
    E2E::load("e2e_map_size_01.json")?.run()
}

#[test]
fn e2e_map_size_02() -> Result<()> {
    E2E::load("e2e_map_size_02.json")?.run()
}

#[test]
fn e2e_map_size_03() -> Result<()> {
    E2E::load("e2e_map_size_03.json")?.run()
}

#[test]
fn e2e_contains_all_00() -> Result<()> {
    E2E::load("e2e_contains_all_00.json")?.run()
}

#[test]
fn e2e_contains_all_01() -> Result<()> {
    E2E::load("e2e_contains_all_01.json")?.run()
}

#[test]
fn e2e_contains_all_02() -> Result<()> {
    E2E::load("e2e_contains_all_02.json")?.run()
}

#[test]
fn e2e_contains_all_03() -> Result<()> {
    E2E::load("e2e_contains_all_03.json")?.run()
}

#[test]
fn e2e_contains_all_04() -> Result<()> {
    E2E::load("e2e_contains_all_04.json")?.run()
}

#[test]
fn e2e_contains_all_05() -> Result<()> {
    E2E::load("e2e_contains_all_05.json")?.run()
}

#[test]
fn e2e_concat_hello_00() -> Result<()> {
    E2E::load("e2e_concat_hello_00.json")?.run()
}

#[test]
fn e2e_concat_hello_01() -> Result<()> {
    E2E::load("e2e_concat_hello_01.json")?.run()
}

#[test]
fn e2e_concat_hello_02() -> Result<()> {
    E2E::load("e2e_concat_hello_02.json")?.run()
}

#[test]
fn e2e_empty_map_00() -> Result<()> {
    E2E::load("e2e_empty_map_00.json")?.run()
}

#[test]
fn e2e_get_map_value_00() -> Result<()> {
    E2E::load("e2e_get_map_value_00.json")?.run()
}

#[test]
fn e2e_get_map_value_01() -> Result<()> {
    E2E::load("e2e_get_map_value_01.json")?.run()
}

#[test]
fn e2e_get_map_value_02() -> Result<()> {
    E2E::load("e2e_get_map_value_02.json")?.run()
}

#[test]
fn e2e_get_and_update_map_00() -> Result<()> {
    E2E::load("e2e_get_and_update_map_00.json")?.run()
}

#[test]
fn e2e_get_and_update_map_01() -> Result<()> {
    E2E::load("e2e_get_and_update_map_01.json")?.run()
}

#[test]
fn e2e_get_and_update_map_02() -> Result<()> {
    E2E::load("e2e_get_and_update_map_02.json")?.run()
}

#[test]
fn e2e_get_and_update_map_03() -> Result<()> {
    E2E::load("e2e_get_and_update_map_03.json")?.run()
}

#[test]
fn e2e_get_and_update_map_04() -> Result<()> {
    E2E::load("e2e_get_and_update_map_04.json")?.run()
}

#[test]
fn e2e_get_and_update_map_05() -> Result<()> {
    E2E::load("e2e_get_and_update_map_05.json")?.run()
}

#[test]
fn e2e_get_and_update_map_06() -> Result<()> {
    E2E::load("e2e_get_and_update_map_06.json")?.run()
}

#[test]
fn e2e_map_iter_00() -> Result<()> {
    E2E::load("e2e_map_iter_00.json")?.run()
}

#[test]
fn e2e_map_iter_01() -> Result<()> {
    E2E::load("e2e_map_iter_01.json")?.run()
}

#[test]
fn e2e_if_00() -> Result<()> {
    E2E::load("e2e_if_00.json")?.run()
}

#[test]
fn e2e_if_01() -> Result<()> {
    E2E::load("e2e_if_01.json")?.run()
}

#[test]
fn e2e_left_right_00() -> Result<()> {
    E2E::load("e2e_left_right_00.json")?.run()
}

#[test]
fn e2e_left_right_01() -> Result<()> {
    E2E::load("e2e_left_right_01.json")?.run()
}

#[test]
fn e2e_reverse_loop_00() -> Result<()> {
    E2E::load("e2e_reverse_loop_00.json")?.run()
}

#[test]
fn e2e_reverse_loop_01() -> Result<()> {
    E2E::load("e2e_reverse_loop_01.json")?.run()
}

#[test]
fn e2e_exec_concat_00() -> Result<()> {
    E2E::load("e2e_exec_concat_00.json")?.run()
}

#[test]
fn e2e_exec_concat_01() -> Result<()> {
    E2E::load("e2e_exec_concat_01.json")?.run()
}

#[test]
fn e2e_balance_00() -> Result<()> {
    E2E::load("e2e_balance_00.json")?.run()
}

#[test]
fn e2e_level_00() -> Result<()> {
    E2E::load("e2e_level_00.json")?.run()
}

#[test]
fn e2e_tez_add_sub_00() -> Result<()> {
    E2E::load("e2e_tez_add_sub_00.json")?.run()
}

#[test]
fn e2e_tez_add_sub_01() -> Result<()> {
    E2E::load("e2e_tez_add_sub_01.json")?.run()
}

#[test]
fn e2e_tez_add_sub_02() -> Result<()> {
    E2E::load("e2e_tez_add_sub_02.json")?.run()
}

#[test]
fn e2e_tez_add_sub_03() -> Result<()> {
    E2E::load("e2e_tez_add_sub_03.json")?.run()
}

#[test]
fn e2e_add_00() -> Result<()> {
    E2E::load("e2e_add_00.json")?.run()
}

#[test]
fn e2e_abs_00() -> Result<()> {
    E2E::load("e2e_abs_00.json")?.run()
}

#[test]
fn e2e_abs_01() -> Result<()> {
    E2E::load("e2e_abs_01.json")?.run()
}

#[test]
fn e2e_abs_02() -> Result<()> {
    E2E::load("e2e_abs_02.json")?.run()
}

#[test]
fn e2e_int_00() -> Result<()> {
    E2E::load("e2e_int_00.json")?.run()
}

#[test]
fn e2e_int_01() -> Result<()> {
    E2E::load("e2e_int_01.json")?.run()
}

#[test]
fn e2e_int_02() -> Result<()> {
    E2E::load("e2e_int_02.json")?.run()
}

#[test]
fn e2e_dip_00() -> Result<()> {
    E2E::load("e2e_dip_00.json")?.run()
}

#[test]
fn e2e_dip_01() -> Result<()> {
    E2E::load("e2e_dip_01.json")?.run()
}

#[test]
fn e2e_first_00() -> Result<()> {
    E2E::load("e2e_first_00.json")?.run()
}

#[test]
fn e2e_first_01() -> Result<()> {
    E2E::load("e2e_first_01.json")?.run()
}

#[test]
fn e2e_hash_string_00() -> Result<()> {
    E2E::load("e2e_hash_string_00.json")?.run()
}

#[test]
fn e2e_hash_string_01() -> Result<()> {
    E2E::load("e2e_hash_string_01.json")?.run()
}

#[test]
fn e2e_if_some_00() -> Result<()> {
    E2E::load("e2e_if_some_00.json")?.run()
}

#[test]
fn e2e_if_some_01() -> Result<()> {
    E2E::load("e2e_if_some_01.json")?.run()
}

#[test]
fn e2e_set_car_00() -> Result<()> {
    E2E::load("e2e_set_car_00.json")?.run()
}

#[test]
fn e2e_set_car_01() -> Result<()> {
    E2E::load("e2e_set_car_01.json")?.run()
}

#[test]
fn e2e_set_car_02() -> Result<()> {
    E2E::load("e2e_set_car_02.json")?.run()
}

#[test]
fn e2e_set_cdr_00() -> Result<()> {
    E2E::load("e2e_set_cdr_00.json")?.run()
}

#[test]
fn e2e_set_cdr_01() -> Result<()> {
    E2E::load("e2e_set_cdr_01.json")?.run()
}

#[test]
fn e2e_set_cdr_02() -> Result<()> {
    E2E::load("e2e_set_cdr_02.json")?.run()
}

#[test]
fn e2e_hash_key_00() -> Result<()> {
    E2E::load("e2e_hash_key_00.json")?.run()
}

#[test]
fn e2e_hash_key_01() -> Result<()> {
    E2E::load("e2e_hash_key_01.json")?.run()
}

#[test]
fn e2e_add_timestamp_delta_00() -> Result<()> {
    E2E::load("e2e_add_timestamp_delta_00.json")?.run()
}

#[test]
fn e2e_add_timestamp_delta_01() -> Result<()> {
    E2E::load("e2e_add_timestamp_delta_01.json")?.run()
}

#[test]
fn e2e_add_timestamp_delta_02() -> Result<()> {
    E2E::load("e2e_add_timestamp_delta_02.json")?.run()
}

#[test]
fn e2e_add_delta_timestamp_00() -> Result<()> {
    E2E::load("e2e_add_delta_timestamp_00.json")?.run()
}

#[test]
fn e2e_add_delta_timestamp_01() -> Result<()> {
    E2E::load("e2e_add_delta_timestamp_01.json")?.run()
}

#[test]
fn e2e_add_delta_timestamp_02() -> Result<()> {
    E2E::load("e2e_add_delta_timestamp_02.json")?.run()
}

#[test]
fn e2e_sub_timestamp_delta_00() -> Result<()> {
    E2E::load("e2e_sub_timestamp_delta_00.json")?.run()
}

#[test]
fn e2e_sub_timestamp_delta_01() -> Result<()> {
    E2E::load("e2e_sub_timestamp_delta_01.json")?.run()
}

#[test]
fn e2e_sub_timestamp_delta_02() -> Result<()> {
    E2E::load("e2e_sub_timestamp_delta_02.json")?.run()
}

#[test]
fn e2e_diff_timestamps_00() -> Result<()> {
    E2E::load("e2e_diff_timestamps_00.json")?.run()
}

#[test]
fn e2e_diff_timestamps_01() -> Result<()> {
    E2E::load("e2e_diff_timestamps_01.json")?.run()
}

#[test]
fn e2e_diff_timestamps_02() -> Result<()> {
    E2E::load("e2e_diff_timestamps_02.json")?.run()
}

#[test]
fn e2e_diff_timestamps_03() -> Result<()> {
    E2E::load("e2e_diff_timestamps_03.json")?.run()
}

#[test]
fn e2e_packunpack_rev_00() -> Result<()> {
    E2E::load("e2e_packunpack_rev_00.json")?.run()
}

#[test]
fn e2e_packunpack_rev_01() -> Result<()> {
    E2E::load("e2e_packunpack_rev_01.json")?.run()
}

#[test]
fn e2e_packunpack_rev_cty_00() -> Result<()> {
    E2E::load("e2e_packunpack_rev_cty_00.json")?.run()
}

#[test]
fn e2e_packunpack_rev_cty_01() -> Result<()> {
    E2E::load("e2e_packunpack_rev_cty_01.json")?.run()
}

#[test]
fn e2e_ediv_00() -> Result<()> {
    E2E::load("e2e_ediv_00.json")?.run()
}

#[test]
fn e2e_ediv_01() -> Result<()> {
    E2E::load("e2e_ediv_01.json")?.run()
}

#[test]
fn e2e_ediv_02() -> Result<()> {
    E2E::load("e2e_ediv_02.json")?.run()
}

#[test]
fn e2e_ediv_mutez_00() -> Result<()> {
    E2E::load("e2e_ediv_mutez_00.json")?.run()
}

#[test]
fn e2e_ediv_mutez_01() -> Result<()> {
    E2E::load("e2e_ediv_mutez_01.json")?.run()
}

#[test]
fn e2e_ediv_mutez_02() -> Result<()> {
    E2E::load("e2e_ediv_mutez_02.json")?.run()
}

#[test]
fn e2e_ediv_mutez_03() -> Result<()> {
    E2E::load("e2e_ediv_mutez_03.json")?.run()
}

#[test]
fn e2e_ediv_mutez_04() -> Result<()> {
    E2E::load("e2e_ediv_mutez_04.json")?.run()
}

#[test]
fn e2e_ediv_mutez_05() -> Result<()> {
    E2E::load("e2e_ediv_mutez_05.json")?.run()
}

#[test]
fn e2e_ediv_mutez_06() -> Result<()> {
    E2E::load("e2e_ediv_mutez_06.json")?.run()
}

#[test]
fn e2e_compare_00() -> Result<()> {
    E2E::load("e2e_compare_00.json")?.run()
}

#[test]
fn e2e_comparisons_00() -> Result<()> {
    E2E::load("e2e_comparisons_00.json")?.run()
}

#[test]
fn e2e_address_00() -> Result<()> {
    E2E::load("e2e_address_00.json")?.run()
}

#[test]
fn e2e_contract_00() -> Result<()> {
    E2E::load("e2e_contract_00.json")?.run()
}

#[test]
fn e2e_mul_00() -> Result<()> {
    E2E::load("e2e_mul_00.json")?.run()
}

#[test]
fn e2e_neg_00() -> Result<()> {
    E2E::load("e2e_neg_00.json")?.run()
}

#[test]
fn e2e_neg_01() -> Result<()> {
    E2E::load("e2e_neg_01.json")?.run()
}

#[test]
fn e2e_neg_02() -> Result<()> {
    E2E::load("e2e_neg_02.json")?.run()
}

#[test]
fn e2e_neg_03() -> Result<()> {
    E2E::load("e2e_neg_03.json")?.run()
}

#[test]
fn e2e_neg_04() -> Result<()> {
    E2E::load("e2e_neg_04.json")?.run()
}

#[test]
fn e2e_dign_00() -> Result<()> {
    E2E::load("e2e_dign_00.json")?.run()
}

#[test]
fn e2e_dugn_00() -> Result<()> {
    E2E::load("e2e_dugn_00.json")?.run()
}

#[test]
fn e2e_dropn_00() -> Result<()> {
    E2E::load("e2e_dropn_00.json")?.run()
}

#[test]
fn e2e_dipn_00() -> Result<()> {
    E2E::load("e2e_dipn_00.json")?.run()
}

#[test]
fn e2e_dig_eq_00() -> Result<()> {
    E2E::load("e2e_dig_eq_00.json")?.run()
}

#[test]
fn e2e_dig_eq_01() -> Result<()> {
    E2E::load("e2e_dig_eq_01.json")?.run()
}

#[test]
fn e2e_pexec_00() -> Result<()> {
    E2E::load("e2e_pexec_00.json")?.run()
}

#[test]
fn e2e_pexec_2_00() -> Result<()> {
    E2E::load("e2e_pexec_2_00.json")?.run()
}

#[test]
fn e2e_chain_id_store_00() -> Result<()> {
    E2E::load("e2e_chain_id_store_00.json")?.run()
}

#[test]
fn e2e_chain_id_store_01() -> Result<()> {
    E2E::load("e2e_chain_id_store_01.json")?.run()
}

#[test]
fn e2e_chain_id_store_02() -> Result<()> {
    E2E::load("e2e_chain_id_store_02.json")?.run()
}

#[test]
fn e2e_self_with_entrypoint_00() -> Result<()> {
    E2E::load("e2e_self_with_entrypoint_00.json")?.run()
}

#[test]
fn e2e_self_with_default_entrypoint_00() -> Result<()> {
    E2E::load("e2e_self_with_default_entrypoint_00.json")?.run()
}

#[test]
fn e2e_self_address_00() -> Result<()> {
    E2E::load("e2e_self_address_00.json")?.run()
}

#[test]
fn e2e_unpair_00() -> Result<()> {
    E2E::load("e2e_unpair_00.json")?.run()
}

#[test]
fn e2e_comb_00() -> Result<()> {
    E2E::load("e2e_comb_00.json")?.run()
}

#[test]
fn e2e_uncomb_00() -> Result<()> {
    E2E::load("e2e_uncomb_00.json")?.run()
}

#[test]
fn e2e_comb_get_00() -> Result<()> {
    E2E::load("e2e_comb_get_00.json")?.run()
}

#[test]
fn e2e_comb_set_00() -> Result<()> {
    E2E::load("e2e_comb_set_00.json")?.run()
}

#[test]
fn e2e_comb_set_2_00() -> Result<()> {
    E2E::load("e2e_comb_set_2_00.json")?.run()
}

#[test]
fn e2e_dup_n_00() -> Result<()> {
    E2E::load("e2e_dup_n_00.json")?.run()
}

#[test]
fn e2e_map_map_sideeffect_00() -> Result<()> {
    E2E::load("e2e_map_map_sideeffect_00.json")?.run()
}

#[test]
fn e2e_map_map_sideeffect_01() -> Result<()> {
    E2E::load("e2e_map_map_sideeffect_01.json")?.run()
}

#[test]
fn e2e_map_map_sideeffect_02() -> Result<()> {
    E2E::load("e2e_map_map_sideeffect_02.json")?.run()
}

#[test]
fn e2e_packunpack_00() -> Result<()> {
    E2E::load("e2e_packunpack_00.json")?.run()
}

#[test]
fn e2e_check_signature_00() -> Result<()> {
    E2E::load("e2e_check_signature_00.json")?.run()
}
