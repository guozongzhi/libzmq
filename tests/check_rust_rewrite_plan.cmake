# SPDX-License-Identifier: MPL-2.0

if(NOT DEFINED SOURCE_DIR)
  message(FATAL_ERROR "SOURCE_DIR must point at the libzmq source tree")
endif()

set(PLAN_FILE "${SOURCE_DIR}/doc/rust_rewrite_plan.adoc")
set(PUBLIC_HEADER "${SOURCE_DIR}/include/zmq.h")
set(C_API_FILE "${SOURCE_DIR}/src/zmq.cpp")
set(UTILS_API_FILE "${SOURCE_DIR}/src/zmq_utils.cpp")

foreach(required_file ${PLAN_FILE} ${PUBLIC_HEADER} ${C_API_FILE} ${UTILS_API_FILE})
  if(NOT EXISTS "${required_file}")
    message(FATAL_ERROR "Required file does not exist: ${required_file}")
  endif()
endforeach()

file(READ "${PLAN_FILE}" PLAN_CONTENT)
file(READ "${PUBLIC_HEADER}" HEADER_CONTENT)
file(READ "${C_API_FILE}" C_API_CONTENT)
file(READ "${UTILS_API_FILE}" UTILS_API_CONTENT)
set(C_API_CONTENT "${C_API_CONTENT}${UTILS_API_CONTENT}")

set(REQUIRED_PLAN_TERMS
  "== 目标"
  "== 非目标"
  "== 总体框架"
  "== 兼容性契约"
  "== Rust workspace 布局"
  "== FFI 和 unsafe 策略"
  "== 子模块迁移阶段"
  "=== 阶段 0：整体框架和编译环境"
  "=== 阶段 0.5：Atomic counter 工具 API"
  "=== 阶段 1：消息模块"
  "=== 阶段 2：Context 生命周期"
  "=== 阶段 3：Socket base 和 inproc transport"
  "=== 阶段 4：Polling"
  "== 构建集成计划"
  "== 测试策略"
  "== 回滚策略"
  "include/zmq.h"
  "src/zmq.cpp"
  "ENABLE_RUST_REWRITE"
  "RUST_REWRITE_STRICT"
  "zmq_rs_version_probe")

foreach(term ${REQUIRED_PLAN_TERMS})
  string(FIND "${PLAN_CONTENT}" "${term}" term_index)
  if(term_index EQUAL -1)
    message(FATAL_ERROR "Rust rewrite plan is missing required term: ${term}")
  endif()
endforeach()

set(REQUIRED_PUBLIC_APIS
  "zmq_msg_init"
  "zmq_msg_close"
  "zmq_ctx_new"
  "zmq_ctx_term"
  "zmq_socket"
  "zmq_close"
  "zmq_send"
  "zmq_recv"
  "zmq_poll"
  "zmq_poller_new"
  "zmq_atomic_counter_new"
  "zmq_atomic_counter_destroy")

foreach(api ${REQUIRED_PUBLIC_APIS})
  string(FIND "${PLAN_CONTENT}" "${api}" plan_api_index)
  if(plan_api_index EQUAL -1)
    message(FATAL_ERROR "Rust rewrite plan does not mention required API: ${api}")
  endif()

  string(FIND "${HEADER_CONTENT}" "${api}" header_api_index)
  if(header_api_index EQUAL -1)
    message(FATAL_ERROR "Public header does not declare expected API: ${api}")
  endif()
endforeach()

set(REQUIRED_C_API_ENTRYPOINTS
  "zmq_msg_init"
  "zmq_ctx_new"
  "zmq_ctx_term"
  "zmq_socket"
  "zmq_send"
  "zmq_recv"
  "zmq_poll"
  "zmq_poller_new"
  "zmq_atomic_counter_new"
  "zmq_atomic_counter_destroy")

foreach(api ${REQUIRED_C_API_ENTRYPOINTS})
  string(FIND "${C_API_CONTENT}" "${api}" c_api_index)
  if(c_api_index EQUAL -1)
    message(FATAL_ERROR "C API shim does not contain expected entry point: ${api}")
  endif()
endforeach()

set(REQUIRED_RUST_FILES
  "rust/.gitignore"
  "rust/Cargo.toml"
  "rust/Cargo.lock"
  "rust/libzmq-core/Cargo.toml"
  "rust/libzmq-core/src/lib.rs"
  "rust/libzmq-core/src/atomic_counter.rs"
  "rust/libzmq-core/src/message.rs"
  "rust/libzmq-core/src/context.rs"
  "rust/libzmq-ffi/Cargo.toml"
  "rust/libzmq-ffi/src/lib.rs"
  "rust/libzmq-protocol/Cargo.toml"
  "rust/libzmq-transport/Cargo.toml")

foreach(rust_file ${REQUIRED_RUST_FILES})
  if(NOT EXISTS "${SOURCE_DIR}/${rust_file}")
    message(FATAL_ERROR "Rust rewrite scaffold is missing required file: ${rust_file}")
  endif()
endforeach()

message(STATUS "Rust rewrite plan invariants validated")
