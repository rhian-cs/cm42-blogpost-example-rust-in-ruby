require 'ffi'

module Adder
  extend FFI::Library
  ffi_lib '../adder/target/release/libadder.so'
  attach_function :add, [:uint, :uint], :uint
end

puts Adder.add(5, 3)
