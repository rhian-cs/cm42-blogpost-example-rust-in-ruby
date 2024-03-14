require 'ffi'

module Adder
  extend FFI::Library
  ffi_lib '../adder/target/release/libadder.so'
  attach_function :add, [:uint32, :uint32], :uint32
end

puts 1
puts Adder.add(5, 3)
