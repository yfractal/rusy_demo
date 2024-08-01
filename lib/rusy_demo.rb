# frozen_string_literal: true

require_relative 'rusy_demo/version'
require 'rutie'

module RusyDemo
  class Error < StandardError; end
  Rutie.new(:rusy_demo).init 'Init_rusy_demo', __dir__
end
