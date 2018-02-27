// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


pub const pmempool_check_msg_type_PMEMPOOL_CHECK_MSG_TYPE_ERROR: pmempool_check_msg_type = 1;
pub const pmempool_check_msg_type_PMEMPOOL_CHECK_MSG_TYPE_INFO: pmempool_check_msg_type = 0;
pub const pmempool_check_msg_type_PMEMPOOL_CHECK_MSG_TYPE_QUESTION: pmempool_check_msg_type = 2;
pub const pmempool_check_result_PMEMPOOL_CHECK_RESULT_CANNOT_REPAIR: pmempool_check_result = 3;
pub const pmempool_check_result_PMEMPOOL_CHECK_RESULT_CONSISTENT: pmempool_check_result = 0;
pub const pmempool_check_result_PMEMPOOL_CHECK_RESULT_ERROR: pmempool_check_result = 4;
pub const pmempool_check_result_PMEMPOOL_CHECK_RESULT_NOT_CONSISTENT: pmempool_check_result = 1;
pub const pmempool_check_result_PMEMPOOL_CHECK_RESULT_REPAIRED: pmempool_check_result = 2;
pub const pmempool_pool_type_PMEMPOOL_POOL_TYPE_BLK: pmempool_pool_type = 2;
pub const pmempool_pool_type_PMEMPOOL_POOL_TYPE_BTT: pmempool_pool_type = 4;
pub const pmempool_pool_type_PMEMPOOL_POOL_TYPE_CTO: pmempool_pool_type = 5;
pub const pmempool_pool_type_PMEMPOOL_POOL_TYPE_DETECT: pmempool_pool_type = 0;
pub const pmempool_pool_type_PMEMPOOL_POOL_TYPE_LOG: pmempool_pool_type = 1;
pub const pmempool_pool_type_PMEMPOOL_POOL_TYPE_OBJ: pmempool_pool_type = 3;
pub const pobj_action_type_POBJ_ACTION_TYPE_HEAP: pobj_action_type = 0;
pub const pobj_action_type_POBJ_ACTION_TYPE_MEM: pobj_action_type = 1;
pub const pobj_action_type_POBJ_MAX_ACTION_TYPE: pobj_action_type = 2;
pub const pobj_header_type_MAX_POBJ_HEADER_TYPES: pobj_header_type = 3;
pub const pobj_header_type_POBJ_HEADER_COMPACT: pobj_header_type = 1;
pub const pobj_header_type_POBJ_HEADER_LEGACY: pobj_header_type = 0;
pub const pobj_header_type_POBJ_HEADER_NONE: pobj_header_type = 2;
pub const pobj_tx_lock_TX_LOCK_MUTEX: pobj_tx_lock = 1;
pub const pobj_tx_lock_TX_LOCK_NONE: pobj_tx_lock = 0;
pub const pobj_tx_lock_TX_LOCK_RWLOCK: pobj_tx_lock = 2;
pub const pobj_tx_param_TX_PARAM_CB: pobj_tx_param = 3;
pub const pobj_tx_param_TX_PARAM_MUTEX: pobj_tx_param = 1;
pub const pobj_tx_param_TX_PARAM_NONE: pobj_tx_param = 0;
pub const pobj_tx_param_TX_PARAM_RWLOCK: pobj_tx_param = 2;
pub const pobj_tx_stage_MAX_TX_STAGE: pobj_tx_stage = 5;
pub const pobj_tx_stage_TX_STAGE_FINALLY: pobj_tx_stage = 4;
pub const pobj_tx_stage_TX_STAGE_NONE: pobj_tx_stage = 0;
pub const pobj_tx_stage_TX_STAGE_ONABORT: pobj_tx_stage = 3;
pub const pobj_tx_stage_TX_STAGE_ONCOMMIT: pobj_tx_stage = 2;
pub const pobj_tx_stage_TX_STAGE_WORK: pobj_tx_stage = 1;
