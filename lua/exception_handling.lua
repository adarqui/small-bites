-- exception handling

function error_thingy()
--	if unexpected_condition then error() end
	print(x[1][i])
end

err = pcall(error_thingy)
if(err) then
	-- returns true if no error
	print("no error")
else
	-- otherwise, error
	print("error",err)
end
