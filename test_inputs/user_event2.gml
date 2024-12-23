
//~ CONSTANTS ~//
// Everything defined in here should ideally be a primitive, unless it's marked for use only in init.gml.

//#region Custom Indexes

// UE1 -> user_event1 commands
UE1_GENERATE = 0;
UE1_GRANT = 1;
UE1_REVOKE = 2;
UE1_GEN_THREE = 3;

// RTY -> Rarity
RTY_VOID = -2; // for items that cannot drop under normal circumstances
RTY_DUMMY = -1; // for if an item needs to be dummied out
RTY_COMMON = 0;
RTY_UNCOMMON = 1;
RTY_RARE = 2;

//ITP -> Item Type
ITP_LEGENDARY = -1; // special type
ITP_DAMAGE = 0;
ITP_KNOCKBACK = 1;
ITP_HEALING = 2;
ITP_SPEED = 3;
ITP_CRITICAL = 4;
ITP_ATTACK_SPEED = 5;
ITP_BARRIER = 6;
ITP_EXPLOSIVE = 7;
ITP_META = 8;         // Refers to items that operate on or grant other items.
ITP_BURNING = 9;      // Should really only be used as a secondary index
NUM_ITP_INDICES = 10; // update as needed! excludes legendary.
                      // be sure to also add a new entry to the weight and name arrays.

// IG -> Item grid
IG_NAME = 0;
IG_RARITY = 1;
IG_TYPE = 2; // Primary type sets the item's probability.
IG_TYPE2 = 3; // Tentative inclusion, might remove
IG_NUM_HELD = 4;
IG_INCOMPATIBLE = 5; // denotes an index for another item that, if held, prevents this item from being held.
IG_DESC = 6;
IG_RANDOMIZER_INDEX = 7;

// AG -> Attack Grid
AG_DISABLES_JETPACK = 40;

// HG -> Hitbox Grid
HG_IS_CRITICAL = 80;
HG_PROJECTILE_FAKE_HIT = 81; // disables certain on-hit behavior, such as dspecial cooldown updates
HG_STRONG_FINISHER = 82;
HG_IS_BLAST = 83;
HG_IS_GUNSHOT = 2 * (20 + 22); // applies followup explosion while Brilliant Behemoth is active. should only be used on finishers

// ITEM -> Item indices
ITEM_CROWBAR        = 0;
ITEM_WARBANNER      = 1;
ITEM_STOMPERS       = 2;
ITEM_APROUNDS       = 3;
ITEM_BUNGUS         = 4;
ITEM_HOOF           = 5;
ITEM_EDRINK         = 6;
ITEM_BLADES         = 7;
ITEM_SCARF          = 8;
ITEM_BROOCH         = 9;
ITEM_GLASSES        = 10;
ITEM_BLEEDDAGGER    = 11;
ITEM_TASER          = 12;
ITEM_SYRINGE        = 13;
ITEM_MOCHA          = 14;
ITEM_STICKYBOMB     = 15;
ITEM_GASOLINE       = 16;
ITEM_TTIMES         = 17;

ITEM_FIREBAND       = 18;
ITEM_ICEBAND        = 19;
ITEM_UKELELE        = 20;
ITEM_FEATHER        = 21;
ITEM_HEART          = 22;
ITEM_JEWEL          = 23;
ITEM_SCYTHE         = 24;
ITEM_IGNITION       = 25;
ITEM_INSTINCTS      = 26;
ITEM_STUNGRENADE    = 27;
ITEM_ATG1           = 28;
ITEM_RJETPACK       = 29;
ITEM_SPARK          = 30;

ITEM_SCEPTER        = 31;
ITEM_FIREBOOTS      = 32;
ITEM_ATG2           = 33;
ITEM_LOPPER         = 34;
ITEM_SHATTERING     = 35;
ITEM_CODES          = 36;
ITEM_PJETPACK       = 37;
ITEM_HEADSET        = 38;
ITEM_AFTERBURNER    = 39;
ITEM_SCOPE          = 40;
ITEM_TURBINE        = 41;
ITEM_AEGIS          = 42;
ITEM_BEHEMOTH       = 43;
ITEM_DIOS           = 44;
ITEM_DIOS_SPENT     = 45;
ITEM_CLOVER         = 46;

ITEM_MTOOTH         = 47;
ITEM_QUAIL          = 48;
ITEM_FILIAL         = 49;
ITEM_CELL           = 50;
ITEM_SHIPPING       = 51;
ITEM_FSHIELD        = 52;
ITEM_TRICORN        = 53;
ITEM_TRICORN_SPENT  = 54;
ITEM_FIREWORKS      = 55;
ITEM_SNAKEEYES      = 56;
ITEM_CRITDAGGER     = 57;
ITEM_NECTAR         = 58;

// ST -> Statuses
ST_STICKY = 0;
ST_BLEED = 1;
ST_STUN_ELECTRIC = 2;
ST_STUN_EXPLOSIVE = 3;
ST_LOPPER = 4;
ST_SHATTERED = 5;
ST_STATIC = 6;

// TMU -> Training Mode Utility (states)
TMU_INACTIVE = -1;
TMU_OPENING = 0;
TMU_ITEM = 1;
TMU_ITEM_CLOSING = 2;
TMU_INFO = 3;
TMU_INFO_CLOSING = 4;

//#endregion

//#region General Properties

// Probability properties
LEGENDARY_ODDS = 0.01;
LEGENDARY_ABYSS_ODDS = 0.157;
UNCOMMON_LIMIT = 3; // per uncommon item
UNCOMMON_ABYSS_LIMIT = 5;
RARE_LIMIT = 3; // across all rare items
RARE_ABYSS_LIMIT = 5;
INIT_WEIGHTS = [6, 6, 4, 6, 6, 6, 4, 5, 4, 6]; // maps to primary ITP indices. this init array is only for use by init.gml!
ABYSS_BUFFED_VALUE = 15;
SYNERGY_BUFFED_VALUE = 10; // Used by Ignition Tank and Brilliant Behemoth

TRISHOP_ODDS = [25, 50, 75, 100]; // array goes with Shipping Request Form

SCHEST_C_WEIGHT = 80;
SCHEST_U_WEIGHT = 15;
SCHEST_R_WEIGHT = 5;

LCHEST_C_WEIGHT = 0;
LCHEST_U_WEIGHT = 80;
LCHEST_R_WEIGHT = 20;


// Move Speed stacking properties
MSPEED_WALK_ANIM_SCALE = 0.04;
MSPEED_DASH_ANIM_SCALE = 0.03;

MSPEED_WALK_SPEED_SCALE = 1;
MSPEED_WALK_ACCEL_SCALE = 0.08;
MSPEED_DASH_SPEED_SCALE = 0.8;
MSPEED_IDASH_SPEED_SCALE = 1;
MSPEED_MOONWALK_ACCEL_SCALE = 0.075;

MSPEED_MAX_JUMP_HSP_SCALE = 1;
MSPEED_AIR_MAX_HSP_SCALE = 0.5;

MAX_JUMP_MOD = 5; // for jump_speed

// Aesthetic Properties
TEXTBOX_BIG_THRESHOLD = 50; // in char length

//#endregion

//#region Attack Properties

DATTACK_SPEED_SCALE = 0.75;
DATTACK_EDRINK_SCALE = 1.5;

DSPEC_SCHEST_RADIUS = 54;
DSPEC_LCHEST_RADIUS = 54;
DSPEC_INIT_CD_HITS = 0;
DSPEC_SCHEST_CD_HITS = 5;
DSPEC_LCHEST_CD_HITS = 8;
DSPEC_CLOVER_HITS = 7;

FSPEC_GROUND_HSP_BASE = 10;
FSPEC_GROUND_HSP_SCALE = 0.5; // per move speed stack
FSPEC_AIR_HSP_BASE = 13;
FSPEC_AIR_HSP_SCALE = 0.65;
FSPEC_AIR_VSP_BASE = -5;
FSPEC_AIR_VSP_SCALE = -0.25;

//#endregion


//#region Item Properties

// Crowbar
CROWBAR_MULT_BASE = 0;
CROWBAR_MULT_SCALE = 0.5;
CROWBAR_KB_ADD_SCALE = 10;

// Warbanner
WARBANNER_MULT_BASE = 0;
WARBANNER_MULT_SCALE = 0.2;
WARBANNER_SPEED_BASE = 0;
WARBANNER_SPEED_SCALE = 1;
WARBANNER_ASPEED_BASE = 0;
WARBANNER_ASPEED_SCALE = 2;
WARBANNER_RADIUS_BASE = 100;
WARBANNER_RADIUS_SCALE = 50;

// Headstompers
STOMPERS_DAMAGE_SCALE = 0.5; // unlike other scalings, this one starts at 0 for a single item copy and increases from there
STOMPERS_BHP_SCALE = 1;

// Armor-Piercing Rounds
APROUNDS_DAMAGE_SCALE = 2;
APROUNDS_BKB_SCALE = 0.75;
APROUNDS_KBS_SCALE = 0;

// Bustling Fungus
BUNGUS_WAIT_TIME = 90;
BUNGUS_TICK_TIME = 30; // Heal 1% every n/(bungus count) frames

// Paul's Goat Hoof
HOOF_SPEED_SCALE = 1;

// Energy Drink
EDRINK_DASH_SPEED_SCALE = 1;
EDRINK_IDASH_SPEED_SCALE = 1.5;
EDRINK_DASH_ANIM_SCALE = 0.015;
EDRINK_MOONWALK_ACCEL_SCALE = 0.1;

// Arcane Blades
BLADES_SPEED_SCALE = 1.5;
BLADES_THRESHOLD = 100;

// Hermit's Scarf
SCARF_FRAMES_BASE = 0; // Added invuln time in frames
SCARF_FRAMES_SCALE = 2;

// Topaz Brooch
BROOCH_BARRIER_SCALE = 5;

// Lens Maker's Glasses
GLASSES_DAMAGE_BASE = 1;
GLASSES_DAMAGE_SCALE = 2;

// Tri-Tip Dagger
BLEEDDAGGER_DAMAGE_BASE = 2;
BLEEDDAGGER_DAMAGE_SCALE = 3;
BLEED_TICK_TIME = 45;

// Taser
TASER_STUN_BASE = 7;
TASER_STUN_SCALE = 7;

// Soldier's Syringe
SYRINGE_ASPEED_SCALE = 2;

// Mocha
MOCHA_SPEED_SCALE = 0.5;
MOCHA_ASPEED_SCALE = 1;

// Gasoline
GASOLINE_DAMAGE_BASE = 0;
GASOLINE_DAMAGE_SCALE = 3;

// Sticky Bomb
STICKY_DELAY = 30;
STICKY_DAMAGE_SCALE = 2;  // unlike other scalings, this one starts at 0 for a single item copy and increases from there
STICKY_CD = 45;

// Tough Times
TTIMES_KBADJ_EXP_SET = 0.9; // Sets kb_adj to 0.9^n

// Kjaro's Band
FIREBAND_DAMAGE_BASE = 2;
FIREBAND_DAMAGE_SCALE = 3;

// Runald's Band
ICEBAND_HITPAUSE = 8;
ICEBAND_EXTRA_HITPAUSE = 4;
ICEBAND_KBS_SCALE = 0.1;

// Guardian Heart
HEART_ENDANGERED_TIME = 300; // Time in frames until heart barrier starts regenerating
HEART_TICK_TIME = 60; // Time between regeneration ticks
HEART_BARRIER_BASE = 2;
HEART_BARRIER_SCALE = 2;

// Locked Jewel
JEWEL_BARRIER_SCALE = 4;
JEWEL_SPEED_SCALE = 2;
JEWEL_DURATION = 300;

// Harvester's Scythe
SCYTHE_HEAL_BASE = 0.166666666;
SCYTHE_HEAL_SCALE = 0.166666667;

// Ignition Tank
IGNITION_KBS_SCALE = 0.05;

// Predatory Instincts
INSTINCTS_DURATION = 300;
INSTINCTS_ASPEED_BASE = 1;
INSTINCTS_ASPEED_SCALE = 2;

// Stun Grenade
STUNGRENADE_STUN_BASE = 10;
STUNGRENADE_STUN_SCALE = 20;

// Rusty Jetpack
RJETPACK_JUMP_SCALE = 1;
RJETPACK_DJUMP_SCALE = 0.5;
RJETPACK_WJUMP_SCALE = 1;
RJETPACK_MAX_FALL_SCALE = -0.5;
RJETPACK_GRAV_SPEED_BASE = -0.05; // Just a flat modifier, no scaling here

// Fireman's Boots
FIREBOOTS_THRESHOLD = 26; // Number of pixels walked to generate a fire instance
FIREBOOTS_PARRY_LOCKOUT = 180;
FIREBOOTS_HIT_LOCKOUT = 5  *2 *  2;
FIREBOOTS_DAMAGE = 4;

// The Ol' Lopper
LOPPER_DAMAGE_THRESHOLD = 120;
LOPPER_KB_THRESHOLD = 6;
LOPPER_AWAIT_TIME = 60;
LOPPER_LOCKOUT = 300;

// Shattering Justice
SHATTERING_DAMAGE_THRESHOLD = 100;
SHATTERING_DURATION = 90;
SHATTERING_KB_SHRED = 0.099973; // oddly specific magic number value is used to better detect changes

// Photon Jetpack
PJETPACK_FUEL_MAX_BASE = 30;
PJETPACK_FUEL_MAX_SCALE = 45;
PJETPACK_ACCEL = 0.3;
PJETPACK_THRESHOLD = -1; // VSP value after a jump that re-enables the jetpack
PJETPACK_MAX_RISE = -5;
PJETPACK_MAX_FALL = 3;

// H3AD-5T V2
HEADSET_LOCKOUT_TIME = 8; // Fastfall lockout after a jump, in frames
HEADSET_JUMP_SCALE = 4;
HEADSET_FAST_FALL_SCALE = 6;

// Aegis
AEGIS_RATIO_BASE = 0.5;
AEGIS_RATIO_SCALE = 0.25;

// Laser Scope
SCOPE_KBS_ADD = 0.1; // Applied on ignition hitboxes, or if Snake Eyes is active
SCOPE_DAMAGE_ADD = 5; // Ditto (these will apply twice if both conditions are met)

// Brilliant Behemoth
BEHEMOTH_AWAIT_MULT = 0.67; // As a percentage of enemy hitpause to wait through before exploding
BEHEMOTH_HITPAUSE_MULT = 0.67;

// Dio's Best Friend
DIOS_REVIVE_WAIT = 60;
DIOS_INVINCE_TIME = 60;

// 57-Leaf Clover
CLOVER_RADIUS = 120;
CLOVER_Y = -20;
CLOVER_RUNTIME = 45;

// Wax Quail
QUAIL_JUMP_BASE = 1;
QUAIL_JUMP_SCALE = 1;
QUAIL_WAVE_BASE = 2;
QUAIL_WAVE_SCALE = 3;

// Filial Imprinting
FILIAL_CHASE_RANGE = 150;
FILIAL_ENDCHASE_RANGE = 100;
FILIAL_CHASE_VARIANCE = 250;
FILIAL_WARP_RADIUS = 720;
FILIAL_BUFFSPAWN_FRAMES = 600;

FILIAL_BUFF_DURATION = 240;
FILIAL_ASPEED_STACKS = 3;
FILIAL_SPEED_STACKS = 2;
FILIAL_HEAL_AMOUNT = 4;

// Energy Cell
CELL_THRESHOLD_BASE = 40;
CELL_THRESHOLD_SCALE = -5; // Note that this scaling causes problems upon reaching non-positive values! The standard uncommon limit prevents this.

// Fire Shield
FSHIELD_DAMAGE_BASE = 4;
FSHIELD_DAMAGE_SCALE = 2;
FSHIELD_RADIUS = 100; // for collision on proc
FSHIELD_DODGE_RADIUS = 60; // strictly for activation on dodges

// Snake Eyes
SNAKEEYES_KBS_ADD = 0.05; // Added once for each Snake Eyes over the baseline
SNAKEEYES_DAMAGE_ADD = 3; // Ditto

//#endregion