//                                  debug                                     //
init_complete = false; // used to ensure that draw scripts don't flood the error log if initialization is interrupted
is_playtest = (object_index == oTestPlayer);

debug_display_opened = 0;
debug_display_count = 23;
debug_display_index = 0;
debug_display_scrolltimer = 0;
debug_display_type = 0;
debug_display_typerange = 3;

rainfont = font_get("_rfont");
rainfontbig = font_get("_rfontbig");


//=-(                        ~~//** CONSTANTS **//~~                       )-=//
user_event(2); // Lots of primitive constants are defined in here. These should be baked into the code before release (using the planned tool)

// Non-primitive constants are below. (mostly text strings)
rarity_names = ["Common", "Uncommon", "Rare"];
negative_rarity_names = ["", "Dummy", "Void"];
item_type_names = ["Damage", "Knockback", "Healing", "Speed", "Critical", "Attack Speed", "Barrier", "Explosive", "Meta", "Burning"];
legendary_type_name = "Legendary";



//=-(                    ~~//** NON-ITEM MANAGEMENT **//~~                  )-=//

// Lightweight particles
lfx_list = ds_list_create();

// General utility
prev_attack = noone;

// FSpec
fspec_air_uses = 1;

// DSpec
chest_obj = noone;
dspec_cooldown_hits = DSPEC_INIT_CD_HITS; // Hits on the opponent remaining until DSpec goes off cooldown.
first_hit = false; // Mirrors has_hit, but is accessible from hit_player to track the first hit applied.
trishop_odds = TRISHOP_ODDS[0];
call_sfx_instance = noone;

//Death Messages
var death_messages = [
    "You have died. Maybe next time..",
    "ur dead LOL get wrecked",
    "DEAD",
    "Smashed.",
    "Your family will never know how you died.",
    "You died painlessly.",
    "Your death was extremely painful.",
    "Dead from blunt trauma to the face.",
    "You have passed away. Try again?",
    "Your internal organs have failed.",
    "This planet has killed you.",
    "Crushed.",
    "[TBD]",
    "You have broken every bone in your body.",
    "rekt",
    "ded",
    "Sucks to suck.",
    "They will surely feast on your flesh.",
    "Remember to use your Down Special.",
    "You are dead.",
    "Get styled upon.",
    "You walk towards the light.",
    "You embrace the void.",
    "Come back soon!",
    "Your body was gone an hour later.",
    "Try playing the Intermediate Defense tutorial to live longer.",
    "Choose a new character?",
    "Consider picking a different stage.",
    "That was definitely not your fault.",
    "That was absolutely your fault.",
    "Close!",
    "..the harder they fall.",
    "Beep.. beep.. beeeeeeeeeeeeeeeee",
    "You really messed up.",
    "It wasn't your time to die...",
    "You had a lot more to live for.",
    "Maybe next time.",
    "You die in a hilarious pose.",
    "You die a slightly embarassing death."
]
death_message_pick = death_messages[random_func_2( 0, array_length(death_messages), 1 )]
final_death_timer = 0;
is_na = 0; // n/a compat




//=-(                     ~~//** ITEM MANAGEMENT **//~~                     )-=//

// Item Grid
// Format: see IG indices.
// Do not reorder items without updating their indices (user_event2.gml)! If you need to remove an item, use RTY_DUMMY to disable it.
// Legendary items must be correctly tagged in the primary type field, or they will be treated as normal items.
// Critical items must be correctly tagged in either type field to enable their effects.
//#RCFBEGINDEFORMAT
item_grid = [
    ["Crowbar",                 RTY_COMMON,     ITP_KNOCKBACK,    ITP_DAMAGE,       0, noone, "Deal more damage & knockback to healthy enemies.", noone], // 0 | hit_player.gml
    ["Warbanner",               RTY_COMMON,     ITP_SPEED,        ITP_ATTACK_SPEED, 0, noone, "Taunt to place down a powerful buffing Warbanner.", noone], // 1 | other_init.gml, article3_update.gml, attack_update.gml, update.gml
    ["Headstompers",            RTY_COMMON,     ITP_DAMAGE,       noone,            0, noone, "Hurt enemies by fast-falling.", noone], // 2 | update.gml, AT_EXTRA1, hit_player.gml, melee hitbox update
    ["Armor-Piercing Rounds",   RTY_COMMON,     ITP_KNOCKBACK,    ITP_DAMAGE,       0, noone, "Strongs deal more damage and slightly more knockback.", noone], // 3 | melee hitbox update
    ["Bustling Fungus",         RTY_COMMON,     ITP_HEALING,      noone,            0, noone, "Crouch to heal over time.", noone], // 4 | update.gml, post_draw.gml
    ["Paul's Goat Hoof",        RTY_COMMON,     ITP_SPEED,        noone,            0, noone, "Move faster.", noone], // 5 | user_event0.gml
    ["Energy Drink",            RTY_COMMON,     ITP_SPEED,        noone,            0, noone, "Dash faster.", noone], // 6 | user_event0.gml
    ["Arcane Blades",           RTY_COMMON,     ITP_SPEED,        noone,            0, noone, "Move faster after reaching 100%.", noone], // 7 | user_event0.gml, update.gml
    ["Hermit's Scarf",          RTY_COMMON,     ITP_SPEED,        noone,            0, noone, "Parry, rolls, and airdodges have more invulnerability.", noone], // 8 | user_event0.gml
    ["Topaz Brooch",            RTY_COMMON,     ITP_BARRIER,      noone,            0, noone, "Gain 5% barrier on kill.", noone], // 9 | update.gml, general barrier utils
    ["Lens Maker's Glasses",    RTY_COMMON,     ITP_CRITICAL,     noone,            0, noone, "Critical Strikes deal more damage.", noone], // 10 | update.gml ~ melee hitbox update
    ["Tri-Tip Dagger",          RTY_COMMON,     ITP_CRITICAL,     noone,            0, noone, "Critical Strikes bleed opponents, dealing damage over time.", noone], // 11 | update.gml, hit_player.gml
    ["Taser",                   RTY_COMMON,     ITP_CRITICAL,     noone,            0, noone, "Critical Strikes briefly stun opponents.", noone], // 12 | hit_player.gml, update.gml
    ["Soldier's Syringe",       RTY_COMMON,     ITP_ATTACK_SPEED, noone,            0, noone, "Increased attack speed.", noone], // 13 | user_event0.gml
    ["Mocha",                   RTY_COMMON,     ITP_ATTACK_SPEED, ITP_SPEED,        0, noone, "Slightly increased movement & attack speed.", noone], // 14 | user_event0.gml
    ["Sticky Bomb",             RTY_COMMON,     ITP_EXPLOSIVE,    noone,            0, noone, "Blast attacks attach a little more firepower.", noone], // 15 | hit_player.gml, update.gml
    ["Gasoline",                RTY_COMMON,     ITP_EXPLOSIVE,    ITP_BURNING,      0, noone, "Blast attacks set enemies on fire.", noone], // 16 | hit_player.gml, user_event0.gml
    ["Tough Times",             RTY_COMMON,     ITP_LEGENDARY,    noone,            0, noone, "I'm coming home soon. Stay strong.", noone], // 17 | user_event0.gml
    
    ["Kjaro's Band",            RTY_UNCOMMON,   ITP_DAMAGE,       ITP_BURNING,      0, noone, "Strongs blast enemies with runic fire, lighting them ablaze.", noone], // 18 | hit_player.gml, user_event0.gml
    ["Runald's Band",           RTY_UNCOMMON,   ITP_KNOCKBACK,    noone,            0, noone, "Strongs blast enemies with runic ice, freezing to the bone.", noone], // 19 | melee hitbox update, hit_player.gml
    ["Ukelele",                 RTY_UNCOMMON,   ITP_KNOCKBACK,    ITP_CRITICAL,     0, noone, "..And his music was electric.", noone], // 20 | user_event0.gml, AT_USTRONG_2
    ["Hopoo Feather",           RTY_UNCOMMON,   ITP_SPEED,        noone,            0, noone, "Gain an extra jump.", noone], // 21 | user_event0.gml
    ["Guardian Heart",          RTY_UNCOMMON,   ITP_BARRIER,      noone,            0, noone, "Gain a 4% shield. Recharges outside of danger.", noone], // 22 | update.gml, got_hit.gml, user_event0.gml, general barrier utils
    ["Locked Jewel",            RTY_UNCOMMON,   ITP_BARRIER,      ITP_SPEED,        0, noone, "Gain a burst of shield and speed after opening chests.", noone], // 23 | attack_update.gml, update.gml, general barrier utils
    ["Harvester's Scythe",      RTY_UNCOMMON,   ITP_HEALING,      ITP_CRITICAL,     0, noone, "Critical Strikes heal you by a portion of the damage they deal.", noone], // 24 | hit_player.gml
    ["Ignition Tank",           RTY_UNCOMMON,   ITP_CRITICAL,     ITP_BURNING,      0, noone, "Critical Strikes deal extra knockback to enemies on fire.", noone], // 25 | Crit attacks, user_event0.gml, hit_player.gml, attack_update.gml, got_hit.gml, death.gml
    ["Predatory Instincts",     RTY_UNCOMMON,   ITP_CRITICAL,     ITP_ATTACK_SPEED, 0, noone, "Critical Strikes increase attack speed.", noone], // 26 | update.gml, hit_player.gml, user_event0.gml
    ["Stun Grenade",            RTY_UNCOMMON,   ITP_EXPLOSIVE,    noone,            0, noone, "Blast attacks stun enemies briefly.", noone], // 27 | hit_player.gml, update.gml
    ["AtG Missile Mk. 1",       RTY_UNCOMMON,   ITP_DAMAGE,       noone,            0, noone, "Strongs fire a missile.", noone], // 28 | user_event0.gml, hit_player.gml, article3, AT_EXTRA1
    ["Rusty Jetpack",           RTY_UNCOMMON,   ITP_SPEED,        noone,            0, noone, "Increase jump height and reduce gravity.", noone], // 29 | user_event0.gml
    ["Legendary Spark",         RTY_UNCOMMON,   ITP_LEGENDARY,    noone,            0, noone, "Smite them. Smite them all.", noone], // 30 | Unimplemented
    
    ["Ancient Scepter",         RTY_RARE,       ITP_DAMAGE,       noone,            0, noone, "Upgrade your Neutral Special.", noone], // 31 | Unimplemented
    ["Fireman's Boots",         RTY_RARE,       ITP_DAMAGE,       noone,            0, noone, "Fight fire with fire...", noone], // 32 | update.gml, article3, user_event0.gml
    ["AtG Missile Mk. 2",       RTY_RARE,       ITP_DAMAGE,       noone,            0, noone, "Hooah.", noone], // 33 | user_event0.gml, hit_player.gml, article3, AT_EXTRA1
    ["The Ol' Lopper",          RTY_RARE,       ITP_KNOCKBACK,    noone,            0, 35,    "Enemies above 120% take massive knockback.", noone], // 34 | update.gml, hit_player.gml, other_post_draw.gml
    ["Shattering Justice",      RTY_RARE,       ITP_KNOCKBACK,    noone,            0, 34,    "Enemies above 100% have their Armor shattered.", noone], // 35 | update.gml, hit_player.gml, other_post_draw.gml
    ["Classified Access Codes", RTY_RARE,       ITP_DAMAGE,       noone,            0, noone, "Down Special requests extreme reinforcements after 15 seconds.", noone], // 36 | article1_update.gml
    ["Photon Jetpack",          RTY_RARE,       ITP_SPEED,        noone,            0, 38,    "No hands!", noone], // 37 | user_event0.gml, update.gml, post_draw.gml
    ["H3AD-5T V2",              RTY_RARE,       ITP_SPEED,        noone,            0, 37,    "Jump much higher, and fall much faster.", noone], // 38 | user_event0.gml
    ["Hardlight Afterburner",   RTY_RARE,       ITP_SPEED,        noone,            0, noone, "Upgrades your side special.", noone], // 39 | update.gml, user_event0.gml
    ["Laser Scope",             RTY_RARE,       ITP_CRITICAL,     noone,            0, 41,    "Critical hits deal massive damage and knockback.", noone], // 40 | Crit attacks, set_attack.gml
    ["Laser Turbine",           RTY_RARE,       ITP_ATTACK_SPEED, noone,            0, 40,    "Gunshots charge up a huge laser blast.", noone], // 41 | Unimplemented
    ["Aegis",                   RTY_RARE,       ITP_BARRIER,      ITP_HEALING,      0, noone, "All healing also gives you half of its value as barrier.", noone], // 42 | user_event0, integrated into the healing-applying function (and general barrier utils)
    ["Brilliant Behemoth",      RTY_RARE,       ITP_EXPLOSIVE,    noone,            0, noone, "Your gunshots explode!", noone], // 43 | melee hitbox update, AT_EXTRA_1, attack_update.gml, got_hit.gml, death.gml, update.gml, user_event0
    ["Dio's Best Friend",       RTY_RARE,       ITP_HEALING,      noone,            0, noone, "Cheat death.", noone], // 44 | update.gml, death.gml
    ["Withered Best Friend",    RTY_VOID,       ITP_HEALING,      noone,            0, noone, "A spent item with no remaining power.", noone], // 45 | N/A
    ["57 Leaf Clover",          RTY_RARE,       ITP_LEGENDARY,    noone,            0, noone, "Luck is on your side.", noone], // 46 | article1_update.gml, got_hit.gml, update.gml, death.gml, user_event0
    
    ["Monster Tooth",           RTY_COMMON,     ITP_HEALING,      noone,            0, noone, "Enemies that get launched hard enough spawn healing orbs.", noone], // 47 | hit_player.gml, update.gml, article3
    ["Wax Quail",               RTY_UNCOMMON,   ITP_SPEED,        noone,            0, noone, "Jumping while dashing boosts you forward.", noone], // 48 | update.gml
    ["Filial Imprinting",       RTY_UNCOMMON,   ITP_ATTACK_SPEED, ITP_SPEED,        0, noone, "Hatch a strange creature who drops buffs every 15 seconds.", noone], // 49 | user_event0.gml, article3
    ["Energy Cell",             RTY_UNCOMMON,   ITP_ATTACK_SPEED, noone,            0, noone, "Gain attack speed the more you're damaged.", noone], // 50 | user_event0.gml, update.gml
    ["Shipping Request Form",   RTY_UNCOMMON,   ITP_META,         noone,            0, noone, "Increases the odds of calling down a Tri-Shop.", noone], // 51 | user_event0.gml
    ["Fire Shield",             RTY_COMMON,     ITP_DAMAGE,       ITP_BURNING,      0, noone, "Set opponents on fire when dodging or parrying attacks.", noone], // 52 | user_event0.gml, parry.gml, update.gml
    ["Trophy Hunter's Tricorn", RTY_RARE,       ITP_META,         ITP_DAMAGE,       0, noone, "Claim a trophy from the opponent with FStrong. Only has one shot.", noone], // 53 | user_event0.gml, fstrong_2.gml, attack_update.gml, update.gml, hit_player.gml, article3
    ["Trophy Hunter's Relic",   RTY_VOID,       ITP_META,         ITP_DAMAGE,       0, noone, "Looks kinda cool, but that's about it. ", noone], // 54 | N/A
    ["Fireworks",               RTY_COMMON,     ITP_DAMAGE,       noone,            0, noone, "Launch fireworks when opening chests.", noone], // 55 | user_event0, article1_update.gml, article3, AT_EXTRA1, hitbox_update.gml, got_parried.gml
    ["Snake Eyes",              RTY_UNCOMMON,   ITP_CRITICAL,     noone,            0, noone, "Consecutive critical hits become stronger.", noone], // 56 | set_attack.gml, hit_player.gml, user_event0.gml
    ["Ceremonial Dagger",       RTY_RARE,       ITP_CRITICAL,     noone,            0, noone, "Critical hits summon daggers to chase down opponents.", noone], // 57 | hit_player.gml, article3, AT_EXTRA1, hitbox_update.gml
    ["Growth Nectar",           RTY_RARE,       ITP_META,         noone,            0, noone, "Common items grow more powerful.", noone], // 58 | user_event0.gml, animation.gml, anywhere common items are implemented
    
]
//#RCFENDDEFORMAT

// Ordering for in-game utilities (debug displays and practice mode)
item_id_ordering = [
    ITEM_CROWBAR,       // 0
    ITEM_WARBANNER,
    ITEM_STOMPERS,
    ITEM_APROUNDS,
    ITEM_BUNGUS,
    ITEM_HOOF,          // 5
    ITEM_EDRINK,
    ITEM_BLADES,
    ITEM_SCARF,
    ITEM_BROOCH,
    ITEM_MTOOTH,        // 10
    ITEM_GLASSES,
    ITEM_BLEEDDAGGER,
    ITEM_TASER,
    ITEM_SYRINGE,
    ITEM_MOCHA,         // 15
    ITEM_STICKYBOMB,
    ITEM_GASOLINE,
    ITEM_FSHIELD,
    ITEM_FIREWORKS,     // 19
    noone, // category delimiter
    ITEM_FIREBAND,      // 21
    ITEM_ICEBAND,       
    ITEM_UKELELE,
    ITEM_RJETPACK,
    ITEM_QUAIL,         // 25
    ITEM_FEATHER,
    ITEM_HEART,
    ITEM_JEWEL,
    ITEM_FILIAL,
    ITEM_SNAKEEYES,     // 30
    ITEM_SCYTHE,
    ITEM_IGNITION,
    ITEM_INSTINCTS,
    ITEM_SHIPPING,
    ITEM_CELL,          // 35
    ITEM_STUNGRENADE,
    ITEM_ATG1,
    noone,
    ITEM_SCEPTER,       // 39
    ITEM_TRICORN,       // 40
    ITEM_FIREBOOTS,
    ITEM_ATG2,
    ITEM_LOPPER,
    ITEM_SHATTERING,
    ITEM_CODES,         // 45
    ITEM_PJETPACK,
    ITEM_HEADSET,
    ITEM_AFTERBURNER,
    ITEM_CRITDAGGER,
    ITEM_SCOPE,         // 50
    ITEM_TURBINE,
    ITEM_AEGIS,
    ITEM_BEHEMOTH,
    ITEM_NECTAR,
    ITEM_DIOS,          // 55
    noone,
    ITEM_TTIMES,        // 57
    ITEM_SPARK,
    ITEM_CLOVER,
];

ordering_start_indices = [0, 21, 39, 57];

// If items need to be manually removed from the pool for any reason (e.g. during an emergency patch), do so here.
// Format: item_grid[@ ITEM_NAME_HERE][@ IG_RARITY] = RTY_VOID;
// v BETA REMOVALS v
if (!get_match_setting(SET_PRACTICE)) {
    item_grid[@ ITEM_UKELELE][@ IG_RARITY] = RTY_VOID;
    item_grid[@ ITEM_SCEPTER][@ IG_RARITY] = RTY_VOID;
    item_grid[@ ITEM_TURBINE][@ IG_RARITY] = RTY_VOID;
}


// Inventory store
inventory_list = [];

// For use by item init (user_event0)
new_item_id = noone;

// For use by user_event1
ue1_command = 0;
item_silenced = false; // Flag that silences the next item-get pop-up. Reset on every UE1 call


// Randomizer properties
legendary_pool_size = array_create(3, 0); // to be initialized
rares_remaining = 3; // manual limit, assumes that at least 3 rares exist
uncommon_limit = UNCOMMON_LIMIT;
uncommon_pool_size = 0; // to be initialized
item_seed = player * 5; // max 200, this should hold within the rivals engine
common_count = 0; // Not actually used by the randomizer, but still handled in the same places.

grant_rarity = noone; // for user_event(1). This default value throws an error as a sanity check


// New randomizer
// Initialize arrays
p_item_ids = array_create(3);
p_item_weights = array_create(3);
p_item_values = array_create(3); // weight per quantity remaining
p_item_remaining = array_create(3);
p_legendary_ids = array_create(3);
p_legendary_available = array_create(3); // mostly identical to legendary_remaining; gets set to 0 in an incompat case
p_legendary_remaining = array_create(3);

for (var rty = 0; rty < 3; rty++) {
    p_item_ids[rty] = [];
    p_item_weights[rty] = [];
    p_item_values[rty] = [];
    p_item_remaining[rty] = [];
    p_legendary_ids[rty] = [];
    p_legendary_available[rty] = [];
    p_legendary_remaining[rty] = [];
}

// Populate arrays
var num_items = array_length(item_grid);
var type_weights = INIT_WEIGHTS;
for (var i = 0; i < num_items; i++) {
    var rty = item_grid[i][IG_RARITY];
    var itp = item_grid[i][IG_TYPE];
    
    if (rty < RTY_COMMON || RTY_RARE < rty) continue;
    var quantity = (rty == RTY_UNCOMMON) ? uncommon_limit : 1;
    
    if (itp == ITP_LEGENDARY) {
        item_grid[@ i][@ IG_RANDOMIZER_INDEX] = array_length(p_legendary_ids[rty]);
        array_push(p_legendary_ids[rty], i);
        array_push(p_legendary_available[rty], quantity);
        array_push(p_legendary_remaining[rty], quantity);
        legendary_pool_size[rty] += quantity;
    }
    else {
        item_grid[@ i][@ IG_RANDOMIZER_INDEX] = array_length(p_item_ids[rty]);
        array_push(p_item_ids[rty], i);
        array_push(p_item_weights[rty], quantity*type_weights[itp]);
        array_push(p_item_values[rty], type_weights[itp]);
        array_push(p_item_remaining[rty], quantity);
        if (rty == RTY_UNCOMMON) uncommon_pool_size += quantity;
    }
}

// Item weights are altered by 1-cost abyss runes.
if (get_match_setting(SET_RUNES)) {
    
    for (var rty = RTY_COMMON; rty <= RTY_RARE; rty++) {
        var arr_len = array_length(p_item_ids[rty]);
        for (var i = 0; i < arr_len; i++) {
            var iid = p_item_ids[rty][i];
            var type1 = item_grid[iid][IG_TYPE];
            var type2 = item_grid[iid][IG_TYPE2];
            if ( (has_rune("A") && (type1 == ITP_DAMAGE || type2 == ITP_DAMAGE || type1 == ITP_KNOCKBACK || type2 == ITP_KNOCKBACK))
              || (has_rune("B") && (type1 == ITP_SPEED || type2 == ITP_SPEED))
              || (has_rune("C") && (type1 == ITP_HEALING || type2 == ITP_HEALING || type1 == ITP_BARRIER || type2 == ITP_BARRIER))
              || (has_rune("D") && (type1 == ITP_CRITICAL || type2 == ITP_CRITICAL))
              || (has_rune("E") && (type1 == ITP_ATTACK_SPEED || type2 == ITP_ATTACK_SPEED))
              || (has_rune("F") && (type1 == ITP_EXPLOSIVE || type2 == ITP_EXPLOSIVE))
            ) {
                var quantity = p_item_remaining[rty][i];
                p_item_values[@ rty][@ i] = ABYSS_BUFFED_VALUE;
                p_item_weights[@ rty][@ i] = quantity * ABYSS_BUFFED_VALUE;
            }
        }
    }
    
}


// Item variables
// Keyword trackers
critical_active = 0;     // enables checks for crit items
attack_speed = 1;        // inits to 1, goes up with attack speed items
move_speed = 0;          // inits to 0, goes up with items like Paul's Goat Hoof
dodge_duration_add = 0;  // inits to 0, adds n frames to shield actions

// Attack overwrites (see set_attack.gml)
ntaunt_index = AT_TAUNT; // taunts altered by Ukelele/Warbanner
utaunt_index = AT_TAUNT;
dtaunt_index = get_match_setting(SET_PRACTICE) ? AT_EXTRA_3 : AT_TAUNT;
ustrong_index = AT_USTRONG; // altered by Ukelele
fstrong_index = AT_FSTRONG; // altered by Tricorn

// Multipliers and fractional damage (see also: other_init.gml)
u_mult_damage_buffer = 0;

// Hitbox data storage (for use by ATG and Behemoth)
hbox_stored_damage = 0; // probably won't see use in practice
hbox_stored_bkb = 0;
hbox_stored_kbg = 0;
hbox_stored_angle = 0; // this one should actually grab the opponent's launch angle
hbox_stored_bhp = 0; // hitpause
hbox_stored_hps = 0;

// Status (see also: other_init.gml; user_event2.gml for indices)
commando_status_state = array_create(7);
commando_status_counter = array_create(7);
commando_status_owner = array_create(7, noone); // for the sake of the ditto

// Dodge duration overrides
dodge_duration_timer = 0;

// Percent tracking / barriers
old_damage = 0;
brooch_barrier = 0;      // Topaz Brooch
heart_barrier = 0;       // Guardian Heart
heart_barrier_endangered = 1;
heart_barrier_timer = 0;
heart_barrier_max = 0;   // see user_event0
jewel_barrier = 0;       // Locked Jewel
jewel_barrier_timer = 0;
aegis_barrier = 0;       // Aegis
aegis_ratio = AEGIS_RATIO_BASE;
aegis_odds_applied = false;
hud_barrier_fade_alpha = 0;

// Kill tracking
recently_hit = array_create(20, noone)
num_recently_hit = 0;

// Misc item-specific vars
warbanner_obj = noone;
commando_warbanner_owner = noone; // mirrored in other_init
commando_warbanner_strength = 0;
commando_warbanner_updated = 0;

stompers_active = 0;
stompers_hbox_air = noone;
stompers_hbox_ground = noone;

bungus_active = 0;
bungus_timer = 0;
bungus_vis_timer = 999;
bungus_vis_x = x;
bungus_vis_y = y;

bleeddagger_outline_col = [100, 0, 0];

instincts_timer = 0; // Predatory Instincts

do_ignite_hbox = 0; // Ignition Tank
ignition_odds_applied = 0; // Flag for buffing burn item odds

atg_freq = 0;

h3ad_lockout_timer = 0; // H3AD-5T, used for fast falling
h3ad_was_fast_falling = false;

fireboots_distance = 0;
fireboots_prev_x = x;
fireboots_lockout = 0;

do_behemoth_hbox = 0;
behemoth_hfx = noone;
behemoth_hfx_hitstop = 0;
behemoth_odds_applied = 0; // Flag for buffing explosive item odds

pjetpack_fuel = 0;
pjetpack_fuel_max = 75;
pjetpack_available = 0;
pjetpack_hud_alpha = 0;
pjetpack_vis_fuel = 0;
pjetpack_sound = noone;

dios_revive_timer = -999;
dios_stored_damage = 0;

clover_active = false;
clover_test = false;
clover_timer = 0;
clover_caught = [];

tooth_awaiting_spawn = array_create(20, -1);

quail_do_boost = 0;

filial_num_spawned = 0;
filial_aspeed_timer = 0;
filial_speed_timer = 0;
filial_do_update = false;
filial_aspeed_outline = [170, 0, 0];
filial_speed_outline = [50, 40, 160];
filial_double_outline = [120, 31, 150];

fshield_damage = 0;
fshield_triggered = 0;

do_tricorn_remove = 0;

fireworks_freq = 0;

snakeeyes_active = 0;

nectar_mult = 1;
nectar_prev_outline = [0, 0, 0];


// Training mode utility
tmu_state = TMU_INACTIVE;
tmu_exists = get_match_setting(SET_PRACTICE);
init_prompt_active = tmu_exists;
init_prompt_timer = 0;
if (tmu_exists) {
    
    tmu_timer = 0;
    
    tmu_row = 0;
    tmu_column = 0;
    tmu_selected = 0;
    tmu_display_row = 0;
    
    tmu_item_panel = 0;
    tmu_item_panel_max = 3;
    tmu_item_panel_contents = noone; // filled on load
    tmu_legendary_unlock_counter = 0;
    
    tmu_item_id = 0;
    tmu_signal_add_item = false;
    tmu_signal_remove_item = false;
    
    tmu_y_offscreen = -200;
    tmu_y_offset = tmu_y_offscreen;
    
    tmu_infowindow = noone;
    tmu_infowindow_active = false;
    
    orig_depth = depth;
    
}


//          Sound Effects (gonna use init this time, wanna see if it makes it easier)                //
s_dag_swing = sound_get("cm_dagger_swing");
s_cbar = sound_get("cm_crowbar");
s_dios = sound_get("cm_item_dios");
s_shotty = sound_get("cm_shotgun_blast");
s_reload = sound_get("cm_shotgun_load");

s_gunf = sound_get("cm_shootfast"); //fast, multihit
s_gunl = sound_get("cm_shootlight_1"); //light
s_gunm = sound_get("cm_shootlight2"); //med
s_gunh = sound_get("cm_shootmedwav"); //heavy //idk why it got named that lol
s_tricorn = sound_get("c_tricorn_BOOM");
s_tap = sound_get("cm_dspec_taptaptap");

s_knifel = sound_get("sfx_knifehit_s");
s_knifem = sound_get("sfx_knifehit_m");
s_crit = sound_get("cm_crit");
s_critheal = sound_get("cm_crit_heal");

s_roll = sound_get("cm_roll");
s_coin = sound_get("cm_shine");
s_slide = sound_get("cm_slide");

s_cd = sound_get("cm_cdend");
s_cfall = sound_get("cm_chestfall");
s_cland = sound_get("cm_chestland");
s_itemw = sound_get("cm_item_white");
s_itemg = sound_get("cm_item_green");
s_itemr = sound_get("cm_item_red");

s_mortem = sound_get("death");
s_jailed = sound_get("sentence");
s_err = sound_get("cm_err");
//


// 

//                      TEMPLATE ATTACK/WINDOW INDEXES                        //

/*
- free attack data indexes technically start at 24 up to 99, went with 30 to
make it look cleaner
*/


// might add ai indexes here later so you can tell the cpu when to use certain
// moves

/*
- free window data indexes technically start at 61 up to 99, went with 70 to
make it look cleaner
*/

// adds looping frames to an attack's charge window, while charging
AG_WINDOW_HAS_CHARGE_LOOP       = 70;   // if the window has a charge anim loop
AG_WINDOW_CHARGE_FRAME_START    = 71;   // anim frame of the start of the loop
AG_WINDOW_CHARGE_FRAMES         = 72;   // total number of frames
AG_WINDOW_CHARGE_LOOP_SPEED     = 73;   // speed of the loop animation

AG_WINDOW_GRAB_OPPONENT         = 74;   // if the window is a grab window (1),
                                        // hold opponent, otherwise let them go
AG_WINDOW_GRAB_POS_X            = 75;   // x position to hold grabbed opponent
AG_WINDOW_GRAB_POS_Y            = 76;   // y position to hold grabbed opponent

AG_WINDOW_CAN_WALLJUMP          = 77;   // if the player can walljump out of the
                                        // window



//                               HITBOX INDEXES                               //

/*
- free hitbox data indexes technically start at 54 up to 99, went with 60 to
make it look cleaner
*/

HG_HAS_GRAB                     = 60;   // makes the hitbox into a command grab
HG_BREAKS_GRAB                  = 61;   // if the grabbed player is hit, they're
                                        // no longer grabbed
HG_GRAB_WINDOW_GOTO             = 62;   // window the grab goes into
                                        // -1 if it continues in the same window
HG_GRAB_WINDOWS_NUM             = 63;   // up to what window the grab goes to
                                        // -1 if it doesnt change window num
HG_HAS_LERP                     = 64;   // if the hitbox has lerp properties
HG_LERP_PERCENT                 = 65;   // how much pull the lerp has
                                        // from 0.0~1.0
HG_LERP_POS_X                   = 66;   // x position that the lerp pulls to
HG_LERP_POS_Y                   = 67;   // y position that the lerp pulls to

HG_PROJECTILE_MULTIHIT          = 70;   // if a projectile multihits
HG_PROJECTILE_MULTIHIT_RATE     = 71;   // rate at which a projectile multihits
                                        // ex.: if 10, hits every 10 frames
                                        // (individual per opponent)
HG_PROJECTILE_MAX_HITS          = 72;   // max number of times the projectile
                                        // can hit before being destroyed
                                        // (individual per opponent)
                                        // put -1 for no limit

// if you're making custom indexes for your character, I recommend starting at
// 80 or 90, as slots up to 79 may be filled in future updates

/*
if you're using multihit properties, be sure to check if the projectile goes 
through enemies, otherwise it might just despawn on hit
*/


//=-(                    ~~//** TEMPLATE VARIABLES **//~~                    )-=//


//                               PRE-SET STUFF                                //

// animation stuff
idle_air_loops                  = false;// whether idle air has a looping 
                                        // animation or not
idle_air_looping                = false;// checks if the loop is happening
jump_frames                     = 5;    // how many animation frames the jump 
                                        // has, the loop starts there
idle_air_loop_speed             = 0.25;  // animation speed of the loop
idle_air_platfalls              = false; // if the character has an animation for
                                        // dropping from platforms
idle_air_platfalling            = false;// checks if platfall is happening
idle_air_platfall_speed         = 0.25; // platfall animation speed
idle_air_platfall_frames        = 7;    // how many frames the platfall anim has
                                        // when finished goes to air idle
dash_moonwalks                  = false; // if the character has a moonwalk anim

//=-(                     ~~//** CUSTOM EFFECTS **//~~                     )-=//


//                           --visual effects--                               //
fx_crit                     = hit_fx_create(sprite_get("vfx_crit"), 24);
fx_crit_blood               = hit_fx_create(sprite_get("vfx_crit_blood"), 24);
fx_crit_shock               = hit_fx_create(sprite_get("vfx_crit_shock"), 24);
fx_crit_shock_long          = hit_fx_create(sprite_get("vfx_crit_shock"), 50);
fx_blast                    = hit_fx_create(sprite_get("vfx_blast"), 17);

fx_item_res                 = hit_fx_create(sprite_get("vfx_item_res"), 160);
fx_bleed                    = [hit_fx_create(sprite_get("vfx_bleed"), 15), hit_fx_create(sprite_get("vfx_bleed_2"), 15)];
fx_sucker_buff_red          = hit_fx_create(sprite_get("vfx_sucker_buff_red"), 16);
fx_sucker_buff_blue         = hit_fx_create(sprite_get("vfx_sucker_buff_blue"), 16);

fx_small_chest_land         = hit_fx_create(sprite_get("dspec_smallchest_landvfx"), 16);
fx_large_chest_land         = hit_fx_create(sprite_get("dspec_largechest_landvfx"), 16);



//=-(                      ~~//** BASE STATS **//~~                        )-=//

//                              --hurtboxes--                                 //
hurtbox_spr                     = asset_get("ex_guy_hurt_box");
crouchbox_spr                   = asset_get("ex_guy_crouch_box");
air_hurtbox_spr                 = -1;
hitstun_hurtbox_spr             = -1;


//                  --animation speeds + %-arrow offset--                     //
char_height                     = 60;
idle_anim_speed                 = .1;
crouch_anim_speed               = 0.03;
walk_anim_speed                 = 0.125;
dash_anim_speed                 = 0.2;
pratfall_anim_speed             = 0.25;
walk_anim_speed_base            = walk_anim_speed;
dash_anim_speed_base            = dash_anim_speed;

//                      --grounded movement stats--                           //
walk_speed                      = 2.4;
walk_accel                      = 0.2;
walk_turn_time                  = 6;
initial_dash_time               = 12;
initial_dash_speed              = 6;
dash_speed                      = 5.5;
dash_turn_time                  = 10;
dash_turn_accel                 = 1.5;
dash_stop_time                  = 8;
dash_stop_percent               = 0.35;
ground_friction                 = 0.7;
moonwalk_accel                  = 1.4;
walk_speed_base                 = walk_speed;
walk_accel_base                 = walk_accel;
initial_dash_speed_base         = initial_dash_speed;
dash_speed_base                 = dash_speed;
moonwalk_accel_base             = moonwalk_accel;


//                       --aerial movement stats--                            //
jump_start_time                 = 5; // minus one
jump_speed                      = 11;
short_hop_speed                 = 6;
djump_speed                     = 10;
jump_speed_base                 = jump_speed;
short_hop_speed_base            = short_hop_speed;
djump_speed_base                = djump_speed;

leave_ground_max                = 7;
max_jump_hsp                    = 6;
air_max_speed                   = 5;
jump_change                     = 3;
max_jump_hsp_base               = max_jump_hsp;
air_max_speed_base              = air_max_speed;

air_accel                       = 0.3;
prat_fall_accel                 = 0.85;
air_friction                    = 0.04;

max_djumps                      = 1;
double_jump_time                = 28;
max_djumps_base                 = max_djumps;

walljump_hsp                    = 7;
walljump_vsp                    = 8;
walljump_time                   = 15;
wall_frames                     = 2;
walljump_vsp_base               = walljump_vsp;

max_fall                        = 10;
fast_fall                       = 14;
gravity_speed                   = 0.5;
hitstun_grav                    = 0.5;
max_fall_base                   = max_fall;
fast_fall_base                  = fast_fall;
gravity_speed_base              = gravity_speed;

//                    --character knockback adjustment--                      //
/* 
- higher num = 'lighter' character; 
- lower num = 'heavier' character 
*/
knockback_adj                   = 1.0;
knockback_adj_base              = knockback_adj;

//                           --landing stats--                                //
land_time                       = 4; 
prat_land_time                  = 10;
wave_land_time                  = 8;
wave_land_adj                   = 1.35;
wave_friction                   = 0.12;

//                          --animation frames--                              //
crouch_startup_frames           = 1;
crouch_active_frames            = 2;
crouch_recovery_frames          = 1;

dodge_startup_frames            = 1;
dodge_active_frames             = 2;
dodge_recovery_frames           = 2;

tech_active_frames              = 3;
tech_recovery_frames            = 1;

techroll_startup_frames         = 2;
techroll_active_frames          = 2;
techroll_recovery_frames        = 2;


air_dodge_startup_frames        = 1;
air_dodge_active_frames         = 1;
air_dodge_recovery_frames       = 2;

roll_forward_startup_frames     = 1;
roll_forward_active_frames      = 3;
roll_forward_recovery_frames    = 2;

roll_back_startup_frames        = 1;
roll_back_active_frames         = 3;
roll_back_recovery_frames       = 2;

//                        --defensive action speed--                          //
techroll_speed                  = 10;

air_dodge_speed                 = 7.5;

roll_forward_max                = 9; 
roll_backward_max               = 9;

//                      --base movement sound effects--                       //
land_sound                      = asset_get("sfx_land_med");
landing_lag_sound               = asset_get("sfx_land");
waveland_sound                  = asset_get("sfx_waveland_zet");
jump_sound                      = asset_get("sfx_jumpground");
djump_sound                     = asset_get("sfx_jumpair");
air_dodge_sound                 = asset_get("sfx_quick_dodge");

//                       --ranno bubble visual offset--                       //
bubble_x                        = 0;
bubble_y                        = 8;

//win stuff
set_victory_portrait(sprite_get("portrait_base"));

init_complete = true;

one two
1 1 1 1 IG_RARITY "waow";