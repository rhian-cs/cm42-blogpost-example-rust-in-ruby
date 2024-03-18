require 'ffi'
require 'json'

module Adder
  extend FFI::Library
  ffi_lib ENV.fetch('LIBADDER_SO_PATH') { '../adder/target/release/libadder.so' }

  attach_function :add, [:uint32, :uint32], :uint32
  attach_function :process_request, [:string], :strptr
  attach_function :deallocate_ptr, [:pointer], :void
end

request = {
  name: 'John Doe',
  age: 18
}

result_str, result_ptr = Adder.process_request(request.to_json)
response = JSON.parse(result_str)
Adder.deallocate_ptr(result_ptr)

puts response
