-- functions

function some_function(a,b,c)
	print("some_function:")
	print("\t",a,b,c)
end

function multiple_return_values(a,b,c)
	return a,b,c
end


function max(a)
	-- i liked this function - from pil
	local mi = 1
	local m = a[mi]
	for i,val in ipairs(a) do
		if val > m then
			mi = i
			m = val
		end
	end
	return m,mi
end



function variable_number_of_arguments(...)
	print("variable_number_of_arguments:", #arg,arg,unpack(arg))
	for i,j in ipairs(arg) do
		print("\t",i,j)
	end

	print("\tUsing select")
	for i=1, select('#',...) do
		print("\t",select(i,...))
	end

	print("\tUsing table.pack")
	t = table.pack(...)
	for i=1, t.n do
		print("\t\t",t[i])
	end
end



function named_arguments(arg)
	print("named_arguments:")
	print("\t",arg.name,arg.hi)
end

function main()
	some_function()
	a,b,c = multiple_return_values(1,2,3)
	print("multiple_return_values:\n","\t",a,b,c)
	print("max:\n","\t",max({1,2,3,4,9,4,3,5,54,2,1,4,5}))
	variable_number_of_arguments(1,2,3)
	variable_number_of_arguments(0,"a",{1,2},3,4,"5",6.6)
	-- when providing arguments by name, use { } instead of ()
	named_arguments{name="lua", hi="hello"}
end


main()
