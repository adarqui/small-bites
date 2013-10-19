-- string operations
--
--

function gsub(a,b,c)
	d = string.gsub(a,b,c)
	return d
end


function string_concatenation()
	print("string_concatenation:")
	print("\t","hello" .. " world" .. 10 .. 00 .. 00)
end


function main()
	string_concatenation()
	e = gsub("hello world", "world", "derp")
	print("gsub:\n","\t",e)
end


main()
