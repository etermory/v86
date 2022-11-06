#[allow(non_camel_case_types)]
pub enum stat {
    COMPILE,
    COMPILE_SKIPPED_NO_NEW_ENTRY_POINTS,
    COMPILE_WRONG_ADDRESS_SPACE,
    COMPILE_CUT_OFF_AT_END_OF_PAGE,
    COMPILE_WITH_LOOP_SAFETY,
    COMPILE_PAGE,
    COMPILE_BASIC_BLOCK,
    COMPILE_DUPLICATED_BASIC_BLOCK,
    COMPILE_WASM_BLOCK,
    COMPILE_WASM_LOOP,
    COMPILE_DISPATCHER,
    COMPILE_ENTRY_POINT,
    COMPILE_WASM_TOTAL_BYTES,

    RUN_INTERPRETED,
    RUN_INTERPRETED_PAGE_HAS_CODE,
    RUN_INTERPRETED_PAGE_HAS_ENTRY_AFTER_PAGE_WALK,
    RUN_INTERPRETED_NEAR_END_OF_PAGE,
    RUN_INTERPRETED_DIFFERENT_STATE,
    RUN_INTERPRETED_DIFFERENT_STATE_CPL3,
    RUN_INTERPRETED_DIFFERENT_STATE_FLAT,
    RUN_INTERPRETED_DIFFERENT_STATE_IS32,
    RUN_INTERPRETED_DIFFERENT_STATE_SS32,
    RUN_INTERPRETED_MISSED_COMPILED_ENTRY_RUN_INTERPRETED,
    RUN_INTERPRETED_STEPS,

    RUN_FROM_CACHE,
    RUN_FROM_CACHE_STEPS,

    DIRECT_EXIT,
    INDIRECT_JUMP,
    INDIRECT_JUMP_NO_ENTRY,
    NORMAL_PAGE_CHANGE,
    NORMAL_FALLTHRU,
    NORMAL_FALLTHRU_WITH_TARGET_BLOCK,
    NORMAL_BRANCH,
    NORMAL_BRANCH_WITH_TARGET_BLOCK,
    CONDITIONAL_JUMP,
    CONDITIONAL_JUMP_PAGE_CHANGE,
    CONDITIONAL_JUMP_EXIT,
    CONDITIONAL_JUMP_FALLTHRU,
    CONDITIONAL_JUMP_FALLTHRU_WITH_TARGET_BLOCK,
    CONDITIONAL_JUMP_BRANCH,
    CONDITIONAL_JUMP_BRANCH_WITH_TARGET_BLOCK,
    DISPATCHER_SMALL,
    DISPATCHER_LARGE,
    LOOP,

    LOOP_SAFETY,

    CONDITION_OPTIMISED,
    CONDITION_UNOPTIMISED,
    CONDITION_UNOPTIMISED_PF,
    CONDITION_UNOPTIMISED_UNHANDLED_L,
    CONDITION_UNOPTIMISED_UNHANDLED_LE,

    FAILED_PAGE_CHANGE,

    SAFE_READ_FAST,
    SAFE_READ_SLOW_PAGE_CROSSED,
    SAFE_READ_SLOW_NOT_VALID,
    SAFE_READ_SLOW_NOT_USER,
    SAFE_READ_SLOW_IN_MAPPED_RANGE,

    SAFE_WRITE_FAST,
    SAFE_WRITE_SLOW_PAGE_CROSSED,
    SAFE_WRITE_SLOW_NOT_VALID,
    SAFE_WRITE_SLOW_NOT_USER,
    SAFE_WRITE_SLOW_IN_MAPPED_RANGE,
    SAFE_WRITE_SLOW_READ_ONLY,
    SAFE_WRITE_SLOW_HAS_CODE,

    SAFE_READ_WRITE_FAST,
    SAFE_READ_WRITE_SLOW_PAGE_CROSSED,
    SAFE_READ_WRITE_SLOW_NOT_VALID,
    SAFE_READ_WRITE_SLOW_NOT_USER,
    SAFE_READ_WRITE_SLOW_IN_MAPPED_RANGE,
    SAFE_READ_WRITE_SLOW_READ_ONLY,
    SAFE_READ_WRITE_SLOW_HAS_CODE,

    PAGE_FAULT,
    TLB_MISS,

    DO_MANY_CYCLES,
    CYCLE_INTERNAL,

    INVALIDATE_ALL_MODULES_NO_FREE_WASM_INDICES,
    INVALIDATE_MODULE_WRITTEN_WHILE_COMPILED,
    INVALIDATE_MODULE_UNUSED_AFTER_OVERWRITE,
    INVALIDATE_MODULE_DIRTY_PAGE,

    INVALIDATE_PAGE_HAD_CODE,
    INVALIDATE_PAGE_HAD_ENTRY_POINTS,
    DIRTY_PAGE_DID_NOT_HAVE_CODE,

    RUN_FROM_CACHE_EXIT_SAME_PAGE,
    RUN_FROM_CACHE_EXIT_NEAR_END_OF_PAGE,
    RUN_FROM_CACHE_EXIT_DIFFERENT_PAGE,

    CLEAR_TLB,
    FULL_CLEAR_TLB,
    TLB_FULL,
    TLB_GLOBAL_FULL,

    MODRM_SIMPLE_REG,
    MODRM_SIMPLE_REG_WITH_OFFSET,
    MODRM_SIMPLE_CONST_OFFSET,
    MODRM_COMPLEX,

    SEG_OFFSET_OPTIMISED,
    SEG_OFFSET_NOT_OPTIMISED,
}

#[allow(non_upper_case_globals)]
pub static mut stat_array: [u64; 500] = [0; 500];

pub fn stat_increment(stat: stat) { stat_increment_by(stat, 1); }

pub fn stat_increment_by(stat: stat, by: u64) {
    if cfg!(feature = "profiler") {
        unsafe { stat_array[stat as usize] += by }
    }
}

#[no_mangle]
pub fn profiler_init() {
    unsafe {
        for x in stat_array.iter_mut() {
            *x = 0
        }
    }
}

#[no_mangle]
pub fn profiler_stat_get(stat: stat) -> f64 {
    if cfg!(feature = "profiler") {
        unsafe { stat_array[stat as usize] as f64 }
    }
    else {
        0.0
    }
}

#[no_mangle]
pub fn profiler_is_enabled() -> bool { cfg!(feature = "profiler") }
