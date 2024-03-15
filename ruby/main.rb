require 'ffi'

module Adder
  extend FFI::Library
  ffi_lib '../adder/target/release/libadder.so'
  attach_function :add, [:uint32, :uint32], :uint32
  attach_function :process_request, [:string], :strptr
  attach_function :deallocate_ptr, [:pointer], :void
end

result_str, result_ptr = Adder.process_request("a cup of coffee ☕")
puts result_str
Adder.deallocate_ptr(result_ptr)
