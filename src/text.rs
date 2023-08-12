// This file contains big const string-slices to make the 'main.rs' file less complex
// ==> goal is to make 'main.rs' as simple as possible

pub const T_PORYDEL_HELP: &str = "                                                __       __       __      
                ____   ____   _____ __  __ ____/ /___   / /___   / /_ ___ 
               / __ \\ / __ \\ / ___// / / // __  // _ \\ / // _ \\ / __// _ \\
              / /_/ // /_/ // /   / /_/ // /_/ //  __// //  __// /_ /  __/
             / .___/ \\____//_/    \\__, / \\__,_/ \\___//_/ \\___/ \\__/ \\___/ 
            /_/                  /____/     
    Porydelete by Voluptua. Repository link: 'https://github.com/Voluptua/porydelete'


[USAGE]:

    [OPTIONS] [ARGUMENTS]

[OPTIONS]:

    --help          gives this output of information about this tool.
    --ma            option to delete map-attributes in 'map.json'.
    --m             option to delete all available maps given via arguments.

[ARGUMENTS]:

    [OPTIONS] --help
    
";


pub const T_MA_HELP: &str = "
'--ma' is the option to delete map attributes in all maps (filtering will be added asap).
It needs one of the 5 arguments (which are the available attributes to be deleted) which then will be removed. If no 
argument is given, Porydelete won't delete anything.

[USAGE]:

    [OPTIONS] [ARGUMENTS]
                                      
[ARGUMENTS]:
     
    'connections'
    'object_events'                   
    'warp_events'             
    'bg_events'
    'coord_events'
                  
";