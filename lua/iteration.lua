-- iteration


function iteration_over_array()

	print ("iteration_over_array:")

	a = {}
	a[0] = 0
	a[1] = 1
	a[2] = 2
	a[3] = 3
	a["a"] = "a"
	a["b"] = "b"

	print("\tlength",#a)

	for i,num in ipairs(a) do
		print("\t"..num)
	end
end


function numeric_for_loop()
	-- the 'numeric' for syntax provides the comparison/increment for you
	-- never change the value of the control variable
	print("numeric_for_loop:")
	local i
	for i=0,10,1 do
		print("\t",i)
	end
end


function generic_for_loop()
	print("generic_for_loop:")
	local days = { "sun","mon","tues","wed","thurs","fri","sat" }
	for i,j in ipairs(days) do
		print("\t",i,j)
	end
end


function while_loop()
	local i = 1
	print("while_loop:")
	while i < 10 do
		print("\t",i)
		i=i+1
	end
end


function repeat_loop()
	print("repeat_loop:")
	local i = 1
	repeat
		print("\t","hello!")
		i=i+1
	until i == 10
end


function break_loop()
	print("break_loop:")
	local i = 0
	while true do
		print("\t","yes")
		i = i + 1
		if i == 10 then break end
	end
end
function main()
	numeric_for_loop()
	generic_for_loop()
	while_loop()
	repeat_loop()
	iteration_over_array()
	break_loop()
end

main()
