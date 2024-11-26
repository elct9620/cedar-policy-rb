# frozen_string_literal: true

RSpec.describe CedarPolicy::Request do
  let(:principal) { CedarPolicy::EntityUid.new("User", "1") }
  let(:action) { CedarPolicy::EntityUid.new("Action", "view") }
  let(:resource) { CedarPolicy::EntityUid.new("Image", "1") }

  subject(:request) { CedarPolicy::Request.new(principal, action, resource, {}) }

  describe "#principal" do
    subject { request.principal }

    it { is_expected.to eq(CedarPolicy::EntityUid.new("User", "1")) }

    describe "with empty principal" do
      let(:principal) { nil }

      it {
        expect do
          request
        end.to raise_error(ArgumentError).with_message("no implicit conversion of NilClass into EntityUid")
      }
    end
  end

  describe "#action" do
    subject { request.action }

    it { is_expected.to eq(CedarPolicy::EntityUid.new("Action", "view")) }

    describe "with empty action" do
      let(:action) { nil }

      it {
        expect do
          request
        end.to raise_error(ArgumentError).with_message("no implicit conversion of NilClass into EntityUid")
      }
    end
  end

  describe "#resource" do
    subject { request.resource }

    it { is_expected.to eq(CedarPolicy::EntityUid.new("Image", 1)) }

    describe "with empty resource" do
      let(:resource) { nil }

      it {
        expect do
          request
        end.to raise_error(ArgumentError).with_message("no implicit conversion of NilClass into EntityUid")
      }
    end
  end

  context("when schema dictates principals for actions") do
    let(:schema) do
      CedarPolicy::Schema.new(
        <<~SCHEMA
          entity User, Admin, Image;

          action view appliesTo {
              principal: [User],
              resource: [Image]
          };

          action delete appliesTo {
              principal: [Admin],
              resource: [Image]
          };
        SCHEMA
      )
    end

    subject(:request) { CedarPolicy::Request.new(principal, action, resource, {}, schema: schema) }

    describe ".new raises error when principal is not valid for action" do
      let(:principal) { CedarPolicy::EntityUid.new("User", "1") }
      let(:action) { CedarPolicy::EntityUid.new("Action", "delete") }

      it { expect { request }.to(raise_error(CedarPolicy::RequestValidationError)) }
    end

    describe ".new returns request when principal is valid for action" do
      let(:principal) { CedarPolicy::EntityUid.new("Admin", "1") }
      let(:action) { CedarPolicy::EntityUid.new("Action", "delete") }

      it { is_expected.to(have_attributes(principal: CedarPolicy::EntityUid.new("Admin", "1"))) }
    end
  end
end
