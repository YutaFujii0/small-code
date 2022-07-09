class Employee
    def initialize(empId, empName)
        @empId = empId
        @empName = empName
    end

    def get_empId
        return @empId
    end

    def get_empName
        return @empName
    end
end

emp = Employee.new(1, "Mugambo")
seredObject = Marshal.dump(emp)
# puts seredObject


de = Marshal.load(seredObject)
# puts de
