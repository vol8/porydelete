// This file contains big const string-slices to make the 'main.rs' file less complex
// ==> goal is to make 'main.rs' as simple as possible

pub const T_NONE_HELP: &str = "
No help text for this [COMMAND].
";

pub const T_PORYDEL_HELP: &str =
    "                                                __       __       __      
                ____   ____   _____ __  __ ____/ /___   / /___   / /_ ___ 
               / __ \\ / __ \\ / ___// / / // __  // _ \\ / // _ \\ / __// _ \\
              / /_/ // /_/ // /   / /_/ // /_/ //  __// //  __// /_ /  __/
             / .___/ \\____//_/    \\__, / \\__,_/ \\___//_/ \\___/ \\__/ \\___/ 
            /_/                  /____/     
    Porydelete by Voluptua. Repository link: 'https://github.com/Voluptua/porydelete'


[USAGE]:

    [COMMANDS] [ARGUMENTS]

[COMMANDS]:

    --help          gives this output of information about this tool.
    --ma            command to delete map-attributes in 'map.json'.
    --m             command to delete maps given via arguments.
    --ts            command to delete map-tilesets given via arguments.
    --s             command to delete scripts given via arguments.
    --pkmn          command to delete a specific Pokemon given via arguments.
    --item          command to delete a specific Item given via arguments.

    --l-Us          command to list all detected unused scripts.
    --l-s           command to list all available scripts.

[ARGUMENTS]:

    [COMMANDS] --help
    
";

pub const T_MA_HELP: &str = "
'--ma' is the command to delete map attributes in all maps.
It needs one of the 5 arguments (which are the available attributes to be deleted) which then will be removed. 
You can also use '-a' for all attributes. With '--ma' you can also '--filter' or '--defilter' maps so that they won't get edited.
If no argument is given, Porydelete won't delete anything.

[USAGE]:

    [COMMANDS] [ARGUMENTS]

[COMMANDS]:
    '--filter'      filters specific maps given via arguments  
    '--defilter'    moves the filtered maps given via arguments back

[ARGUMENTS]:
     
    'connections'
    'object_events'                   
    'warp_events'             
    'bg_events'
    'coord_events'

    <mapname>       these are the maps filtered or defiltered when using '--filter'/'--defilter'

    '-a'            deletes for every other map-attribute
                  
";

pub const T_M_HELP: &str = "
'--m' is the command to delete maps created with Porymap. The only argument you need is the name of the map.
So for example: './porydelete --m LittlerootTown'. Important: The map name of each map is the one by the map-
selector on the left of Porymap.

[USAGE]:

    [COMMANDS] [ARGUMENTS]

[ARGUMENTS]:

    <mapname>

";

pub const T_TS_HELP: &str = "
'--ts' is the command to delete map-tilesets created with Porymap. The only argument you need is the name of
the tileset. So for example: './porydelete --ts gTileset_Fallarbor'. Important: The name of the map-tileset
is the one you see in the Tileset selection part of Porymap.

[USAGE]:

    [COMMANDS] [ARGUMENTS]

[ARGUMENTS]:

    <tilesetname>

";
