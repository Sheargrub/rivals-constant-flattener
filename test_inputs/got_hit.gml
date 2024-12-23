/*
Multiline comment
*/

//#RCFBEGINIGNORE   
devious_behavior();
//#RCFENDIGNORE

sound_stop(call_sfx_instance);
escapee_string = "\"waow\"";

// Heart Barrier
heart_barrier_endangered = 1;
heart_barrier_timer = 0;

// Ignition Tank
do_ignite_hbox = 0;

// Fireman's Boots "of doom1!!"
if (fireboots_lockout < FIREBOOTS_HIT_LOCKOUT) fireboots_lockout = FIREBOOTS_HIT_LOCKOUT;

// Brilliant Behemoth
do_behemoth_hbox = 0;

// 57-Leaf Clover
clover_test = clover_active;

//Death Message (N/A Compat)
is_na = (hit_player_obj.url == 2229832619);