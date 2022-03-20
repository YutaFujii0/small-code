require 'forwardable'

class ProductType
    attr_reader :recognize_revenue_rule

    def initialize(recognize_revenue_rule)
        @recognize_revenue_rule = recognize_revenue_rule
    end

    def recognize_revenue(contract)
        revenue_recognitions = @recognize_revenue_rule.map do |rule|
            {
                date: (contract.sign_on + rule[:lag]).strftime("%Y-%m-%d"),
                amount: contract.price * rule[:multiple] / rule[:denominator]
            }
        end

        remainder = contract.price - revenue_recognitions.map { |item| item[:amount] }.sum()

        remainder.times do |i|
            revenue_recognitions[i][:amount] += 1
        end
        revenue_recognitions
    end
end

class WordProcessor < ProductType; end
class Spreadsheet < ProductType; end

class Product
    attr_reader :name, :type, :price

    def initialize(name, type, price)
        @name = name
        @type = type
        @price = price
    end
end

class Contract
    extend Forwardable
    attr_reader :product, :sign_on, :revenue_recognitions
    def_delegators :@product, :price, :type

    def initialize(product, sign_on)
        @product = product
        @sign_on = sign_on
        recognize_revenue
    end

    private
        def recognize_revenue
            @revenue_recognitions = type.recognize_revenue(self)
        end
end

def sign(product, sign_on)
    contract = Contract.new(product, sign_on)
    contract.revenue_recognitions
end
