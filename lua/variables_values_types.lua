-- variables, values, and data types

-- local variables
function local_variables()
	print("local_variables:")
	a = 1; b = a*2;
	c = 5 d = 8;
	e = 10
	f = e
end


function tables()
	-- tables = associative arrays
	print("tables:")
	a = {} -- creates a table
	a["b"] = 1
	a[1] = "one"
	a["1"] = "two"
	a.b = 5
	print("\t",a["b"], a[1], a["1"])
	a.b = nil
	print("\t",a["b"])
end

-- assign global variables
global_var = 1

function global_test()
	print("global_test:")
	print("\t",global_var)
	-- de-assign global variables
	global_var = nil
end


function print_types()
	-- finding the type of a variable/value -> type()
	print("print_types")
	print("\t",type("string"))
	print("\t",type(3.14))
	print("\t",type(print))
	print("\t",type(true))
	print("\t",type(nil))
end



function adding_string_and_number()
	-- lua will convert the string to a number if possible
	print("adding_string_and_number:")
	if unexpected_condition then error() end
	print("\t","99"+1)
	print("\t","hello"+1)
end



function type_conversion()
	print("type_conversion:")
	a = 10
	print("\t",tostring(a))
	b = "10"
	print("\t",tonumber(b))
end


function swapping_values()
	print("swapping_values:")
	a = 5 b = 10
	a, b = b, a
	print("\t",a,b)
end



function silently_discarded_assignment()
	print("silently_discarded_assignment:")
	-- c is discarded
	a, b, c = 0, 1
	print("\t",a,b,c)
end


function local_variables_specific_to_chunks()
	-- using the local keyword
	local i = 10
	print("local_variables_specific_to_chunks:")
	while i > 1 do
		local j = i * i
		print("\t", j)
		i = i - 1
	end
end

function main()
	-- in control structures, nil is false
	if(nil) then
		print "nil"
	end

	local_variables()
	tables()
	print_types()
	global_test()
	pcall(adding_string_and_number)
	type_conversion()
	swapping_values()
	silently_discarded_assignment()
	local_variables_specific_to_chunks()
end



main()
