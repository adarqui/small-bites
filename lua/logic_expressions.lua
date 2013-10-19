-- expressions & logic

function relational_operators()
	a = 1 b = 2 c = 3;
	print("relational_operators");
	--[[
		~= -> negation operator
	]]--
	
	print("\t",a==b,a~=b);
end


function logical_operators()
	print("logical_operators:")
	print("\t",1 and 2)
	print("\t",1 or 2)
	print("\t",not nil)
	print("\t",not false)
	print("\t",not 0)
	print("\t",not true)
	print("\t",not not not not not true)

	x = x or 5;
	print("\t",x)

end



function operator_precedence()
--[[
^not - (unary)
* /
+ -
..
< > <=  >= ~= ==
and
or
]]--
	print("operator_precedence:")
	print("\t",1 ^ 5 * 2 + 100 .. "hello")
end


function if_then_else()
	print("if_then_else:")
	if true then
		print("\t","true")
	elseif false then
		print("\t","false")
	else
		print("\t","false")
	end
end

function main()
	relational_operators()
	logical_operators()
	if_then_else()
end


main()
