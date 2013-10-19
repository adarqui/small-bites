--[[
	accessing argv/argc/envp

	argc = #arg
	argv = arg
	envp = _G
]]--


function main()
	print("main:")
	print("\t","argc="..(#arg),"argv[0]="..arg[0])
	print("\t","env",_G)
	print("\t","arg",arg)

	print ("envp:")
	for i,env in pairs(_G)
		do print("\t",i,env)
	end

	print("argv:")
	for i,arg in pairs(arg)
		do print("\t",i,arg)
	end
end

main()
