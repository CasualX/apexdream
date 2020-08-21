## Interfaces

```
r5apex.exe!0x011a24a8 ClientRenderTargets001
r5apex.exe!0x01197908 EngineTraceClient004
r5apex.exe!0x011963d8 EngineTraceClientDecals004
r5apex.exe!0x01893640 EventSystem001
r5apex.exe!0x01e7abd0 GameUI011
r5apex.exe!0x0119aee8 ISoundC002
r5apex.exe!0x01cacf08 RunGameEngine005
r5apex.exe!0x012ef1c0 ShaderSystem002
r5apex.exe!0x0189f570 VClient018
r5apex.exe!0x01b39ae8 VClientEntityList003
r5apex.exe!0x01e79b80 VClientPrediction001
r5apex.exe!0x011991e0 VCvarQuery001
r5apex.exe!0x01197a50 VDebugOverlay004
r5apex.exe!0x0119baa8 VENGINE_GAMEUIFUNCS_VERSION005
r5apex.exe!0x012dc6c0 VENGINE_LAUNCHER_API_VERSION004
r5apex.exe!0x011a5d80 VEngineModel016
r5apex.exe!0x01199d78 VEngineRandom001
r5apex.exe!0x011996c8 VEngineRenderView013
r5apex.exe!0x01e98e70 VGUI_System010
r5apex.exe!0x0119c358 VMaterialSystemConfig004
```

## Miscellaneous

```
TimeDateStamp = 0x5f5823f9
CheckSum = 0x1fd77d1
GameVersion = "v3.0.3.454"
NUM_ENT_ENTRIES = 0x10000
r5apex.exe!0x18b9a98 cl_entitylist
r5apex.exe!0x119efec LocalEntityHandle
r5apex.exe!0x1c684b8 LocalPlayer
r5apex.exe!0x126f550 GlobalVars
r5apex.exe!0x40deef0 PlayerResources
r5apex.exe!0x40deec8 ViewRender + 0x1b3bd0 ViewMatrix
r5apex.exe!0x126f850 ClientState
r5apex.exe!0x126f8e8 SignonState
r5apex.exe!0x126fa00 LevelName
CWeaponX!0x1e08 m_flProjectileSpeed
CWeaponX!0x1e10 m_flProjectileScale
```

## NetworkedStringTables

```
r5apex.exe!0x040de5e0 EffectDispatch
r5apex.exe!0x040ddd28 ExtraParticleFilesTable
r5apex.exe!0x040deee0 ImpactEffectTableNames
r5apex.exe!0x040ddcf8 Materials
r5apex.exe!0x040deeb0 Movies
r5apex.exe!0x040de5f8 ParticleEffectNames
r5apex.exe!0x040dd7f8 Scenes
r5apex.exe!0x040ddce0 ScriptNames
r5apex.exe!0x040ddcd8 SettingsAssets
r5apex.exe!0x040deef8 SoundIDs
r5apex.exe!0x040ddd20 VguiScreen
r5apex.exe!0x040deea0 WeaponNames
r5apex.exe!0x040d9388 localize
```

## Buttons

These are addresses to global instances of the [`kbutton_t`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/kbutton.h#L14-L20) struct.

```
r5apex.exe!0x040defa0 kbutton_t in_attack
r5apex.exe!0x040def38 kbutton_t in_backward
r5apex.exe!0x081ca368 kbutton_t in_break
r5apex.exe!0x085c8700 kbutton_t in_camin
r5apex.exe!0x081ca6a0 kbutton_t in_camout
r5apex.exe!0x081ca650 kbutton_t in_campitchdown
r5apex.exe!0x085c8790 kbutton_t in_campitchup
r5apex.exe!0x081ca6e0 kbutton_t in_camyawleft
r5apex.exe!0x081ca680 kbutton_t in_camyawright
r5apex.exe!0x040def60 kbutton_t in_commandermousemove
r5apex.exe!0x040df038 kbutton_t in_dodge
r5apex.exe!0x081ca388 kbutton_t in_duck
r5apex.exe!0x040def10 kbutton_t in_forward
r5apex.exe!0x040defc0 kbutton_t in_graph
r5apex.exe!0x040df018 kbutton_t in_jump
r5apex.exe!0x085c8768 kbutton_t in_klook
r5apex.exe!0x081ca6c8 kbutton_t in_left
r5apex.exe!0x081ca670 kbutton_t in_lookdown
r5apex.exe!0x081ca378 kbutton_t in_lookup
r5apex.exe!0x085c8778 kbutton_t in_melee
r5apex.exe!0x081ca6f0 kbutton_t in_movedown
r5apex.exe!0x040def00 kbutton_t in_moveleft
r5apex.exe!0x040def28 kbutton_t in_moveright
r5apex.exe!0x085c8748 kbutton_t in_moveup
r5apex.exe!0x085c8758 kbutton_t in_offhand0
r5apex.exe!0x081ca6b8 kbutton_t in_offhand1
r5apex.exe!0x081ca480 kbutton_t in_offhand2
r5apex.exe!0x081ca460 kbutton_t in_offhand3
r5apex.exe!0x081ca420 kbutton_t in_offhand4
r5apex.exe!0x040df008 kbutton_t in_pause_menu
r5apex.exe!0x040def70 kbutton_t in_ping
r5apex.exe!0x040defb0 kbutton_t in_reload
r5apex.exe!0x081ca690 kbutton_t in_right
r5apex.exe!0x081ca490 kbutton_t in_score
r5apex.exe!0x081ca490 kbutton_t in_showscores
r5apex.exe!0x040def48 kbutton_t in_speed
r5apex.exe!0x040defd0 kbutton_t in_strafe
r5apex.exe!0x085c8720 kbutton_t in_toggle_duck
r5apex.exe!0x085c8710 kbutton_t in_toggle_zoom
r5apex.exe!0x081ca470 kbutton_t in_use
r5apex.exe!0x081ca410 kbutton_t in_useAndReload
r5apex.exe!0x085c8730 kbutton_t in_use_alt
r5apex.exe!0x081ca400 kbutton_t in_use_long
r5apex.exe!0x040defe8 kbutton_t in_variableScopeToggle
r5apex.exe!0x081ca440 kbutton_t in_walk
r5apex.exe!0x081ca660 kbutton_t in_weaponCycle
r5apex.exe!0x081ca430 kbutton_t in_weapon_discard
r5apex.exe!0x081ca450 kbutton_t in_zoom
```

## ClientClasses

<details>
<summary><code>client_class CAI_BaseNPC</code></summary>

class_id: `0`  
sizeof: `7488`  
</details>
<details>
<summary><code>client_class CAmbientGeneric</code></summary>

class_id: `1`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CBaseAnimating</code></summary>

class_id: `2`  
sizeof: `5632`  
</details>
<details>
<summary><code>client_class CBaseAnimatingOverlay</code></summary>

class_id: `3`  
sizeof: `6528`  
</details>
<details>
<summary><code>client_class CBaseButton</code></summary>

class_id: `0`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CBaseCombatCharacter</code></summary>

class_id: `4`  
sizeof: `7040`  
</details>
<details>
<summary><code>client_class CBaseEntity</code></summary>

class_id: `5`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CBaseGrenade</code></summary>

class_id: `6`  
sizeof: `11520`  
</details>
<details>
<summary><code>client_class CBaseParticleEntity</code></summary>

class_id: `0`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CBaseTempEntity</code></summary>

class_id: `7`  
sizeof: `40`  
</details>
<details>
<summary><code>client_class CBaseToggle</code></summary>

class_id: `8`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBaseTrigger</code></summary>

class_id: `9`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CBaseVPhysicsTrigger</code></summary>

class_id: `11`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBaseViewModel</code></summary>

class_id: `10`  
sizeof: `20352`  
</details>
<details>
<summary><code>client_class CBoneFollower</code></summary>

class_id: `12`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBreakableProp</code></summary>

class_id: `13`  
sizeof: `5632`  
</details>
<details>
<summary><code>client_class CBreakableSurface</code></summary>

class_id: `14`  
sizeof: `3776`  
</details>
<details>
<summary><code>client_class CCascadeLight</code></summary>

class_id: `15`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CColorCorrection</code></summary>

class_id: `16`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CCrossbowBolt</code></summary>

class_id: `17`  
sizeof: `11392`  
</details>
<details>
<summary><code>client_class CDeathBoxProp</code></summary>

class_id: `18`  
sizeof: `5760`  
</details>
<details>
<summary><code>client_class CDynamicLight</code></summary>

class_id: `19`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CDynamicProp</code></summary>

class_id: `20`  
sizeof: `5696`  
</details>
<details>
<summary><code>client_class CDynamicPropLightweight</code></summary>

class_id: `21`  
sizeof: `5696`  
</details>
<details>
<summary><code>client_class CEntityBlocker</code></summary>

class_id: `22`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CEntityDissolve</code></summary>

class_id: `23`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CEntityLinkPage</code></summary>

class_id: `24`  
sizeof: `4736`  
</details>
<details>
<summary><code>client_class CEnvDecoy</code></summary>

class_id: `25`  
sizeof: `5632`  
</details>
<details>
<summary><code>client_class CEnvWind</code></summary>

class_id: `26`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CFirstPersonProxy</code></summary>

class_id: `27`  
sizeof: `5760`  
</details>
<details>
<summary><code>client_class CFuncBrush</code></summary>

class_id: `28`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CFuncBrushLightweight</code></summary>

class_id: `29`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CFuncMoveLinear</code></summary>

class_id: `30`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CGameRulesProxy</code></summary>

class_id: `31`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CGlobalNonRewinding</code></summary>

class_id: `32`  
sizeof: `4736`  
</details>
<details>
<summary><code>client_class CGrappleHook</code></summary>

class_id: `33`  
sizeof: `5696`  
</details>
<details>
<summary><code>client_class CHardPointEntity</code></summary>

class_id: `34`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CHardPointFrontierEntity</code></summary>

class_id: `35`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CHealthKit</code></summary>

class_id: `36`  
sizeof: `5632`  
</details>
<details>
<summary><code>client_class CImportantOnEntSound</code></summary>

class_id: `37`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CInfoPlacementHelper</code></summary>

class_id: `38`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CInfoTarget</code></summary>

class_id: `39`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CInfoTargetGravity</code></summary>

class_id: `40`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CInfoTargetMinimap</code></summary>

class_id: `41`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CLootGrabber</code></summary>

class_id: `42`  
sizeof: `5760`  
</details>
<details>
<summary><code>client_class CMissile</code></summary>

class_id: `43`  
sizeof: `11712`  
</details>
<details>
<summary><code>client_class CMovieDisplay</code></summary>

class_id: `44`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CNPC_Drone</code></summary>

class_id: `45`  
sizeof: `7552`  
</details>
<details>
<summary><code>client_class CNPC_Dropship</code></summary>

class_id: `46`  
sizeof: `7616`  
</details>
<details>
<summary><code>client_class CNPC_SentryTurret</code></summary>

class_id: `47`  
sizeof: `7552`  
</details>
<details>
<summary><code>client_class CNPC_Titan</code></summary>

class_id: `48`  
sizeof: `7680`  
</details>
<details>
<summary><code>client_class CParticleSystem</code></summary>

class_id: `49`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CPhysicsProp</code></summary>

class_id: `50`  
sizeof: `5760`  
</details>
<details>
<summary><code>client_class CPlayer</code></summary>

class_id: `51`  
sizeof: `17536`  
</details>
<details>
<summary><code>client_class CPlayerDecoy</code></summary>

class_id: `52`  
sizeof: `5760`  
</details>
<details>
<summary><code>client_class CPlayerResource</code></summary>

class_id: `53`  
sizeof: `12480`  
</details>
<details>
<summary><code>client_class CPlayerTasklist</code></summary>

class_id: `54`  
sizeof: `4032`  
</details>
<details>
<summary><code>client_class CPlayerVehicle</code></summary>

class_id: `55`  
sizeof: `7168`  
</details>
<details>
<summary><code>client_class CPlayerWaypoint</code></summary>

class_id: `56`  
sizeof: `3392`  
</details>
<details>
<summary><code>client_class CPointCamera</code></summary>

class_id: `57`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CPortal_PointPush</code></summary>

class_id: `58`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CPostProcessController</code></summary>

class_id: `59`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CPredictedFirstPersonProxy</code></summary>

class_id: `60`  
sizeof: `5824`  
</details>
<details>
<summary><code>client_class CProjectile</code></summary>

class_id: `61`  
sizeof: `11328`  
</details>
<details>
<summary><code>client_class CPropDoor</code></summary>

class_id: `62`  
sizeof: `5888`  
</details>
<details>
<summary><code>client_class CPropSurvival</code></summary>

class_id: `63`  
sizeof: `5696`  
</details>
<details>
<summary><code>client_class CRopeKeyframe</code></summary>

class_id: `64`  
sizeof: `3904`  
</details>
<details>
<summary><code>client_class CScriptMover</code></summary>

class_id: `65`  
sizeof: `6272`  
</details>
<details>
<summary><code>client_class CScriptMoverTrainNode</code></summary>

class_id: `66`  
sizeof: `4224`  
</details>
<details>
<summary><code>client_class CScriptNetData</code></summary>

class_id: `67`  
sizeof: `3200`  
</details>
<details>
<summary><code>client_class CScriptNetDataGlobal</code></summary>

class_id: `74`  
sizeof: `3520`  
</details>
<details>
<summary><code>client_class CScriptNetDataGlobalNonRewind</code></summary>

class_id: `75`  
sizeof: `3520`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_DEATH_BOX</code></summary>

class_id: `68`  
sizeof: `3328`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_GLOBAL</code></summary>

class_id: `69`  
sizeof: `3520`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_GLOBAL_NON_REWIND</code></summary>

class_id: `70`  
sizeof: `3520`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_PLAYER_EXCLUSIVE</code></summary>

class_id: `71`  
sizeof: `3648`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_PLAYER_GLOBAL</code></summary>

class_id: `72`  
sizeof: `3456`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_TITAN_SOUL</code></summary>

class_id: `73`  
sizeof: `3328`  
</details>
<details>
<summary><code>client_class CScriptProp</code></summary>

class_id: `76`  
sizeof: `5888`  
</details>
<details>
<summary><code>client_class CScriptTraceVolume</code></summary>

class_id: `77`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CShieldProp</code></summary>

class_id: `78`  
sizeof: `5760`  
</details>
<details>
<summary><code>client_class CSkyCamera</code></summary>

class_id: `79`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CStatueProp</code></summary>

class_id: `0`  
sizeof: `5824`  
</details>
<details>
<summary><code>client_class CStatusEffectPlugin</code></summary>

class_id: `80`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CTEBreakModel</code></summary>

class_id: `82`  
sizeof: `112`  
</details>
<details>
<summary><code>client_class CTEEffectDispatch</code></summary>

class_id: `83`  
sizeof: `208`  
</details>
<details>
<summary><code>client_class CTEExplosion</code></summary>

class_id: `84`  
sizeof: `136`  
</details>
<details>
<summary><code>client_class CTEGibEvent</code></summary>

class_id: `85`  
sizeof: `56`  
</details>
<details>
<summary><code>client_class CTEParticleSystem</code></summary>

class_id: `86`  
sizeof: `56`  
</details>
<details>
<summary><code>client_class CTEPhysicsProp</code></summary>

class_id: `87`  
sizeof: `96`  
</details>
<details>
<summary><code>client_class CTEProjectileTrail</code></summary>

class_id: `88`  
sizeof: `88`  
</details>
<details>
<summary><code>client_class CTEScriptParticleSystem</code></summary>

class_id: `89`  
sizeof: `80`  
</details>
<details>
<summary><code>client_class CTEScriptParticleSystemOnEntity</code></summary>

class_id: `90`  
sizeof: `64`  
</details>
<details>
<summary><code>client_class CTEScriptParticleSystemOnEntityWithPos</code></summary>

class_id: `91`  
sizeof: `88`  
</details>
<details>
<summary><code>client_class CTEShatterSurface</code></summary>

class_id: `92`  
sizeof: `120`  
</details>
<details>
<summary><code>client_class CTESoundDispatch</code></summary>

class_id: `93`  
sizeof: `72`  
</details>
<details>
<summary><code>client_class CTeam</code></summary>

class_id: `81`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CTitanSoul</code></summary>

class_id: `94`  
sizeof: `3520`  
</details>
<details>
<summary><code>client_class CTriggerCylinderHeavy</code></summary>

class_id: `95`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CTriggerNoGrapple</code></summary>

class_id: `96`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTriggerNoZipline</code></summary>

class_id: `97`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTriggerPlayerMovement</code></summary>

class_id: `98`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CTriggerPointGravity</code></summary>

class_id: `99`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CTriggerSlip</code></summary>

class_id: `100`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CTriggerUpdraft</code></summary>

class_id: `101`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTurret</code></summary>

class_id: `102`  
sizeof: `6720`  
</details>
<details>
<summary><code>client_class CVGuiScreen</code></summary>

class_id: `103`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CVortexSphere</code></summary>

class_id: `104`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CWaterLODControl</code></summary>

class_id: `105`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CWeaponX</code></summary>

class_id: `106`  
sizeof: `26240`  
</details>
<details>
<summary><code>client_class CWorld</code></summary>

class_id: `107`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CZipline</code></summary>

class_id: `108`  
sizeof: `4224`  
</details>
<details>
<summary><code>client_class CZiplineEnd</code></summary>

class_id: `109`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class DoorMover</code></summary>

class_id: `110`  
sizeof: `6336`  
</details>
<details>
<summary><code>client_class ScriptMoverLightweight</code></summary>

class_id: `111`  
sizeof: `6336`  
</details>
<details>
<summary><code>client_class Titan_Cockpit</code></summary>

class_id: `0`  
sizeof: `6144`  
</details>

### Addresses

```
r5apex.exe!0x011a5b68 ClientClass CAI_BaseNPC
r5apex.exe!0x011a2d18 ClientClass CAmbientGeneric
r5apex.exe!0x0119ee08 ClientClass CBaseAnimating
r5apex.exe!0x011a4368 ClientClass CBaseAnimatingOverlay
r5apex.exe!0x011a5988 ClientClass CBaseButton
r5apex.exe!0x011a0438 ClientClass CBaseCombatCharacter
r5apex.exe!0x01e7e038 ClientClass CBaseEntity
r5apex.exe!0x012d7e68 ClientClass CBaseGrenade
r5apex.exe!0x01ca94b8 ClientClass CBaseParticleEntity
r5apex.exe!0x011a5848 ClientClass CBaseTempEntity
r5apex.exe!0x011a4ee8 ClientClass CBaseToggle
r5apex.exe!0x011ac9b8 ClientClass CBaseTrigger
r5apex.exe!0x011a5d48 ClientClass CBaseVPhysicsTrigger
r5apex.exe!0x01c49a08 ClientClass CBaseViewModel
r5apex.exe!0x0119eba8 ClientClass CBoneFollower
r5apex.exe!0x0119f998 ClientClass CBreakableProp
r5apex.exe!0x011a2408 ClientClass CBreakableSurface
r5apex.exe!0x0119f8f8 ClientClass CCascadeLight
r5apex.exe!0x011a5348 ClientClass CColorCorrection
r5apex.exe!0x01e68238 ClientClass CCrossbowBolt
r5apex.exe!0x0119f3f8 ClientClass CDeathBoxProp
r5apex.exe!0x011a5028 ClientClass CDynamicLight
r5apex.exe!0x011a55c8 ClientClass CDynamicProp
r5apex.exe!0x0119f538 ClientClass CDynamicPropLightweight
r5apex.exe!0x01c61b28 ClientClass CEntityBlocker
r5apex.exe!0x011a3118 ClientClass CEntityDissolve
r5apex.exe!0x011aa8b8 ClientClass CEntityLinkPage
r5apex.exe!0x01ca3b88 ClientClass CEnvDecoy
r5apex.exe!0x0119f7b8 ClientClass CEnvWind
r5apex.exe!0x01e67498 ClientClass CFirstPersonProxy
r5apex.exe!0x011a3af8 ClientClass CFuncBrush
r5apex.exe!0x011a1df8 ClientClass CFuncBrushLightweight
r5apex.exe!0x011a0f08 ClientClass CFuncMoveLinear
r5apex.exe!0x01c9aee0 ClientClass CGameRulesProxy
r5apex.exe!0x01c52178 ClientClass CGlobalNonRewinding
r5apex.exe!0x01e691f8 ClientClass CGrappleHook
r5apex.exe!0x011a75b8 ClientClass CHardPointEntity
r5apex.exe!0x011a8b78 ClientClass CHardPointFrontierEntity
r5apex.exe!0x01ca9598 ClientClass CHealthKit
r5apex.exe!0x0126d588 ClientClass CImportantOnEntSound
r5apex.exe!0x01caf458 ClientClass CInfoPlacementHelper
r5apex.exe!0x01c975a8 ClientClass CInfoTarget
r5apex.exe!0x012d62d8 ClientClass CInfoTargetGravity
r5apex.exe!0x01c4c1f8 ClientClass CInfoTargetMinimap
r5apex.exe!0x011a50c8 ClientClass CLootGrabber
r5apex.exe!0x01e650e8 ClientClass CMissile
r5apex.exe!0x011a0078 ClientClass CMovieDisplay
r5apex.exe!0x011a0e68 ClientClass CNPC_Drone
r5apex.exe!0x011a0c18 ClientClass CNPC_Dropship
r5apex.exe!0x0119f5d8 ClientClass CNPC_SentryTurret
r5apex.exe!0x011a29f8 ClientClass CNPC_Titan
r5apex.exe!0x011a2e58 ClientClass CParticleSystem
r5apex.exe!0x011a4978 ClientClass CPhysicsProp
r5apex.exe!0x0119ef48 ClientClass CPlayer
r5apex.exe!0x01ca9480 ClientClass CPlayerDecoy
r5apex.exe!0x011a48d8 ClientClass CPlayerResource
r5apex.exe!0x01ca68a8 ClientClass CPlayerTasklist
r5apex.exe!0x011a9a38 ClientClass CPlayerVehicle
r5apex.exe!0x01ca5a88 ClientClass CPlayerWaypoint
r5apex.exe!0x011a2958 ClientClass CPointCamera
r5apex.exe!0x01caf558 ClientClass CPortal_PointPush
r5apex.exe!0x0119f678 ClientClass CPostProcessController
r5apex.exe!0x01e65b28 ClientClass CPredictedFirstPersonProxy
r5apex.exe!0x01e6abd8 ClientClass CProjectile
r5apex.exe!0x012de7d8 ClientClass CPropDoor
r5apex.exe!0x011a1fd8 ClientClass CPropSurvival
r5apex.exe!0x01ca34b8 ClientClass CRopeKeyframe
r5apex.exe!0x01ca8a00 ClientClass CScriptMover
r5apex.exe!0x01c9ffc8 ClientClass CScriptMoverTrainNode
r5apex.exe!0x01ca7078 ClientClass CScriptNetData
r5apex.exe!0x01ca4d80 ClientClass CScriptNetDataGlobal
r5apex.exe!0x01ca8a38 ClientClass CScriptNetDataGlobalNonRewind
r5apex.exe!0x01c9ff90 ClientClass CScriptNetData_SNDC_DEATH_BOX
r5apex.exe!0x01ca5ac0 ClientClass CScriptNetData_SNDC_GLOBAL
r5apex.exe!0x01c9fdb8 ClientClass CScriptNetData_SNDC_GLOBAL_NON_REWIND
r5apex.exe!0x01ca5498 ClientClass CScriptNetData_SNDC_PLAYER_EXCLUSIVE
r5apex.exe!0x01ca4db8 ClientClass CScriptNetData_SNDC_PLAYER_GLOBAL
r5apex.exe!0x01ca5af8 ClientClass CScriptNetData_SNDC_TITAN_SOUL
r5apex.exe!0x011a1708 ClientClass CScriptProp
r5apex.exe!0x01e64428 ClientClass CScriptTraceVolume
r5apex.exe!0x0119f498 ClientClass CShieldProp
r5apex.exe!0x01e7dab8 ClientClass CSkyCamera
r5apex.exe!0x011a52a8 ClientClass CStatueProp
r5apex.exe!0x01ca69b8 ClientClass CStatusEffectPlugin
r5apex.exe!0x011a9f78 ClientClass CTEBreakModel
r5apex.exe!0x011a70f8 ClientClass CTEEffectDispatch
r5apex.exe!0x011a6b68 ClientClass CTEExplosion
r5apex.exe!0x011a4548 ClientClass CTEGibEvent
r5apex.exe!0x011acb78 ClientClass CTEParticleSystem
r5apex.exe!0x011a8038 ClientClass CTEPhysicsProp
r5apex.exe!0x01e6adc8 ClientClass CTEProjectileTrail
r5apex.exe!0x01c51e98 ClientClass CTEScriptParticleSystem
r5apex.exe!0x01c500b8 ClientClass CTEScriptParticleSystemOnEntity
r5apex.exe!0x01c99130 ClientClass CTEScriptParticleSystemOnEntityWithPos
r5apex.exe!0x011ac058 ClientClass CTEShatterSurface
r5apex.exe!0x011a5ec8 ClientClass CTESoundDispatch
r5apex.exe!0x0126d4e8 ClientClass CTeam
r5apex.exe!0x011a0ad8 ClientClass CTitanSoul
r5apex.exe!0x01ca5460 ClientClass CTriggerCylinderHeavy
r5apex.exe!0x011a68e8 ClientClass CTriggerNoGrapple
r5apex.exe!0x011aad18 ClientClass CTriggerNoZipline
r5apex.exe!0x01ca8768 ClientClass CTriggerPlayerMovement
r5apex.exe!0x01ca6980 ClientClass CTriggerPointGravity
r5apex.exe!0x01ca5a50 ClientClass CTriggerSlip
r5apex.exe!0x011a85b8 ClientClass CTriggerUpdraft
r5apex.exe!0x01e680c8 ClientClass CTurret
r5apex.exe!0x0126d7e8 ClientClass CVGuiScreen
r5apex.exe!0x01e6a0e8 ClientClass CVortexSphere
r5apex.exe!0x011a9998 ClientClass CWaterLODControl
r5apex.exe!0x01e70f78 ClientClass CWeaponX
r5apex.exe!0x0126d6a8 ClientClass CWorld
r5apex.exe!0x01e7d598 ClientClass CZipline
r5apex.exe!0x01e7ba58 ClientClass CZiplineEnd
r5apex.exe!0x01ca3b50 ClientClass DoorMover
r5apex.exe!0x01ca86c8 ClientClass ScriptMoverLightweight
r5apex.exe!0x01cafd78 ClientClass Titan_Cockpit
```

## RecvTables

<details>
<summary><code>class DT_AI_BaseNPC extends DT_BaseCombatCharacter</code></summary>

```
{
	statuseffectsdata_npc: DT_AI_BaseNPC_StatusEffects,
	m_localOrigin: Vector,
	m_hGroundEntity: Int,
	m_iHealth: Int,
	m_localAngles: Vector,
	m_iMaxHealth: Int,
	m_lifeState: Int,
	m_inventory: DT_WeaponInventoryActiveWeaponOnly,
	m_fireteamSlotIndex: Int,
	m_aiSprinting: Int,
	m_aiNetworkFlags: Int,
	m_isHologram: Int,
	m_title: String,
	m_aiSettingsIndex: Int,
	m_subclass: Int,
}
```

### Offsets

```
DT_AI_BaseNPC!0x0000 statuseffectsdata_npc
DT_AI_BaseNPC!0x0004 m_localOrigin
DT_AI_BaseNPC!0x041c m_hGroundEntity
DT_AI_BaseNPC!0x0420 m_iHealth
DT_AI_BaseNPC!0x046c m_localAngles
DT_AI_BaseNPC!0x0550 m_iMaxHealth
DT_AI_BaseNPC!0x0770 m_lifeState
DT_AI_BaseNPC!0x19b0 m_inventory
DT_AI_BaseNPC!0x1b80 m_fireteamSlotIndex
DT_AI_BaseNPC!0x1cea m_aiSprinting
DT_AI_BaseNPC!0x1d0c m_aiNetworkFlags
DT_AI_BaseNPC!0x1d10 m_isHologram
DT_AI_BaseNPC!0x1d11 m_title
DT_AI_BaseNPC!0x1d34 m_aiSettingsIndex
DT_AI_BaseNPC!0x1d38 m_subclass
```
</details>
<details>
<summary><code>class DT_AI_BaseNPC_StatusEffects</code></summary>

```
{
	m_statusEffectsTimedNPCNV: DataTable,
	m_statusEffectsEndlessNPCNV: DataTable,
}
```

### Offsets

```
DT_AI_BaseNPC_StatusEffects!0x1b88 m_statusEffectsTimedNPCNV
DT_AI_BaseNPC_StatusEffects!0x1bd0 m_statusEffectsEndlessNPCNV
```
</details>
<details>
<summary><code>class DT_AmbientGeneric extends DT_BaseEntity</code></summary>

```
{
	m_radius: Float,
	m_isEnabled: Int,
	m_networkTableSoundID: Int,
	m_networkedSegmentEndpointWorldSpace: Vector,
	m_hasPolylineSegments: Int,
}
```

### Offsets

```
DT_AmbientGeneric!0x0a40 m_radius
DT_AmbientGeneric!0x0a44 m_isEnabled
DT_AmbientGeneric!0x0a50 m_networkTableSoundID
DT_AmbientGeneric!0x0a58 m_networkedSegmentEndpointWorldSpace
DT_AmbientGeneric!0x0a64 m_hasPolylineSegments
```
</details>
<details>
<summary><code>class DT_AnimRelativeData</code></summary>

```
{
	m_animInitialPos: Vector,
	m_animInitialVel: Vector,
	m_animInitialRot: Rotation,
	m_animInitialCorrectPos: Vector,
	m_animInitialCorrectRot: Rotation,
	m_animEntityToRefOffset: Vector,
	m_animEntityToRefRotation: Rotation,
	m_animBlendBeginTime: Time,
	m_animBlendEndTime: Time,
	m_animScriptSequence: Int,
	m_animScriptModel: Int,
	m_animIgnoreParentRot: Int,
	m_animMotionMode: Int,
}
```

### Offsets

```
DT_AnimRelativeData!0x0000 m_animInitialPos
DT_AnimRelativeData!0x000c m_animInitialVel
DT_AnimRelativeData!0x0018 m_animInitialRot
DT_AnimRelativeData!0x0028 m_animInitialCorrectPos
DT_AnimRelativeData!0x0034 m_animInitialCorrectRot
DT_AnimRelativeData!0x0044 m_animEntityToRefOffset
DT_AnimRelativeData!0x0050 m_animEntityToRefRotation
DT_AnimRelativeData!0x0060 m_animBlendBeginTime
DT_AnimRelativeData!0x0064 m_animBlendEndTime
DT_AnimRelativeData!0x0068 m_animScriptSequence
DT_AnimRelativeData!0x006c m_animScriptModel
DT_AnimRelativeData!0x0070 m_animIgnoreParentRot
DT_AnimRelativeData!0x0074 m_animMotionMode
```
</details>
<details>
<summary><code>class DT_BaseAnimating extends DT_BaseEntity</code></summary>

```
{
	serveranimdata: DT_ServerAnimationData,
	m_animPlaybackRate: Float,
	m_animFrozen: Int,
	m_animModelIndex: Int,
	m_animSequence: Int,
	m_nNewSequenceParity: Int,
	m_flPoseParameter: DataTable,
	m_bClientSideRagdoll: Int,
	m_vecForce: Vector,
	m_flEstIkOffset: Float,
	m_passDamageToParent: Int,
	m_animNetworkFlags: Int,
	m_animActive: Int,
	m_animCollisionEnabled: Int,
	m_animPlantingEnabled: Int,
	m_animRelativeData: DT_AnimRelativeData,
	m_syncingWithEntity: Int,
	m_predictedAnimEventData: DT_PredictedAnimEventData,
	m_nRagdollImpactFXTableId: Int,
	m_flSkyScaleStartValue: Float,
	m_flSkyScaleEndValue: Float,
	m_flSkyScaleStartTime: Time,
	m_flSkyScaleEndTime: Time,
	m_SequenceTransitioner: DT_SequenceTransitioner,
	m_nSkin: Int,
	m_skinMod: Int,
	m_nBody: Int,
	m_camoIndex: Int,
	m_nForceBone: Int,
	m_bSequenceFinished: Int,
	m_lockedAnimDeltaYaw: Float,
	m_flModelScale: Float,
}
```

### Offsets

```
DT_BaseAnimating!0x0000 serveranimdata
DT_BaseAnimating!0x0010 m_animPlaybackRate
DT_BaseAnimating!0x0014 m_animFrozen
DT_BaseAnimating!0x0018 m_animModelIndex
DT_BaseAnimating!0x001c m_animSequence
DT_BaseAnimating!0x0020 m_nNewSequenceParity
DT_BaseAnimating!0x0024 m_flPoseParameter
DT_BaseAnimating!0x0084 m_bClientSideRagdoll
DT_BaseAnimating!0x0088 m_vecForce
DT_BaseAnimating!0x0094 m_flEstIkOffset
DT_BaseAnimating!0x079c m_passDamageToParent
DT_BaseAnimating!0x0a68 m_animNetworkFlags
DT_BaseAnimating!0x0a6c m_animActive
DT_BaseAnimating!0x0a6f m_animCollisionEnabled
DT_BaseAnimating!0x0a70 m_animPlantingEnabled
DT_BaseAnimating!0x0a74 m_animRelativeData
DT_BaseAnimating!0x0b64 m_syncingWithEntity
DT_BaseAnimating!0x0b68 m_predictedAnimEventData
DT_BaseAnimating!0x0bd4 m_nRagdollImpactFXTableId
DT_BaseAnimating!0x0bd8 m_flSkyScaleStartValue
DT_BaseAnimating!0x0bdc m_flSkyScaleEndValue
DT_BaseAnimating!0x0be0 m_flSkyScaleStartTime
DT_BaseAnimating!0x0be4 m_flSkyScaleEndTime
DT_BaseAnimating!0x0c00 m_SequenceTransitioner
DT_BaseAnimating!0x0e88 m_nSkin
DT_BaseAnimating!0x0e8c m_skinMod
DT_BaseAnimating!0x0e90 m_nBody
DT_BaseAnimating!0x0e94 m_camoIndex
DT_BaseAnimating!0x0ed0 m_nForceBone
DT_BaseAnimating!0x0f34 m_bSequenceFinished
DT_BaseAnimating!0x0f38 m_lockedAnimDeltaYaw
DT_BaseAnimating!0x0f40 m_flModelScale
```
</details>
<details>
<summary><code>class DT_BaseAnimatingOverlay extends DT_BaseAnimating</code></summary>

```
{
	overlay_vars: DT_OverlayVars,
	m_animOverlayIsActive: DataTable,
	m_animOverlayStartTime: DataTable,
	m_animOverlayStartCycle: DataTable,
	m_animOverlayPlaybackRate: DataTable,
	m_animOverlayModelIndex: DataTable,
	m_animOverlaySequence: DataTable,
	m_animOverlayWeight: DataTable,
	m_animOverlayOrder: DataTable,
	m_animOverlayAnimTime: DataTable,
	m_animOverlayFadeInDuration: DataTable,
	m_animOverlayFadeOutDuration: DataTable,
}
```

### Offsets

```
DT_BaseAnimatingOverlay!0x0000 overlay_vars
DT_BaseAnimatingOverlay!0x0008 m_animOverlayIsActive
DT_BaseAnimatingOverlay!0x0014 m_animOverlayStartTime
DT_BaseAnimatingOverlay!0x0038 m_animOverlayStartCycle
DT_BaseAnimatingOverlay!0x005c m_animOverlayPlaybackRate
DT_BaseAnimatingOverlay!0x0080 m_animOverlayModelIndex
DT_BaseAnimatingOverlay!0x00a4 m_animOverlaySequence
DT_BaseAnimatingOverlay!0x00c8 m_animOverlayWeight
DT_BaseAnimatingOverlay!0x00ec m_animOverlayOrder
DT_BaseAnimatingOverlay!0x0110 m_animOverlayAnimTime
DT_BaseAnimatingOverlay!0x0134 m_animOverlayFadeInDuration
DT_BaseAnimatingOverlay!0x0158 m_animOverlayFadeOutDuration
```
</details>
<details>
<summary><code>class DT_BaseCombatCharacter extends DT_BaseAnimatingOverlay</code></summary>

```
{
	bcc_localdata: DT_BCCLocalPlayerExclusive,
	m_weaponGettingSwitchedOut: DataTable,
	m_showActiveWeapon3p: DataTable,
	m_vecViewOffset.x: Float,
	m_vecViewOffset.y: Float,
	m_vecViewOffset.z: Float,
	m_cloakEndTime: Float,
	m_cloakFadeInEndTime: Time,
	m_cloakFadeOutStartTime: Float,
	m_cloakFadeInDuration: Float,
	m_cloakFlickerAmount: Float,
	m_cloakFlickerEndTime: Time,
	m_networkedFlags: Int,
	m_deathVelocity: Vector,
	m_minimapData: DT_MinimapBaseEntityData,
	m_nameVisibilityFlags: Int,
	m_lastFiredTime: Time,
	m_lastFiredWeapon: Int,
	m_raiseFromMeleeEndTime: Time,
	m_sharedEnergyCount: Int,
	m_sharedEnergyTotal: Int,
	m_sharedEnergyLockoutThreshold: Int,
	m_lastSharedEnergyRegenTime: Time,
	m_sharedEnergyRegenRate: Time,
	m_sharedEnergyRegenDelay: Float,
	m_lastSharedEnergyTakeTime: Time,
	m_selectedWeapons: DataTable,
	m_latestPrimaryWeapons: DataTable,
	m_latestPrimaryWeaponsIndexZeroOrOne: DataTable,
	m_latestNonOffhandWeapons: DataTable,
	m_lastCycleSlot: Int,
	m_weaponPermission: Int,
	m_weaponDelayEnableTime: Time,
	m_weaponDisabledInScript: Int,
	m_weaponDisabledFlags: Int,
	m_weaponTypeDisabledFlags: Int,
	m_weaponTypeDisabledRefCount: DataTable,
	m_hudInfo_visibilityTestAlwaysPasses: Int,
	m_contextAction: Int,
	m_phaseShiftTimeStart: Time,
	m_phaseShiftTimeEnd: Time,
	m_targetInfoIconName: String,
}
```

### Offsets

```
DT_BaseCombatCharacter!0x0000 bcc_localdata
DT_BaseCombatCharacter!0x0008 m_weaponGettingSwitchedOut
DT_BaseCombatCharacter!0x0010 m_showActiveWeapon3p
DT_BaseCombatCharacter!0x0038 m_vecViewOffset.x
DT_BaseCombatCharacter!0x003c m_vecViewOffset.y
DT_BaseCombatCharacter!0x0040 m_vecViewOffset.z
DT_BaseCombatCharacter!0x019c m_cloakEndTime
DT_BaseCombatCharacter!0x01a0 m_cloakFadeInEndTime
DT_BaseCombatCharacter!0x01a4 m_cloakFadeOutStartTime
DT_BaseCombatCharacter!0x01a8 m_cloakFadeInDuration
DT_BaseCombatCharacter!0x01ac m_cloakFlickerAmount
DT_BaseCombatCharacter!0x01b0 m_cloakFlickerEndTime
DT_BaseCombatCharacter!0x03d4 m_networkedFlags
DT_BaseCombatCharacter!0x0454 m_deathVelocity
DT_BaseCombatCharacter!0x0948 m_minimapData
DT_BaseCombatCharacter!0x0998 m_nameVisibilityFlags
DT_BaseCombatCharacter!0x1984 m_lastFiredTime
DT_BaseCombatCharacter!0x1988 m_lastFiredWeapon
DT_BaseCombatCharacter!0x198c m_raiseFromMeleeEndTime
DT_BaseCombatCharacter!0x1990 m_sharedEnergyCount
DT_BaseCombatCharacter!0x1994 m_sharedEnergyTotal
DT_BaseCombatCharacter!0x1998 m_sharedEnergyLockoutThreshold
DT_BaseCombatCharacter!0x199c m_lastSharedEnergyRegenTime
DT_BaseCombatCharacter!0x19a0 m_sharedEnergyRegenRate
DT_BaseCombatCharacter!0x19a4 m_sharedEnergyRegenDelay
DT_BaseCombatCharacter!0x19a8 m_lastSharedEnergyTakeTime
DT_BaseCombatCharacter!0x1a08 m_selectedWeapons
DT_BaseCombatCharacter!0x1a0c m_latestPrimaryWeapons
DT_BaseCombatCharacter!0x1a14 m_latestPrimaryWeaponsIndexZeroOrOne
DT_BaseCombatCharacter!0x1a1c m_latestNonOffhandWeapons
DT_BaseCombatCharacter!0x1a24 m_lastCycleSlot
DT_BaseCombatCharacter!0x1a2c m_weaponPermission
DT_BaseCombatCharacter!0x1a30 m_weaponDelayEnableTime
DT_BaseCombatCharacter!0x1a34 m_weaponDisabledInScript
DT_BaseCombatCharacter!0x1a59 m_weaponDisabledFlags
DT_BaseCombatCharacter!0x1a5c m_weaponTypeDisabledFlags
DT_BaseCombatCharacter!0x1a60 m_weaponTypeDisabledRefCount
DT_BaseCombatCharacter!0x1a69 m_hudInfo_visibilityTestAlwaysPasses
DT_BaseCombatCharacter!0x1a7c m_contextAction
DT_BaseCombatCharacter!0x1aa8 m_phaseShiftTimeStart
DT_BaseCombatCharacter!0x1aac m_phaseShiftTimeEnd
DT_BaseCombatCharacter!0x1afc m_targetInfoIconName
```
</details>
<details>
<summary><code>class DT_BaseEntity</code></summary>

```
{
	movetype: Int,
	movecollide: Int,
	predictable_id: DT_PredictableId,
	HighlightSettings: DT_HighlightSettings,
	moveparent: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_usableType: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_clrRender: Int,
	m_cellZ: Int,
	m_clIntensity: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_bossPlayer: Int,
	m_shieldHealth: Int,
	m_shieldHealthMax: Int,
	m_bIsSoundCodeControllerValueSet: Int,
	m_flSoundCodeControllerValue: Float,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_iTeamNum: Int,
	m_teamMemberIndex: Int,
	m_squadID: Int,
	m_grade: Int,
	m_ignorePredictedTriggerFlags: Int,
	m_passThroughFlags: Int,
	m_passThroughThickness: Int,
	m_passThroughDirection: Float,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_bRenderWithViewModels: Int,
	m_nRenderFX: Int,
	m_nRenderMode: Int,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_contents: Int,
	m_collideWithOwner: Int,
	m_iSignifierName: String,
	m_iName: String,
	m_scriptNameIndex: Int,
	m_instanceNameIndex: Int,
	m_holdUsePrompt: String,
	m_pressUsePrompt: String,
	m_phaseShiftFlags: Int,
	m_baseTakeDamage: Int,
	m_invulnerableToDamageCount: Int,
	m_attachmentLerpStartTime: Time,
	m_attachmentLerpEndTime: Time,
	m_attachmentLerpStartOrigin: Vector,
	m_attachmentLerpStartAngles: Vector,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_dissolveEffectEntityHandle: Int,
	m_usablePriority: Int,
	m_usableDistanceOverride: Float,
	m_usableFOV: Float,
	m_usePromptSize: Float,
	m_spottedByTeams: DataTable,
	m_firstChildEntityLink: Int,
	m_firstParentEntityLink: Int,
	m_realmsBitMask: BitMask,
}
```

### Offsets

```
DT_BaseEntity!0x0000 movetype
DT_BaseEntity!0x0000 movecollide
DT_BaseEntity!0x0000 predictable_id
DT_BaseEntity!0x0000 HighlightSettings
DT_BaseEntity!0x001c moveparent
DT_BaseEntity!0x0024 m_parentAttachmentIndex
DT_BaseEntity!0x0044 m_fEffects
DT_BaseEntity!0x0048 m_usableType
DT_BaseEntity!0x004c m_cellX
DT_BaseEntity!0x0050 m_cellY
DT_BaseEntity!0x0050 m_clrRender
DT_BaseEntity!0x0054 m_cellZ
DT_BaseEntity!0x0054 m_clIntensity
DT_BaseEntity!0x0058 m_localOrigin
DT_BaseEntity!0x0064 m_nModelIndex
DT_BaseEntity!0x0124 m_bossPlayer
DT_BaseEntity!0x0170 m_shieldHealth
DT_BaseEntity!0x0174 m_shieldHealthMax
DT_BaseEntity!0x03cc m_bIsSoundCodeControllerValueSet
DT_BaseEntity!0x03d0 m_flSoundCodeControllerValue
DT_BaseEntity!0x03d4 m_networkedFlags
DT_BaseEntity!0x0428 m_visibilityFlags
DT_BaseEntity!0x0430 m_iTeamNum
DT_BaseEntity!0x0438 m_teamMemberIndex
DT_BaseEntity!0x043c m_squadID
DT_BaseEntity!0x0440 m_grade
DT_BaseEntity!0x0444 m_ignorePredictedTriggerFlags
DT_BaseEntity!0x0448 m_passThroughFlags
DT_BaseEntity!0x044c m_passThroughThickness
DT_BaseEntity!0x0450 m_passThroughDirection
DT_BaseEntity!0x046c m_localAngles
DT_BaseEntity!0x0480 m_hOwnerEntity
DT_BaseEntity!0x0484 m_bRenderWithViewModels
DT_BaseEntity!0x0485 m_nRenderFX
DT_BaseEntity!0x0491 m_nRenderMode
DT_BaseEntity!0x0498 m_Collision
DT_BaseEntity!0x0518 m_CollisionGroup
DT_BaseEntity!0x051c m_contents
DT_BaseEntity!0x0520 m_collideWithOwner
DT_BaseEntity!0x0558 m_iSignifierName
DT_BaseEntity!0x0561 m_iName
DT_BaseEntity!0x0668 m_scriptNameIndex
DT_BaseEntity!0x066c m_instanceNameIndex
DT_BaseEntity!0x06f0 m_holdUsePrompt
DT_BaseEntity!0x06f8 m_pressUsePrompt
DT_BaseEntity!0x0790 m_phaseShiftFlags
DT_BaseEntity!0x0794 m_baseTakeDamage
DT_BaseEntity!0x0798 m_invulnerableToDamageCount
DT_BaseEntity!0x080c m_attachmentLerpStartTime
DT_BaseEntity!0x0810 m_attachmentLerpEndTime
DT_BaseEntity!0x0814 m_attachmentLerpStartOrigin
DT_BaseEntity!0x0820 m_attachmentLerpStartAngles
DT_BaseEntity!0x0838 m_parentAttachmentModel
DT_BaseEntity!0x0844 m_fadeDist
DT_BaseEntity!0x08f8 m_dissolveEffectEntityHandle
DT_BaseEntity!0x0908 m_usablePriority
DT_BaseEntity!0x090c m_usableDistanceOverride
DT_BaseEntity!0x0910 m_usableFOV
DT_BaseEntity!0x0914 m_usePromptSize
DT_BaseEntity!0x0928 m_spottedByTeams
DT_BaseEntity!0x0a20 m_firstChildEntityLink
DT_BaseEntity!0x0a24 m_firstParentEntityLink
DT_BaseEntity!0x0a28 m_realmsBitMask
```
</details>
<details>
<summary><code>class DT_BaseGrenade extends DT_Projectile</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_cloakEndTime: Float,
	m_cloakFadeInEndTime: Time,
	m_cloakFadeOutStartTime: Float,
	m_cloakFadeInDuration: Float,
	m_cloakFlickerAmount: Float,
	m_cloakFlickerEndTime: Time,
	m_baseTakeDamage: Int,
	m_invulnerableToDamageCount: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_doesExplode: Int,
	m_DmgRadius: Float,
	m_grenadeStatusFlags: Int,
	m_ziplineGrenadeExpectedEndPosition: Vector,
}
```

### Offsets

```
DT_BaseGrenade!0x001c moveparent
DT_BaseGrenade!0x0020 m_parentAttachmentType
DT_BaseGrenade!0x0024 m_parentAttachmentIndex
DT_BaseGrenade!0x019c m_cloakEndTime
DT_BaseGrenade!0x01a0 m_cloakFadeInEndTime
DT_BaseGrenade!0x01a4 m_cloakFadeOutStartTime
DT_BaseGrenade!0x01a8 m_cloakFadeInDuration
DT_BaseGrenade!0x01ac m_cloakFlickerAmount
DT_BaseGrenade!0x01b0 m_cloakFlickerEndTime
DT_BaseGrenade!0x0794 m_baseTakeDamage
DT_BaseGrenade!0x0798 m_invulnerableToDamageCount
DT_BaseGrenade!0x0834 m_parentAttachmentHitbox
DT_BaseGrenade!0x0838 m_parentAttachmentModel
DT_BaseGrenade!0x2c41 m_doesExplode
DT_BaseGrenade!0x2c44 m_DmgRadius
DT_BaseGrenade!0x2c68 m_grenadeStatusFlags
DT_BaseGrenade!0x2cd8 m_ziplineGrenadeExpectedEndPosition
```
</details>
<details>
<summary><code>class DT_BaseViewModel</code></summary>

```
{
	overlay_vars: DT_OverlayVars,
	m_animStartTime: Time,
	m_animOverlayIsActive: DataTable,
	m_animStartCycle: Float,
	m_animPlaybackRate: Float,
	m_animFrozen: Int,
	m_animOverlayStartTime: DataTable,
	m_animModelIndex: Int,
	m_animSequence: Int,
	m_nNewSequenceParity: Int,
	m_animOverlayStartCycle: DataTable,
	m_fEffects: Int,
	m_clrRender: Int,
	m_animOverlayPlaybackRate: DataTable,
	m_nModelIndex: Int,
	m_animOverlayModelIndex: DataTable,
	m_animOverlaySequence: DataTable,
	m_animOverlayWeight: DataTable,
	m_animOverlayOrder: DataTable,
	m_animOverlayAnimTime: DataTable,
	m_animOverlayFadeInDuration: DataTable,
	m_animOverlayFadeOutDuration: DataTable,
	m_nRenderMode: Int,
	m_nBody: Int,
	m_nResetEventsParity: Int,
	m_bSequenceFinished: Int,
	m_flModelScale: Float,
	m_overlayEventParity: DataTable,
	m_viewModelOwner: Int,
	m_projectileIsVisible: Int,
	m_bBlockEventLayer: Int,
	m_isAdsTransition: Int,
	m_hWeapon: Int,
	m_tracerAttachments: DataTable,
	m_tracerAttachmentsScoped: DataTable,
}
```

### Offsets

```
DT_BaseViewModel!0x0000 overlay_vars
DT_BaseViewModel!0x0008 m_animStartTime
DT_BaseViewModel!0x0008 m_animOverlayIsActive
DT_BaseViewModel!0x000c m_animStartCycle
DT_BaseViewModel!0x0010 m_animPlaybackRate
DT_BaseViewModel!0x0014 m_animFrozen
DT_BaseViewModel!0x0014 m_animOverlayStartTime
DT_BaseViewModel!0x0018 m_animModelIndex
DT_BaseViewModel!0x001c m_animSequence
DT_BaseViewModel!0x0020 m_nNewSequenceParity
DT_BaseViewModel!0x0038 m_animOverlayStartCycle
DT_BaseViewModel!0x0044 m_fEffects
DT_BaseViewModel!0x0050 m_clrRender
DT_BaseViewModel!0x005c m_animOverlayPlaybackRate
DT_BaseViewModel!0x0064 m_nModelIndex
DT_BaseViewModel!0x0080 m_animOverlayModelIndex
DT_BaseViewModel!0x00a4 m_animOverlaySequence
DT_BaseViewModel!0x00c8 m_animOverlayWeight
DT_BaseViewModel!0x00ec m_animOverlayOrder
DT_BaseViewModel!0x0110 m_animOverlayAnimTime
DT_BaseViewModel!0x0134 m_animOverlayFadeInDuration
DT_BaseViewModel!0x0158 m_animOverlayFadeOutDuration
DT_BaseViewModel!0x0491 m_nRenderMode
DT_BaseViewModel!0x0e90 m_nBody
DT_BaseViewModel!0x0e9c m_nResetEventsParity
DT_BaseViewModel!0x0f34 m_bSequenceFinished
DT_BaseViewModel!0x0f40 m_flModelScale
DT_BaseViewModel!0x1711 m_overlayEventParity
DT_BaseViewModel!0x19d8 m_viewModelOwner
DT_BaseViewModel!0x19dc m_projectileIsVisible
DT_BaseViewModel!0x1dc0 m_bBlockEventLayer
DT_BaseViewModel!0x1dc1 m_isAdsTransition
DT_BaseViewModel!0x1dc4 m_hWeapon
DT_BaseViewModel!0x1dc8 m_tracerAttachments
DT_BaseViewModel!0x1dd0 m_tracerAttachmentsScoped
```
</details>
<details>
<summary><code>class DT_BoneFollower</code></summary>

```
{
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_modelIndex: Int,
	m_boneIndex: Int,
}
```

### Offsets

```
DT_BoneFollower!0x004c m_cellX
DT_BoneFollower!0x0050 m_cellY
DT_BoneFollower!0x0054 m_cellZ
DT_BoneFollower!0x0058 m_localOrigin
DT_BoneFollower!0x0064 m_nModelIndex
DT_BoneFollower!0x03d4 m_networkedFlags
DT_BoneFollower!0x046c m_localAngles
DT_BoneFollower!0x0480 m_hOwnerEntity
DT_BoneFollower!0x0498 m_Collision
DT_BoneFollower!0x0518 m_CollisionGroup
DT_BoneFollower!0x0a40 m_modelIndex
DT_BoneFollower!0x0a44 m_boneIndex
```
</details>
<details>
<summary><code>class DT_BreakableSurface extends DT_BaseEntity</code></summary>

```
{
	m_nNumWide: Int,
	m_nNumHigh: Int,
	m_flPanelWidth: Float,
	m_flPanelHeight: Float,
	m_vNormal: Vector,
	m_vUp: Vector,
	m_vCorner: Vector,
	m_bIsBroken: Int,
	m_nSurfaceType: Int,
	m_RawPanelBitVec: DataTable,
}
```

### Offsets

```
DT_BreakableSurface!0x0a48 m_nNumWide
DT_BreakableSurface!0x0a4c m_nNumHigh
DT_BreakableSurface!0x0a50 m_flPanelWidth
DT_BreakableSurface!0x0a54 m_flPanelHeight
DT_BreakableSurface!0x0a58 m_vNormal
DT_BreakableSurface!0x0a64 m_vUp
DT_BreakableSurface!0x0a7c m_vCorner
DT_BreakableSurface!0x0a88 m_bIsBroken
DT_BreakableSurface!0x0a8c m_nSurfaceType
DT_BreakableSurface!0x0ac8 m_RawPanelBitVec
```
</details>
<details>
<summary><code>class DT_CPropDoor</code></summary>

```
{
	HighlightSettings: DT_HighlightSettings,
	m_fEffects: Int,
	m_usableType: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_localAngles: Vector,
	m_nSkin: Int,
	m_skinMod: Int,
	m_closedAngle: Float,
	m_angle: Float,
	m_startAngle: Float,
	m_startAngleVel: Float,
	m_startMoveTime: Time,
	m_isLocked: Int,
	m_oppositeDoor: Int,
	m_interactingPlayer: Int,
	m_interactingPlayerWantsOpen: Int,
}
```

### Offsets

```
DT_CPropDoor!0x0000 HighlightSettings
DT_CPropDoor!0x0044 m_fEffects
DT_CPropDoor!0x0048 m_usableType
DT_CPropDoor!0x004c m_cellX
DT_CPropDoor!0x0050 m_cellY
DT_CPropDoor!0x0054 m_cellZ
DT_CPropDoor!0x0058 m_localOrigin
DT_CPropDoor!0x0064 m_nModelIndex
DT_CPropDoor!0x03d4 m_networkedFlags
DT_CPropDoor!0x046c m_localAngles
DT_CPropDoor!0x0e88 m_nSkin
DT_CPropDoor!0x0e8c m_skinMod
DT_CPropDoor!0x1670 m_closedAngle
DT_CPropDoor!0x1674 m_angle
DT_CPropDoor!0x1678 m_startAngle
DT_CPropDoor!0x167c m_startAngleVel
DT_CPropDoor!0x1680 m_startMoveTime
DT_CPropDoor!0x1684 m_isLocked
DT_CPropDoor!0x1688 m_oppositeDoor
DT_CPropDoor!0x16d8 m_interactingPlayer
DT_CPropDoor!0x16dc m_interactingPlayerWantsOpen
```
</details>
<details>
<summary><code>class DT_CascadeLight extends DT_BaseEntity</code></summary>

```
{
	m_shadowDirection: Vector,
	m_envLightShadowDirection: Vector,
	m_bEnabled: Int,
	m_bEnableShadows: Int,
	m_LightColor: Int,
	m_cloudMaskName: String,
	m_cloudOffset: Vector,
	m_cloudScale: Float,
}
```

### Offsets

```
DT_CascadeLight!0x0a40 m_shadowDirection
DT_CascadeLight!0x0a58 m_envLightShadowDirection
DT_CascadeLight!0x0a6c m_bEnabled
DT_CascadeLight!0x0a6d m_bEnableShadows
DT_CascadeLight!0x0a6f m_LightColor
DT_CascadeLight!0x0a73 m_cloudMaskName
DT_CascadeLight!0x0b78 m_cloudOffset
DT_CascadeLight!0x0b84 m_cloudScale
```
</details>
<details>
<summary><code>class DT_CollisionProperty</code></summary>

```
{
	m_vecMins: Vector,
	m_vecMaxs: Vector,
	m_usSolidFlags: Int,
	m_nSolidType: Int,
	m_triggerBloat: Int,
	m_collisionDetailLevel: Int,
	m_nSurroundType: Int,
	m_vecSpecifiedSurroundingMins: Vector,
	m_vecSpecifiedSurroundingMaxs: Vector,
}
```

### Offsets

```
DT_CollisionProperty!0x0010 m_vecMins
DT_CollisionProperty!0x001c m_vecMaxs
DT_CollisionProperty!0x0028 m_usSolidFlags
DT_CollisionProperty!0x002c m_nSolidType
DT_CollisionProperty!0x002d m_triggerBloat
DT_CollisionProperty!0x002e m_collisionDetailLevel
DT_CollisionProperty!0x003c m_nSurroundType
DT_CollisionProperty!0x0048 m_vecSpecifiedSurroundingMins
DT_CollisionProperty!0x0054 m_vecSpecifiedSurroundingMaxs
```
</details>
<details>
<summary><code>class DT_ColorCorrection extends DT_BaseEntity</code></summary>

```
{
	m_hOwnerEntity: Int,
	m_localOrigin: Vector,
	m_MinFalloff: Float,
	m_MaxFalloff: Float,
	m_flFadeInDuration: Float,
	m_flFadeOutDuration: Float,
	m_flMaxWeight: Float,
	m_flCurWeight: Float,
	m_netLookupFilename: String,
	m_bEnabled: Int,
	m_bMaster: Int,
	m_bClientSide: Int,
	m_bExclusive: Int,
}
```

### Offsets

```
DT_ColorCorrection!0x0480 m_hOwnerEntity
DT_ColorCorrection!0x0a40 m_localOrigin
DT_ColorCorrection!0x0a4c m_MinFalloff
DT_ColorCorrection!0x0a50 m_MaxFalloff
DT_ColorCorrection!0x0a54 m_flFadeInDuration
DT_ColorCorrection!0x0a58 m_flFadeOutDuration
DT_ColorCorrection!0x0a5c m_flMaxWeight
DT_ColorCorrection!0x0a60 m_flCurWeight
DT_ColorCorrection!0x0a64 m_netLookupFilename
DT_ColorCorrection!0x0b68 m_bEnabled
DT_ColorCorrection!0x0b69 m_bMaster
DT_ColorCorrection!0x0b6a m_bClientSide
DT_ColorCorrection!0x0b6b m_bExclusive
```
</details>
<details>
<summary><code>class DT_CurrentData_LocalPlayer</code></summary>

```
{
	m_viewConeAngleMin: Vector,
	m_viewConeAngleMax: Vector,
	m_stepSmoothingOffset: Vector,
	m_duckTransitionRemainderMsec: Int,
	m_vecPunchBase_Angle: Vector,
	m_vecPunchBase_AngleVel: Vector,
	m_vecPunchWeapon_Angle: Vector,
	m_vecPunchWeapon_AngleVel: Vector,
	m_pushedFixedPointOffset: DataTable,
	m_pushedFixedPointOffsetReplayCompensated: DataTable,
	m_localGravityRotation: Rotation,
}
```

### Offsets

```
DT_CurrentData_LocalPlayer!0x0000 m_viewConeAngleMin
DT_CurrentData_LocalPlayer!0x000c m_viewConeAngleMax
DT_CurrentData_LocalPlayer!0x0018 m_stepSmoothingOffset
DT_CurrentData_LocalPlayer!0x0024 m_duckTransitionRemainderMsec
DT_CurrentData_LocalPlayer!0x0028 m_vecPunchBase_Angle
DT_CurrentData_LocalPlayer!0x0034 m_vecPunchBase_AngleVel
DT_CurrentData_LocalPlayer!0x0040 m_vecPunchWeapon_Angle
DT_CurrentData_LocalPlayer!0x004c m_vecPunchWeapon_AngleVel
DT_CurrentData_LocalPlayer!0x0058 m_pushedFixedPointOffset
DT_CurrentData_LocalPlayer!0x0064 m_pushedFixedPointOffsetReplayCompensated
DT_CurrentData_LocalPlayer!0x007c m_localGravityRotation
```
</details>
<details>
<summary><code>class DT_CurrentData_Player</code></summary>

```
{
	m_flHullHeight: Float,
	m_angEyeAngles.x: Float,
	m_angEyeAngles.y: Float,
	m_traversalAnimProgress: Float,
	m_sprintTiltFrac: Float,
	m_ammoPoolCount: DataTable,
}
```

### Offsets

```
DT_CurrentData_Player!0x0014 m_flHullHeight
DT_CurrentData_Player!0x0018 m_angEyeAngles.x
DT_CurrentData_Player!0x001c m_angEyeAngles.y
DT_CurrentData_Player!0x0024 m_traversalAnimProgress
DT_CurrentData_Player!0x0028 m_sprintTiltFrac
DT_CurrentData_Player!0x002c m_ammoPoolCount
```
</details>
<details>
<summary><code>class DT_DoorMover</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_usableType: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_vecAngVelocity: Vector,
	m_networkedFlags: Int,
	m_vecVelocity: Vector,
	m_localAngles: Vector,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_iSignifierName: String,
	m_scriptNameIndex: Int,
	m_holdUsePrompt: String,
	m_pressUsePrompt: String,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_usablePriority: Int,
	m_usableDistanceOverride: Float,
	m_usableFOV: Float,
	m_usePromptSize: Float,
	m_doorFlags: Int,
}
```

### Offsets

```
DT_DoorMover!0x001c moveparent
DT_DoorMover!0x0020 m_parentAttachmentType
DT_DoorMover!0x0024 m_parentAttachmentIndex
DT_DoorMover!0x0044 m_fEffects
DT_DoorMover!0x0048 m_usableType
DT_DoorMover!0x004c m_cellX
DT_DoorMover!0x0050 m_cellY
DT_DoorMover!0x0054 m_cellZ
DT_DoorMover!0x0058 m_localOrigin
DT_DoorMover!0x0064 m_nModelIndex
DT_DoorMover!0x0128 m_vecAngVelocity
DT_DoorMover!0x03d4 m_networkedFlags
DT_DoorMover!0x0460 m_vecVelocity
DT_DoorMover!0x046c m_localAngles
DT_DoorMover!0x0498 m_Collision
DT_DoorMover!0x0518 m_CollisionGroup
DT_DoorMover!0x0558 m_iSignifierName
DT_DoorMover!0x0668 m_scriptNameIndex
DT_DoorMover!0x06f0 m_holdUsePrompt
DT_DoorMover!0x06f8 m_pressUsePrompt
DT_DoorMover!0x0834 m_parentAttachmentHitbox
DT_DoorMover!0x0838 m_parentAttachmentModel
DT_DoorMover!0x0844 m_fadeDist
DT_DoorMover!0x0908 m_usablePriority
DT_DoorMover!0x090c m_usableDistanceOverride
DT_DoorMover!0x0910 m_usableFOV
DT_DoorMover!0x0914 m_usePromptSize
DT_DoorMover!0x1880 m_doorFlags
```
</details>
<details>
<summary><code>class DT_DynamicLight extends DT_BaseEntity</code></summary>

```
{
	m_Flags: Int,
	m_LightStyle: Int,
	m_Radius: Float,
	m_Exponent: Int,
	m_InnerAngle: Float,
	m_OuterAngle: Float,
	m_SpotRadius: Float,
}
```

### Offsets

```
DT_DynamicLight!0x0a40 m_Flags
DT_DynamicLight!0x0a41 m_LightStyle
DT_DynamicLight!0x0a44 m_Radius
DT_DynamicLight!0x0a48 m_Exponent
DT_DynamicLight!0x0a4c m_InnerAngle
DT_DynamicLight!0x0a50 m_OuterAngle
DT_DynamicLight!0x0a54 m_SpotRadius
```
</details>
<details>
<summary><code>class DT_DynamicProp extends DT_BreakableProp</code></summary>

```
{
	m_iTeamNum: Int,
	m_lifeState: Int,
	m_bUseHitboxesForRenderBox: Int,
	m_bAnimateInStaticShadow: Int,
	m_wantsScopeHighlight: Int,
}
```

### Offsets

```
DT_DynamicProp!0x0430 m_iTeamNum
DT_DynamicProp!0x0770 m_lifeState
DT_DynamicProp!0x1601 m_bUseHitboxesForRenderBox
DT_DynamicProp!0x1602 m_bAnimateInStaticShadow
DT_DynamicProp!0x1603 m_wantsScopeHighlight
```
</details>
<details>
<summary><code>class DT_DynamicPropLightweight</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_localAngles: Vector,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_nSkin: Int,
	m_skinMod: Int,
}
```

### Offsets

```
DT_DynamicPropLightweight!0x001c moveparent
DT_DynamicPropLightweight!0x0020 m_parentAttachmentType
DT_DynamicPropLightweight!0x0024 m_parentAttachmentIndex
DT_DynamicPropLightweight!0x0044 m_fEffects
DT_DynamicPropLightweight!0x004c m_cellX
DT_DynamicPropLightweight!0x0050 m_cellY
DT_DynamicPropLightweight!0x0054 m_cellZ
DT_DynamicPropLightweight!0x0058 m_localOrigin
DT_DynamicPropLightweight!0x0064 m_nModelIndex
DT_DynamicPropLightweight!0x03d4 m_networkedFlags
DT_DynamicPropLightweight!0x0428 m_visibilityFlags
DT_DynamicPropLightweight!0x046c m_localAngles
DT_DynamicPropLightweight!0x0498 m_Collision
DT_DynamicPropLightweight!0x0518 m_CollisionGroup
DT_DynamicPropLightweight!0x0838 m_parentAttachmentModel
DT_DynamicPropLightweight!0x0844 m_fadeDist
DT_DynamicPropLightweight!0x0e88 m_nSkin
DT_DynamicPropLightweight!0x0e8c m_skinMod
```
</details>
<details>
<summary><code>class DT_EffectData</code></summary>

```
{
	m_vOrigin.x: Float,
	m_vOrigin.y: Float,
	m_vOrigin.z: Float,
	m_vStart.x: Float,
	m_vStart.y: Float,
	m_vStart.z: Float,
	m_vNormal: Vector,
	m_vAngles: Vector,
	m_effectFlags: Int,
	m_effectEntHandle: Int,
	m_otherEntHandle: Int,
	m_flScale: Float,
	m_flMagnitude: Float,
	m_flRadius: Float,
	m_nAttachmentIndex: Int,
	m_attachmentIndexForViewmodel: Int,
	m_nSurfaceProp: Int,
	m_nDamageType: Int,
	m_nOtherEntIndex: Int,
	m_sharedInt32_A: Int,
	m_sharedInt32_B: Int,
	m_iImpactEffectTableIndex: Int,
	m_nColor: Int,
	m_persistentWeaponEffect: Int,
	m_iEffectName: Int,
}
```

### Offsets

```
DT_EffectData!0x0000 m_vOrigin.x
DT_EffectData!0x0004 m_vOrigin.y
DT_EffectData!0x0008 m_vOrigin.z
DT_EffectData!0x000c m_vStart.x
DT_EffectData!0x0010 m_vStart.y
DT_EffectData!0x0014 m_vStart.z
DT_EffectData!0x0018 m_vNormal
DT_EffectData!0x0024 m_vAngles
DT_EffectData!0x0030 m_effectFlags
DT_EffectData!0x0050 m_effectEntHandle
DT_EffectData!0x0054 m_otherEntHandle
DT_EffectData!0x0058 m_flScale
DT_EffectData!0x005c m_flMagnitude
DT_EffectData!0x0060 m_flRadius
DT_EffectData!0x0064 m_nAttachmentIndex
DT_EffectData!0x0068 m_attachmentIndexForViewmodel
DT_EffectData!0x006c m_nSurfaceProp
DT_EffectData!0x0070 m_nDamageType
DT_EffectData!0x0074 m_nOtherEntIndex
DT_EffectData!0x007c m_sharedInt32_A
DT_EffectData!0x0080 m_sharedInt32_B
DT_EffectData!0x0084 m_iImpactEffectTableIndex
DT_EffectData!0x0088 m_nColor
DT_EffectData!0x009c m_persistentWeaponEffect
DT_EffectData!0x00a0 m_iEffectName
```
</details>
<details>
<summary><code>class DT_EntityDissolve extends DT_BaseEntity</code></summary>

```
{
	m_flStartTime: Time,
	m_flFadeStart: Float,
	m_flFadeLength: Float,
	m_nDissolveType: Int,
	m_isLethal: Int,
}
```

### Offsets

```
DT_EntityDissolve!0x0a48 m_flStartTime
DT_EntityDissolve!0x0a4c m_flFadeStart
DT_EntityDissolve!0x0a50 m_flFadeLength
DT_EntityDissolve!0x0a54 m_nDissolveType
DT_EntityDissolve!0x0a58 m_isLethal
```
</details>
<details>
<summary><code>class DT_EntityLinkPage</code></summary>

```
{
	pageIndex: Int,
	next: DataTable,
	entity: DataTable,
}
```

### Offsets

```
DT_EntityLinkPage!0x0a40 pageIndex
DT_EntityLinkPage!0x0a44 next
DT_EntityLinkPage!0x0e44 entity
```
</details>
<details>
<summary><code>class DT_EnvWindShared</code></summary>

```
{
	m_flStartTime: Time,
	m_iWindSeed: Int,
	m_iMinWind: Int,
	m_iMaxWind: Int,
	m_iMinGust: Int,
	m_iMaxGust: Int,
	m_flMinGustDelay: Float,
	m_flMaxGustDelay: Float,
	m_flGustDuration: Float,
	m_iGustDirChange: Int,
	m_iInitialWindDir: Int,
	m_flInitialWindSpeed: Float,
}
```

### Offsets

```
DT_EnvWindShared!0x0008 m_flStartTime
DT_EnvWindShared!0x000c m_iWindSeed
DT_EnvWindShared!0x0010 m_iMinWind
DT_EnvWindShared!0x0014 m_iMaxWind
DT_EnvWindShared!0x001c m_iMinGust
DT_EnvWindShared!0x0020 m_iMaxGust
DT_EnvWindShared!0x0024 m_flMinGustDelay
DT_EnvWindShared!0x0028 m_flMaxGustDelay
DT_EnvWindShared!0x002c m_flGustDuration
DT_EnvWindShared!0x0030 m_iGustDirChange
DT_EnvWindShared!0x0070 m_iInitialWindDir
DT_EnvWindShared!0x0074 m_flInitialWindSpeed
```
</details>
<details>
<summary><code>class DT_FuncBrushLightweight</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_localAngles: Vector,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
}
```

### Offsets

```
DT_FuncBrushLightweight!0x001c moveparent
DT_FuncBrushLightweight!0x0020 m_parentAttachmentType
DT_FuncBrushLightweight!0x0024 m_parentAttachmentIndex
DT_FuncBrushLightweight!0x004c m_cellX
DT_FuncBrushLightweight!0x0050 m_cellY
DT_FuncBrushLightweight!0x0054 m_cellZ
DT_FuncBrushLightweight!0x0058 m_localOrigin
DT_FuncBrushLightweight!0x0064 m_nModelIndex
DT_FuncBrushLightweight!0x03d4 m_networkedFlags
DT_FuncBrushLightweight!0x0428 m_visibilityFlags
DT_FuncBrushLightweight!0x046c m_localAngles
DT_FuncBrushLightweight!0x0498 m_Collision
DT_FuncBrushLightweight!0x0518 m_CollisionGroup
DT_FuncBrushLightweight!0x0834 m_parentAttachmentHitbox
DT_FuncBrushLightweight!0x0838 m_parentAttachmentModel
```
</details>
<details>
<summary><code>class DT_GlobalNonRewinding</code></summary>

```
{
	m_playerObserver: DataTable,
	m_playerMiscData: DataTable,
}
```

### Offsets

```
DT_GlobalNonRewinding!0x0a40 m_playerObserver
DT_GlobalNonRewinding!0x0e40 m_playerMiscData
```
</details>
<details>
<summary><code>class DT_GrappleData</code></summary>

```
{
	m_grapplePoints: Array,
	m_grappleVel: Vector,
	m_grapplePoints[0]: Vector,
	m_grapplePointCount: Int,
	m_grappleAttached: Int,
	m_grapplePulling: Int,
	m_grappleSwinging: Int,
	m_grappleRetracting: Int,
	m_grappleForcedRetracting: Int,
	m_grappleGracePeriodFinished: Int,
	m_grappleUsedPower: Float,
	m_grappleActivateTime: Time,
	m_grapplePullTime: Time,
	m_grappleAttachTime: Time,
	m_grappleDetachTime: Time,
	m_grappleMeleeTarget: Int,
	m_grappleAutoAimTarget: Int,
	m_grappleHasGoodVelocity: Int,
	m_grappleLastGoodVelocityTime: Time,
	m_grappleSwingDetachLowSpeed: Float,
	m_grappleSwingHoldTime: Time,
}
```

### Offsets

```
DT_GrappleData!0x0000 m_grapplePoints
DT_GrappleData!0x0008 m_grappleVel
DT_GrappleData!0x0014 m_grapplePoints[0]
DT_GrappleData!0x0044 m_grapplePointCount
DT_GrappleData!0x0048 m_grappleAttached
DT_GrappleData!0x0049 m_grapplePulling
DT_GrappleData!0x004a m_grappleSwinging
DT_GrappleData!0x004b m_grappleRetracting
DT_GrappleData!0x004c m_grappleForcedRetracting
DT_GrappleData!0x004d m_grappleGracePeriodFinished
DT_GrappleData!0x0050 m_grappleUsedPower
DT_GrappleData!0x0054 m_grappleActivateTime
DT_GrappleData!0x0058 m_grapplePullTime
DT_GrappleData!0x005c m_grappleAttachTime
DT_GrappleData!0x0060 m_grappleDetachTime
DT_GrappleData!0x0064 m_grappleMeleeTarget
DT_GrappleData!0x0068 m_grappleAutoAimTarget
DT_GrappleData!0x006c m_grappleHasGoodVelocity
DT_GrappleData!0x0070 m_grappleLastGoodVelocityTime
DT_GrappleData!0x0074 m_grappleSwingDetachLowSpeed
DT_GrappleData!0x0078 m_grappleSwingHoldTime
```
</details>
<details>
<summary><code>class DT_GrappleHook</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_visibilityFlags: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_parentAttachmentHitbox: Int,
	m_realmsBitMask: BitMask,
	m_grappleZipline: Int,
}
```

### Offsets

```
DT_GrappleHook!0x001c moveparent
DT_GrappleHook!0x0020 m_parentAttachmentType
DT_GrappleHook!0x0024 m_parentAttachmentIndex
DT_GrappleHook!0x004c m_cellX
DT_GrappleHook!0x0050 m_cellY
DT_GrappleHook!0x0054 m_cellZ
DT_GrappleHook!0x0058 m_localOrigin
DT_GrappleHook!0x0064 m_nModelIndex
DT_GrappleHook!0x0428 m_visibilityFlags
DT_GrappleHook!0x046c m_localAngles
DT_GrappleHook!0x0480 m_hOwnerEntity
DT_GrappleHook!0x0834 m_parentAttachmentHitbox
DT_GrappleHook!0x0a28 m_realmsBitMask
DT_GrappleHook!0x1600 m_grappleZipline
```
</details>
<details>
<summary><code>class DT_HardPointEntity</code></summary>

```
{
	m_localOrigin: Vector,
	m_iTeamNum: Int,
	m_minimapData: DT_MinimapBaseEntityData,
	m_state: Int,
	m_estimatedCaptureTime: Float,
	m_progressRefPoint: Float,
	m_teamMilitiaAICount: Int,
	m_teamIMCAICount: Int,
	m_teamMilitiaPlayerCount: Int,
	m_teamIMCPlayerCount: Int,
	m_teamMilitiaPlayerTitanCount: Int,
	m_teamIMCPlayerTitanCount: Int,
	m_hardpointID: Int,
	m_terminal: Int,
}
```

### Offsets

```
DT_HardPointEntity!0x0004 m_localOrigin
DT_HardPointEntity!0x0430 m_iTeamNum
DT_HardPointEntity!0x0948 m_minimapData
DT_HardPointEntity!0x0a44 m_state
DT_HardPointEntity!0x0a48 m_estimatedCaptureTime
DT_HardPointEntity!0x0a4c m_progressRefPoint
DT_HardPointEntity!0x0a50 m_teamMilitiaAICount
DT_HardPointEntity!0x0a54 m_teamIMCAICount
DT_HardPointEntity!0x0a58 m_teamMilitiaPlayerCount
DT_HardPointEntity!0x0a5c m_teamIMCPlayerCount
DT_HardPointEntity!0x0a60 m_teamMilitiaPlayerTitanCount
DT_HardPointEntity!0x0a64 m_teamIMCPlayerTitanCount
DT_HardPointEntity!0x0a68 m_hardpointID
DT_HardPointEntity!0x0a70 m_terminal
```
</details>
<details>
<summary><code>class DT_HighlightSettings</code></summary>

```
{
	m_highlightParams: DataTable,
	m_highlightFunctionBits: DataTable,
	m_highlightServerFadeBases: DataTable,
	m_highlightServerFadeStartTimes: DataTable,
	m_highlightServerFadeEndTimes: DataTable,
	m_highlightServerContextID: Int,
	m_highlightTeamBits: Int,
}
```

### Offsets

```
DT_HighlightSettings!0x01b8 m_highlightParams
DT_HighlightSettings!0x0290 m_highlightFunctionBits
DT_HighlightSettings!0x02d8 m_highlightServerFadeBases
DT_HighlightSettings!0x02e0 m_highlightServerFadeStartTimes
DT_HighlightSettings!0x02e8 m_highlightServerFadeEndTimes
DT_HighlightSettings!0x0328 m_highlightServerContextID
DT_HighlightSettings!0x0334 m_highlightTeamBits
```
</details>
<details>
<summary><code>class DT_ImportantOnEntSound extends DT_BaseEntity</code></summary>

```
{
	m_networkTableSoundID: Int,
	m_hAttachedToEntity: Int,
	m_beginTime: Time,
	m_hSuppressedClient: Int,
	m_milesSignal: Int,
}
```

### Offsets

```
DT_ImportantOnEntSound!0x0a40 m_networkTableSoundID
DT_ImportantOnEntSound!0x0a44 m_hAttachedToEntity
DT_ImportantOnEntSound!0x0a48 m_beginTime
DT_ImportantOnEntSound!0x0a4c m_hSuppressedClient
DT_ImportantOnEntSound!0x0a50 m_milesSignal
```
</details>
<details>
<summary><code>class DT_InfoPlacementHelper</code></summary>

```
{
	m_localOrigin: Vector,
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_localAngles: Vector,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
}
```

### Offsets

```
DT_InfoPlacementHelper!0x0004 m_localOrigin
DT_InfoPlacementHelper!0x001c moveparent
DT_InfoPlacementHelper!0x0020 m_parentAttachmentType
DT_InfoPlacementHelper!0x0024 m_parentAttachmentIndex
DT_InfoPlacementHelper!0x046c m_localAngles
DT_InfoPlacementHelper!0x0834 m_parentAttachmentHitbox
DT_InfoPlacementHelper!0x0838 m_parentAttachmentModel
```
</details>
<details>
<summary><code>class DT_InfoTarget</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_bIsSoundCodeControllerValueSet: Int,
	m_flSoundCodeControllerValue: Float,
	m_iTeamNum: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_iSignifierName: String,
	m_iName: String,
	m_scriptNameIndex: Int,
	m_instanceNameIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_firstChildEntityLink: Int,
	m_firstParentEntityLink: Int,
}
```

### Offsets

```
DT_InfoTarget!0x001c moveparent
DT_InfoTarget!0x0020 m_parentAttachmentType
DT_InfoTarget!0x0024 m_parentAttachmentIndex
DT_InfoTarget!0x004c m_cellX
DT_InfoTarget!0x0050 m_cellY
DT_InfoTarget!0x0054 m_cellZ
DT_InfoTarget!0x0058 m_localOrigin
DT_InfoTarget!0x03cc m_bIsSoundCodeControllerValueSet
DT_InfoTarget!0x03d0 m_flSoundCodeControllerValue
DT_InfoTarget!0x0430 m_iTeamNum
DT_InfoTarget!0x046c m_localAngles
DT_InfoTarget!0x0480 m_hOwnerEntity
DT_InfoTarget!0x0558 m_iSignifierName
DT_InfoTarget!0x0561 m_iName
DT_InfoTarget!0x0668 m_scriptNameIndex
DT_InfoTarget!0x066c m_instanceNameIndex
DT_InfoTarget!0x0834 m_parentAttachmentHitbox
DT_InfoTarget!0x0838 m_parentAttachmentModel
DT_InfoTarget!0x0a20 m_firstChildEntityLink
DT_InfoTarget!0x0a24 m_firstParentEntityLink
```
</details>
<details>
<summary><code>class DT_Local</code></summary>

```
{
	m_airMoveBlockPlanes: Array,
	m_iHideHUD: Int,
	m_superJumpsUsed: Int,
	m_jumpedOffRodeo: Int,
	m_jumpPressTime: Time,
	m_jetpackActivateTime: Time,
	m_jetpackDeactivateTime: Time,
	m_flSuitPower: Float,
	m_flSuitJumpPower: Float,
	m_flSuitGrapplePower: Float,
	m_flFallVelocity: Float,
	m_flStepSize: Float,
	m_airSlowMoFrac: Float,
	predictableFlags: Int,
	m_bitsActiveDevices: Int,
	m_forceStance: Int,
	m_duckToggleOn: Int,
	m_bDrawViewmodel: Int,
	m_bAllowAutoMovement: Int,
	m_accelScale: Float,
	m_powerRegenRateScale: Float,
	m_dodgePowerDelayScale: Float,
	m_hSkyCamera: Int,
	m_skybox3d.scale: Int,
	m_skybox3d.useWorldFog: Int,
	m_skybox3d.fog.botAlt: Float,
	m_skybox3d.fog.topAlt: Float,
	m_skybox3d.fog.halfDistBot: Float,
	m_skybox3d.fog.halfDistTop: Float,
	m_skybox3d.fog.distColorStr: Float,
	m_skybox3d.fog.dirColorStr: Float,
	m_skybox3d.fog.distOffset: Float,
	m_skybox3d.fog.densityScale: Float,
	m_skybox3d.fog.halfAngleDeg: Float,
	m_skybox3d.fog.HDRColorScale: Float,
	m_skybox3d.fog.distColor: Int,
	m_skybox3d.fog.dirColor: Int,
	m_skybox3d.fog.direction: Vector,
	m_skybox3d.fog.enable: Int,
	m_audio.localSound[0]: Vector,
	m_audio.localSound[1]: Vector,
	m_audio.localSound[2]: Vector,
	m_audio.localSound[3]: Vector,
	m_audio.localSound[4]: Vector,
	m_audio.localSound[5]: Vector,
	m_audio.localSound[6]: Vector,
	m_audio.localSound[7]: Vector,
	m_audio.soundscapeIndex: Int,
	m_audio.localBits: Int,
	m_audio.entIndex: Int,
	m_animNearZ: Float,
	lastAttacker: Int,
	attackedCount: Int,
	m_airMoveBlockPlanes[0]: Vector,
	m_airMoveBlockPlaneTime: Time,
	m_airMoveBlockPlaneCount: Int,
	m_queuedMeleePressTime: Time,
	m_queuedGrappleMeleeTime: Time,
	m_disableMeleeUntilRelease: Int,
	m_meleePressTime: Time,
	m_meleeDisabledCounter: Int,
	m_meleeInputIndex: Int,
	m_trackedChildProjectileCount: Int,
	m_oneHandedWeaponUsage: Int,
	m_flCockpitEntryTime: Time,
	m_ejectStartTime: Time,
	m_disembarkStartTime: Time,
	m_hotDropImpactTime: Time,
	m_outOfBoundsDeadTime: Time,
	m_objectiveIndex: Int,
	m_objectiveEntity: Int,
	m_objectiveEndTime: Time,
	m_cinematicEventFlags: Int,
	m_forcedDialogueOnly: Int,
	m_titanBuildTime: Time,
	m_titanBubbleShieldTime: Time,
	m_titanEmbarkEnabled: Int,
	m_titanDisembarkEnabled: Int,
	m_voicePackIndex: Int,
	m_playerAnimStationaryGoalFeetYaw: Float,
	m_playerAnimJumping: Int,
	m_playerAnimJumpStartTime: Time,
	m_playerAnimFirstJumpFrame: Int,
	m_playerAnimDodging: Int,
	m_playerAnimJumpActivity: Int,
	m_playerAnimLanding: Int,
	m_playerAnimShouldLand: Int,
	m_playerAnimLandStartTime: Time,
	m_playerAnimInAirWalk: Int,
	m_playerAnimPrevFrameSequenceMotionYaw: Float,
	m_playerAnimMeleeParity: Int,
	m_playerAnimMeleeStartTime: Time,
	m_playerLocalGravityBlendStartRotation: Rotation,
	m_playerLocalGravityBlendEndRotation: Rotation,
	m_playerLocalGravityBlendEndDirection: Vector,
	m_playerLocalGravityBlendStartTime: Time,
	m_playerLocalGravityBlendEndTime: Time,
	m_playerLocalGravityBlendStrength: Float,
	m_playerLocalGravityStrength: Float,
	m_playerLocalGravityType: Int,
	m_playerLocalGravityPoint: Vector,
	m_playerLocalGravityLineStart: Vector,
	m_playerLocalGravityLineEnd: Vector,
	m_playerLocalGravityEntity: Int,
	m_playerLocalGravityLineStartEntity: Int,
	m_playerLocalGravityLineEndEntity: Int,
	m_playerFloatLookStartTime: Time,
	m_playerFloatLookEndTime: Time,
	m_wallrunLatestFloorHeight: Float,
	m_wallrunFromJetpack: Int,
	m_groundNormal: Vector,
	m_continuousUseBlocked: Int,
	m_useEnt: Int,
}
```

### Offsets

```
DT_Local!0x0000 m_airMoveBlockPlanes
DT_Local!0x0014 m_iHideHUD
DT_Local!0x0018 m_superJumpsUsed
DT_Local!0x001c m_jumpedOffRodeo
DT_Local!0x0020 m_jumpPressTime
DT_Local!0x0024 m_jetpackActivateTime
DT_Local!0x0028 m_jetpackDeactivateTime
DT_Local!0x002c m_flSuitPower
DT_Local!0x0030 m_flSuitJumpPower
DT_Local!0x0034 m_flSuitGrapplePower
DT_Local!0x0038 m_flFallVelocity
DT_Local!0x003c m_flStepSize
DT_Local!0x0040 m_airSlowMoFrac
DT_Local!0x0044 predictableFlags
DT_Local!0x0048 m_bitsActiveDevices
DT_Local!0x004c m_forceStance
DT_Local!0x0050 m_duckToggleOn
DT_Local!0x0051 m_bDrawViewmodel
DT_Local!0x0052 m_bAllowAutoMovement
DT_Local!0x0054 m_accelScale
DT_Local!0x0058 m_powerRegenRateScale
DT_Local!0x005c m_dodgePowerDelayScale
DT_Local!0x0074 m_hSkyCamera
DT_Local!0x0078 m_skybox3d.scale
DT_Local!0x007c m_skybox3d.useWorldFog
DT_Local!0x0080 m_skybox3d.fog.botAlt
DT_Local!0x0084 m_skybox3d.fog.topAlt
DT_Local!0x0088 m_skybox3d.fog.halfDistBot
DT_Local!0x008c m_skybox3d.fog.halfDistTop
DT_Local!0x0090 m_skybox3d.fog.distColorStr
DT_Local!0x0094 m_skybox3d.fog.dirColorStr
DT_Local!0x0098 m_skybox3d.fog.distOffset
DT_Local!0x009c m_skybox3d.fog.densityScale
DT_Local!0x00a0 m_skybox3d.fog.halfAngleDeg
DT_Local!0x00a4 m_skybox3d.fog.HDRColorScale
DT_Local!0x00a8 m_skybox3d.fog.distColor
DT_Local!0x00ac m_skybox3d.fog.dirColor
DT_Local!0x00b0 m_skybox3d.fog.direction
DT_Local!0x00c1 m_skybox3d.fog.enable
DT_Local!0x00c8 m_audio.localSound[0]
DT_Local!0x00d4 m_audio.localSound[1]
DT_Local!0x00e0 m_audio.localSound[2]
DT_Local!0x00ec m_audio.localSound[3]
DT_Local!0x00f8 m_audio.localSound[4]
DT_Local!0x0104 m_audio.localSound[5]
DT_Local!0x0110 m_audio.localSound[6]
DT_Local!0x011c m_audio.localSound[7]
DT_Local!0x0128 m_audio.soundscapeIndex
DT_Local!0x012c m_audio.localBits
DT_Local!0x0130 m_audio.entIndex
DT_Local!0x014c m_animNearZ
DT_Local!0x0150 lastAttacker
DT_Local!0x0154 attackedCount
DT_Local!0x0180 m_airMoveBlockPlanes[0]
DT_Local!0x0198 m_airMoveBlockPlaneTime
DT_Local!0x019c m_airMoveBlockPlaneCount
DT_Local!0x01a0 m_queuedMeleePressTime
DT_Local!0x01a4 m_queuedGrappleMeleeTime
DT_Local!0x01a9 m_disableMeleeUntilRelease
DT_Local!0x01ac m_meleePressTime
DT_Local!0x01b0 m_meleeDisabledCounter
DT_Local!0x01b4 m_meleeInputIndex
DT_Local!0x01b8 m_trackedChildProjectileCount
DT_Local!0x01bc m_oneHandedWeaponUsage
DT_Local!0x01c0 m_flCockpitEntryTime
DT_Local!0x01c4 m_ejectStartTime
DT_Local!0x01c8 m_disembarkStartTime
DT_Local!0x01cc m_hotDropImpactTime
DT_Local!0x01d0 m_outOfBoundsDeadTime
DT_Local!0x01d4 m_objectiveIndex
DT_Local!0x01d8 m_objectiveEntity
DT_Local!0x01dc m_objectiveEndTime
DT_Local!0x01e0 m_cinematicEventFlags
DT_Local!0x01e4 m_forcedDialogueOnly
DT_Local!0x01e8 m_titanBuildTime
DT_Local!0x01ec m_titanBubbleShieldTime
DT_Local!0x01f0 m_titanEmbarkEnabled
DT_Local!0x01f1 m_titanDisembarkEnabled
DT_Local!0x01f4 m_voicePackIndex
DT_Local!0x01f8 m_playerAnimStationaryGoalFeetYaw
DT_Local!0x01fc m_playerAnimJumping
DT_Local!0x0200 m_playerAnimJumpStartTime
DT_Local!0x0204 m_playerAnimFirstJumpFrame
DT_Local!0x0205 m_playerAnimDodging
DT_Local!0x0208 m_playerAnimJumpActivity
DT_Local!0x020c m_playerAnimLanding
DT_Local!0x020d m_playerAnimShouldLand
DT_Local!0x0210 m_playerAnimLandStartTime
DT_Local!0x0214 m_playerAnimInAirWalk
DT_Local!0x0218 m_playerAnimPrevFrameSequenceMotionYaw
DT_Local!0x021c m_playerAnimMeleeParity
DT_Local!0x0220 m_playerAnimMeleeStartTime
DT_Local!0x0254 m_playerLocalGravityBlendStartRotation
DT_Local!0x0264 m_playerLocalGravityBlendEndRotation
DT_Local!0x0274 m_playerLocalGravityBlendEndDirection
DT_Local!0x0280 m_playerLocalGravityBlendStartTime
DT_Local!0x0284 m_playerLocalGravityBlendEndTime
DT_Local!0x0288 m_playerLocalGravityBlendStrength
DT_Local!0x028c m_playerLocalGravityStrength
DT_Local!0x0290 m_playerLocalGravityType
DT_Local!0x0294 m_playerLocalGravityPoint
DT_Local!0x02a0 m_playerLocalGravityLineStart
DT_Local!0x02ac m_playerLocalGravityLineEnd
DT_Local!0x02b8 m_playerLocalGravityEntity
DT_Local!0x02bc m_playerLocalGravityLineStartEntity
DT_Local!0x02c0 m_playerLocalGravityLineEndEntity
DT_Local!0x02c4 m_playerFloatLookStartTime
DT_Local!0x02c8 m_playerFloatLookEndTime
DT_Local!0x02cc m_wallrunLatestFloorHeight
DT_Local!0x02d0 m_wallrunFromJetpack
DT_Local!0x02d4 m_groundNormal
DT_Local!0x02e0 m_continuousUseBlocked
DT_Local!0x02e4 m_useEnt
```
</details>
<details>
<summary><code>class DT_LocalPlayerExclusive</code></summary>

```
{
	NearbyPushers: DT_NearbyPushers,
	m_localOrigin: VectorXY,
	m_localOrigin.z: Float,
	m_vecAbsVelocity: Vector,
	m_vecBaseVelocity: Vector,
	m_vecVelocity.x: Float,
	m_vecVelocity.y: Float,
	m_vecVelocity.z: Float,
	m_flFriction: Float,
	m_tethers: DataTable,
	m_lastUCmdSimulationTicks: Cycle,
	m_lastUCmdSimulationRemainderTime: Float,
	m_Local: DT_Local,
	m_currentFrameLocalPlayer: DT_CurrentData_LocalPlayer,
	m_modInventory: DataTable,
	m_consumableInventory: DataTable,
	m_fStickySprintMinTime: Float,
	m_sprintStartedTime: Time,
	m_sprintStartedFrac: Float,
	m_sprintEndedTime: Time,
	m_sprintEndedFrac: Float,
	m_stickySprintStartTime: Time,
	m_upDirPredicted: Vector,
	m_lastWallRunStartPos: Vector,
	m_wallrunFrictionScale: Float,
	m_groundFrictionScale: Float,
	m_traversalBegin: Vector,
	m_traversalMid: Vector,
	m_traversalEnd: Vector,
	m_traversalMidFrac: Float,
	m_traversalForwardDir: Vector,
	m_traversalProgress: Float,
	m_traversalStartTime: Time,
	m_traversalHandAppearTime: Time,
	m_traversalReleaseTime: Time,
	m_traversalBlendOutStartTime: Time,
	m_traversalBlendOutStartOffset: Vector,
	m_wallDangleJumpOffTime: Time,
	m_wallDangleMayHangHere: Int,
	m_wallDangleForceFallOff: Int,
	m_wallDangleLastPushedForward: Int,
	m_wallDangleDisableWeapon: Int,
	m_slowMoEnabled: Int,
	m_sliding: Int,
	m_slideLongJumpAllowed: Int,
	m_bIsStickySprinting: Int,
	m_prevMoveYaw: Float,
	m_sprintTiltVel: Float,
	m_sprintTiltPoseParameter: Int,
	m_sprintFracPoseParameter: Int,
	m_ziplineAllowed: Int,
	m_lastZipline: Int,
	m_lastZiplineDetachTime: Time,
	m_zipline: DT_PlayerZipline,
	m_ziplineViewOffsetPosition: Vector,
	m_ziplineViewOffsetVelocity: Vector,
	m_ziplineGrenadeEntity: Int,
	m_sameZiplineCooldownTime: Float,
	m_highSpeedViewmodelAnims: Int,
	m_playAnimationType: Int,
	m_detachGrappleOnPlayAnimationEnd: Int,
	m_playAnimationNext: DataTable,
	m_playAnimationEntityBlocker: Int,
	m_playAnimationEntityBlockerDucking: Int,
	m_boosting: Int,
	m_activateBoost: Int,
	m_repeatedBoost: Int,
	m_boostMeter: Float,
	m_jetpack: Int,
	m_activateJetpack: Int,
	m_jetpackAfterburner: Int,
	m_gliding: Int,
	m_glideMeter: Float,
	m_glideRechargeDelayAccumulator: Float,
	m_hovering: Int,
	m_lastJumpHeight: Float,
	m_touchingUpdraftTriggers: DataTable,
	m_touchingUpdraftTriggersCount: Int,
	m_touchingSlipTriggers: DataTable,
	m_touchingSlipTriggersCount: Int,
	m_slipAirRestrictDirection: Vector,
	m_slipAirRestrictTime: Time,
	m_replayImportantSounds_networkTableSoundID: DataTable,
	m_replayImportantSounds_beginTime: DataTable,
	m_viewConeActive: Int,
	m_viewConeParented: Int,
	m_viewConeParity: Int,
	m_hConstraintEntity: Int,
	m_vecConstraintCenter: Vector,
	m_flConstraintRadius: Float,
	m_flConstraintWidth: Float,
	m_flConstraintSpeedFactor: Float,
	m_bConstraintPastRadius: Int,
	m_observerModeStaticPosition: Vector,
	m_observerModeStaticAngles: Vector,
	m_observerModeStaticFOVOverride: Float,
	m_lastKillTime: Time,
	m_wallRunStartTime: Time,
	m_wallRunClearTime: Time,
	m_dodging: Int,
	m_dodgingInAir: Int,
	m_airSpeed: Float,
	m_airAcceleration: Float,
	m_firstPersonProxy: Int,
	m_predictedFirstPersonProxy: Int,
	m_hardpointEntity: Int,
	m_petTitanMode: Int,
	m_hThirdPersonEnt: Int,
	m_thirdPersonShoulderView: Int,
	m_thirdPerson: DT_ThirdPersonView,
	m_playerLookTargetEntity: Int,
	m_playerLookTargetOffset: Vector,
	m_viewConeLerpTime: Float,
	m_flLaggedMovementValue: Float,
	m_lastMoveInputTime: Time,
	m_ignoreEntityForMovementUntilNotTouching: Int,
	m_lungeTargetEntity: Int,
	m_isLungingToPosition: Int,
	m_lungeTargetPosition: Vector,
	m_lungeStartPositionOffset: Vector,
	m_lungeEndPositionOffset: Vector,
	m_lungeStartTime: Time,
	m_lungeEndTime: Time,
	m_lungeCanFly: Int,
	m_lungeLockPitch: Int,
	m_lungeStartPitch: Float,
	m_lungeSmoothTime: Float,
	m_lungeMaxTime: Float,
	m_lungeMaxEndSpeed: Float,
	m_nearbyPusherCount: Int,
	m_pushAwayFromTopAcceleration: Vector,
	m_minimapTargetZoomScale: Float,
	m_minimapTargetLerpTime: Time,
	m_playerScriptNetDataExclusive: Int,
	m_pingOffset: Float,
	m_skydiveForwardPoseValueVelocity: Float,
	m_skydiveForwardPoseValueCurrent: Float,
	m_skydiveSidePoseValueVelocity: Float,
	m_skydiveSidePoseValueCurrent: Float,
	m_skydiveYawVelocity: Float,
	m_skydiveIsNearLeviathan: Int,
	m_skydiveStartTime: Time,
	m_skydiveEndTime: Time,
	m_skydiveAnticipateStartTime: Time,
	m_skydiveAnticipateEndTime: Time,
	m_skydiveDistanceToLand: Float,
	m_skydiveFreelookEnabled: Int,
	m_skydiveFreelookLockedAngle: Vector,
	m_skydiveFollowing: Int,
	m_skydiveUnfollowVelocity: Vector,
	m_skydiveLeviathanHitPosition: Vector,
	m_skydiveLeviathanHitNormal: Vector,
	m_skydiveSlipVelocity: Vector,
	m_skydiveFromUpdraft: Int,
	m_twitchRewardBits: BitMask,
	m_playerKnockBacks: DataTable,
	m_updraftCount: Int,
	m_updraftStage: Int,
	m_updraftEnterTime: Time,
	m_updraftLeaveTime: Time,
	m_updraftMinShakeActivationHeight: Float,
	m_updraftMaxShakeActivationHeight: Float,
	m_updraftLiftActivationHeight: Float,
	m_updraftLiftSpeed: Float,
	m_updraftLiftAcceleration: Float,
	m_updraftLiftExitDuration: Float,
	m_updraftSlowTime: Time,
	m_armsModelIndex: Int,
	m_deathFieldIndex: Int,
}
```

### Offsets

```
DT_LocalPlayerExclusive!0x0000 NearbyPushers
DT_LocalPlayerExclusive!0x0004 m_localOrigin
DT_LocalPlayerExclusive!0x000c m_localOrigin.z
DT_LocalPlayerExclusive!0x0140 m_vecAbsVelocity
DT_LocalPlayerExclusive!0x0410 m_vecBaseVelocity
DT_LocalPlayerExclusive!0x0460 m_vecVelocity.x
DT_LocalPlayerExclusive!0x0464 m_vecVelocity.y
DT_LocalPlayerExclusive!0x0468 m_vecVelocity.z
DT_LocalPlayerExclusive!0x0478 m_flFriction
DT_LocalPlayerExclusive!0x1b3c m_tethers
DT_LocalPlayerExclusive!0x1c14 m_lastUCmdSimulationTicks
DT_LocalPlayerExclusive!0x1c18 m_lastUCmdSimulationRemainderTime
DT_LocalPlayerExclusive!0x1d38 m_Local
DT_LocalPlayerExclusive!0x21f0 m_currentFrameLocalPlayer
DT_LocalPlayerExclusive!0x258c m_modInventory
DT_LocalPlayerExclusive!0x260c m_consumableInventory
DT_LocalPlayerExclusive!0x2920 m_fStickySprintMinTime
DT_LocalPlayerExclusive!0x293c m_sprintStartedTime
DT_LocalPlayerExclusive!0x2940 m_sprintStartedFrac
DT_LocalPlayerExclusive!0x2944 m_sprintEndedTime
DT_LocalPlayerExclusive!0x2948 m_sprintEndedFrac
DT_LocalPlayerExclusive!0x294c m_stickySprintStartTime
DT_LocalPlayerExclusive!0x29bc m_upDirPredicted
DT_LocalPlayerExclusive!0x29c8 m_lastWallRunStartPos
DT_LocalPlayerExclusive!0x29ec m_wallrunFrictionScale
DT_LocalPlayerExclusive!0x29f0 m_groundFrictionScale
DT_LocalPlayerExclusive!0x2a34 m_traversalBegin
DT_LocalPlayerExclusive!0x2a40 m_traversalMid
DT_LocalPlayerExclusive!0x2a4c m_traversalEnd
DT_LocalPlayerExclusive!0x2a58 m_traversalMidFrac
DT_LocalPlayerExclusive!0x2a5c m_traversalForwardDir
DT_LocalPlayerExclusive!0x2a74 m_traversalProgress
DT_LocalPlayerExclusive!0x2a78 m_traversalStartTime
DT_LocalPlayerExclusive!0x2a7c m_traversalHandAppearTime
DT_LocalPlayerExclusive!0x2a80 m_traversalReleaseTime
DT_LocalPlayerExclusive!0x2a84 m_traversalBlendOutStartTime
DT_LocalPlayerExclusive!0x2a88 m_traversalBlendOutStartOffset
DT_LocalPlayerExclusive!0x2aa0 m_wallDangleJumpOffTime
DT_LocalPlayerExclusive!0x2aa4 m_wallDangleMayHangHere
DT_LocalPlayerExclusive!0x2aa5 m_wallDangleForceFallOff
DT_LocalPlayerExclusive!0x2aa6 m_wallDangleLastPushedForward
DT_LocalPlayerExclusive!0x2aa8 m_wallDangleDisableWeapon
DT_LocalPlayerExclusive!0x2b5c m_slowMoEnabled
DT_LocalPlayerExclusive!0x2b5d m_sliding
DT_LocalPlayerExclusive!0x2b5e m_slideLongJumpAllowed
DT_LocalPlayerExclusive!0x2b6c m_bIsStickySprinting
DT_LocalPlayerExclusive!0x2b70 m_prevMoveYaw
DT_LocalPlayerExclusive!0x2b74 m_sprintTiltVel
DT_LocalPlayerExclusive!0x2b78 m_sprintTiltPoseParameter
DT_LocalPlayerExclusive!0x2b7c m_sprintFracPoseParameter
DT_LocalPlayerExclusive!0x2ce4 m_ziplineAllowed
DT_LocalPlayerExclusive!0x2cec m_lastZipline
DT_LocalPlayerExclusive!0x2cf0 m_lastZiplineDetachTime
DT_LocalPlayerExclusive!0x2d00 m_zipline
DT_LocalPlayerExclusive!0x2d70 m_ziplineViewOffsetPosition
DT_LocalPlayerExclusive!0x2d7c m_ziplineViewOffsetVelocity
DT_LocalPlayerExclusive!0x2d88 m_ziplineGrenadeEntity
DT_LocalPlayerExclusive!0x2d98 m_sameZiplineCooldownTime
DT_LocalPlayerExclusive!0x2d9c m_highSpeedViewmodelAnims
DT_LocalPlayerExclusive!0x2da0 m_playAnimationType
DT_LocalPlayerExclusive!0x2da4 m_detachGrappleOnPlayAnimationEnd
DT_LocalPlayerExclusive!0x2da8 m_playAnimationNext
DT_LocalPlayerExclusive!0x2db0 m_playAnimationEntityBlocker
DT_LocalPlayerExclusive!0x2db4 m_playAnimationEntityBlockerDucking
DT_LocalPlayerExclusive!0x2dbc m_boosting
DT_LocalPlayerExclusive!0x2dbd m_activateBoost
DT_LocalPlayerExclusive!0x2dbe m_repeatedBoost
DT_LocalPlayerExclusive!0x2dc0 m_boostMeter
DT_LocalPlayerExclusive!0x2dc4 m_jetpack
DT_LocalPlayerExclusive!0x2dc5 m_activateJetpack
DT_LocalPlayerExclusive!0x2dc6 m_jetpackAfterburner
DT_LocalPlayerExclusive!0x2dc7 m_gliding
DT_LocalPlayerExclusive!0x2dc8 m_glideMeter
DT_LocalPlayerExclusive!0x2dcc m_glideRechargeDelayAccumulator
DT_LocalPlayerExclusive!0x2dd0 m_hovering
DT_LocalPlayerExclusive!0x2dd4 m_lastJumpHeight
DT_LocalPlayerExclusive!0x2dd8 m_touchingUpdraftTriggers
DT_LocalPlayerExclusive!0x2e18 m_touchingUpdraftTriggersCount
DT_LocalPlayerExclusive!0x2e1c m_touchingSlipTriggers
DT_LocalPlayerExclusive!0x2e5c m_touchingSlipTriggersCount
DT_LocalPlayerExclusive!0x2e60 m_slipAirRestrictDirection
DT_LocalPlayerExclusive!0x2e6c m_slipAirRestrictTime
DT_LocalPlayerExclusive!0x2fa8 m_replayImportantSounds_networkTableSoundID
DT_LocalPlayerExclusive!0x2fb8 m_replayImportantSounds_beginTime
DT_LocalPlayerExclusive!0x2ff5 m_viewConeActive
DT_LocalPlayerExclusive!0x2ff6 m_viewConeParented
DT_LocalPlayerExclusive!0x2ff8 m_viewConeParity
DT_LocalPlayerExclusive!0x3294 m_hConstraintEntity
DT_LocalPlayerExclusive!0x3298 m_vecConstraintCenter
DT_LocalPlayerExclusive!0x32a4 m_flConstraintRadius
DT_LocalPlayerExclusive!0x32a8 m_flConstraintWidth
DT_LocalPlayerExclusive!0x32ac m_flConstraintSpeedFactor
DT_LocalPlayerExclusive!0x32b0 m_bConstraintPastRadius
DT_LocalPlayerExclusive!0x330c m_observerModeStaticPosition
DT_LocalPlayerExclusive!0x3318 m_observerModeStaticAngles
DT_LocalPlayerExclusive!0x3324 m_observerModeStaticFOVOverride
DT_LocalPlayerExclusive!0x3390 m_lastKillTime
DT_LocalPlayerExclusive!0x33b4 m_wallRunStartTime
DT_LocalPlayerExclusive!0x33b8 m_wallRunClearTime
DT_LocalPlayerExclusive!0x33cc m_dodging
DT_LocalPlayerExclusive!0x3426 m_dodgingInAir
DT_LocalPlayerExclusive!0x3440 m_airSpeed
DT_LocalPlayerExclusive!0x3444 m_airAcceleration
DT_LocalPlayerExclusive!0x3470 m_firstPersonProxy
DT_LocalPlayerExclusive!0x3474 m_predictedFirstPersonProxy
DT_LocalPlayerExclusive!0x3484 m_hardpointEntity
DT_LocalPlayerExclusive!0x34bc m_petTitanMode
DT_LocalPlayerExclusive!0x34c4 m_hThirdPersonEnt
DT_LocalPlayerExclusive!0x34c8 m_thirdPersonShoulderView
DT_LocalPlayerExclusive!0x352c m_thirdPerson
DT_LocalPlayerExclusive!0x35f4 m_playerLookTargetEntity
DT_LocalPlayerExclusive!0x35f8 m_playerLookTargetOffset
DT_LocalPlayerExclusive!0x3638 m_viewConeLerpTime
DT_LocalPlayerExclusive!0x388c m_flLaggedMovementValue
DT_LocalPlayerExclusive!0x3890 m_lastMoveInputTime
DT_LocalPlayerExclusive!0x3894 m_ignoreEntityForMovementUntilNotTouching
DT_LocalPlayerExclusive!0x3ddc m_lungeTargetEntity
DT_LocalPlayerExclusive!0x3de0 m_isLungingToPosition
DT_LocalPlayerExclusive!0x3de4 m_lungeTargetPosition
DT_LocalPlayerExclusive!0x3df0 m_lungeStartPositionOffset
DT_LocalPlayerExclusive!0x3dfc m_lungeEndPositionOffset
DT_LocalPlayerExclusive!0x3e08 m_lungeStartTime
DT_LocalPlayerExclusive!0x3e0c m_lungeEndTime
DT_LocalPlayerExclusive!0x3e10 m_lungeCanFly
DT_LocalPlayerExclusive!0x3e11 m_lungeLockPitch
DT_LocalPlayerExclusive!0x3e14 m_lungeStartPitch
DT_LocalPlayerExclusive!0x3e18 m_lungeSmoothTime
DT_LocalPlayerExclusive!0x3e1c m_lungeMaxTime
DT_LocalPlayerExclusive!0x3e20 m_lungeMaxEndSpeed
DT_LocalPlayerExclusive!0x429c m_nearbyPusherCount
DT_LocalPlayerExclusive!0x42ac m_pushAwayFromTopAcceleration
DT_LocalPlayerExclusive!0x42bc m_minimapTargetZoomScale
DT_LocalPlayerExclusive!0x42c0 m_minimapTargetLerpTime
DT_LocalPlayerExclusive!0x42c8 m_playerScriptNetDataExclusive
DT_LocalPlayerExclusive!0x42d4 m_pingOffset
DT_LocalPlayerExclusive!0x42f0 m_skydiveForwardPoseValueVelocity
DT_LocalPlayerExclusive!0x42f8 m_skydiveForwardPoseValueCurrent
DT_LocalPlayerExclusive!0x42fc m_skydiveSidePoseValueVelocity
DT_LocalPlayerExclusive!0x4304 m_skydiveSidePoseValueCurrent
DT_LocalPlayerExclusive!0x4308 m_skydiveYawVelocity
DT_LocalPlayerExclusive!0x430c m_skydiveIsNearLeviathan
DT_LocalPlayerExclusive!0x432c m_skydiveStartTime
DT_LocalPlayerExclusive!0x4330 m_skydiveEndTime
DT_LocalPlayerExclusive!0x4334 m_skydiveAnticipateStartTime
DT_LocalPlayerExclusive!0x4338 m_skydiveAnticipateEndTime
DT_LocalPlayerExclusive!0x433c m_skydiveDistanceToLand
DT_LocalPlayerExclusive!0x4350 m_skydiveFreelookEnabled
DT_LocalPlayerExclusive!0x4354 m_skydiveFreelookLockedAngle
DT_LocalPlayerExclusive!0x4368 m_skydiveFollowing
DT_LocalPlayerExclusive!0x436c m_skydiveUnfollowVelocity
DT_LocalPlayerExclusive!0x437c m_skydiveLeviathanHitPosition
DT_LocalPlayerExclusive!0x4388 m_skydiveLeviathanHitNormal
DT_LocalPlayerExclusive!0x4394 m_skydiveSlipVelocity
DT_LocalPlayerExclusive!0x43a0 m_skydiveFromUpdraft
DT_LocalPlayerExclusive!0x43a8 m_twitchRewardBits
DT_LocalPlayerExclusive!0x43b8 m_playerKnockBacks
DT_LocalPlayerExclusive!0x4438 m_updraftCount
DT_LocalPlayerExclusive!0x443c m_updraftStage
DT_LocalPlayerExclusive!0x4440 m_updraftEnterTime
DT_LocalPlayerExclusive!0x4444 m_updraftLeaveTime
DT_LocalPlayerExclusive!0x4448 m_updraftMinShakeActivationHeight
DT_LocalPlayerExclusive!0x444c m_updraftMaxShakeActivationHeight
DT_LocalPlayerExclusive!0x4450 m_updraftLiftActivationHeight
DT_LocalPlayerExclusive!0x4454 m_updraftLiftSpeed
DT_LocalPlayerExclusive!0x4458 m_updraftLiftAcceleration
DT_LocalPlayerExclusive!0x445c m_updraftLiftExitDuration
DT_LocalPlayerExclusive!0x4460 m_updraftSlowTime
DT_LocalPlayerExclusive!0x4464 m_armsModelIndex
DT_LocalPlayerExclusive!0x4468 m_deathFieldIndex
```
</details>
<details>
<summary><code>class DT_LootGrabber extends DT_DynamicProp</code></summary>

```
{
	m_minimapData: DT_MinimapBaseEntityData,
	m_impactEffectColorID: Int,
	m_lootBeingGrabbed: Int,
	m_lootGrabDist: Float,
}
```

### Offsets

```
DT_LootGrabber!0x0948 m_minimapData
DT_LootGrabber!0x1640 m_impactEffectColorID
DT_LootGrabber!0x1648 m_lootBeingGrabbed
DT_LootGrabber!0x164c m_lootGrabDist
```
</details>
<details>
<summary><code>class DT_MinimapBaseEntityData</code></summary>

```
{
	visibilityDefaultFlag: DataTable,
	visibilityShowFlag: DataTable,
	flags: Int,
	zOrder: Int,
	customState: Int,
	objectScale: Float,
}
```

### Offsets

```
DT_MinimapBaseEntityData!0x0000 visibilityDefaultFlag
DT_MinimapBaseEntityData!0x0020 visibilityShowFlag
DT_MinimapBaseEntityData!0x0040 flags
DT_MinimapBaseEntityData!0x0044 zOrder
DT_MinimapBaseEntityData!0x0048 customState
DT_MinimapBaseEntityData!0x004c objectScale
```
</details>
<details>
<summary><code>class DT_MovieDisplay extends DT_BaseEntity</code></summary>

```
{
	m_bEnabled: Int,
	m_bLooping: Int,
	m_szMovieFilename: String,
	m_szGroupName: String,
	m_szExternalAudioFilename: String,
	m_bStretchToFill: Int,
	m_bLetterbox: Int,
	m_bPausesWithClient: Int,
	m_bForcedSlave: Int,
	m_bUseCustomUVs: Int,
	m_flUMin: Float,
	m_flUMax: Float,
	m_flVMin: Float,
	m_flVMax: Float,
}
```

### Offsets

```
DT_MovieDisplay!0x0a40 m_bEnabled
DT_MovieDisplay!0x0a41 m_bLooping
DT_MovieDisplay!0x0a43 m_szMovieFilename
DT_MovieDisplay!0x0ac3 m_szGroupName
DT_MovieDisplay!0x0b43 m_szExternalAudioFilename
DT_MovieDisplay!0x0b83 m_bStretchToFill
DT_MovieDisplay!0x0b84 m_bLetterbox
DT_MovieDisplay!0x0b85 m_bPausesWithClient
DT_MovieDisplay!0x0b86 m_bForcedSlave
DT_MovieDisplay!0x0b87 m_bUseCustomUVs
DT_MovieDisplay!0x0b8c m_flUMin
DT_MovieDisplay!0x0b90 m_flUMax
DT_MovieDisplay!0x0b94 m_flVMin
DT_MovieDisplay!0x0b98 m_flVMax
```
</details>
<details>
<summary><code>class DT_NPC_SentryTurret extends DT_AI_BaseNPC</code></summary>

```
{
	m_turretState: Int,
	m_killCount: Int,
	m_titanKillCount: Int,
	m_eyeAttach: Int,
	m_controlPanel: Int,
}
```

### Offsets

```
DT_NPC_SentryTurret!0x1d40 m_turretState
DT_NPC_SentryTurret!0x1d44 m_killCount
DT_NPC_SentryTurret!0x1d48 m_titanKillCount
DT_NPC_SentryTurret!0x1d4c m_eyeAttach
DT_NPC_SentryTurret!0x1d50 m_controlPanel
```
</details>
<details>
<summary><code>class DT_NPC_Titan extends DT_AI_BaseNPC</code></summary>

```
{
	m_decalIndex: Int,
	m_inventory: DT_WeaponInventory,
	m_selectedOffhands: DataTable,
	m_titanSoul: Int,
	m_grappleHook: Int,
	m_grapple: DT_GrappleData,
	m_grappleActive: Int,
	m_canStand: Int,
}
```

### Offsets

```
DT_NPC_Titan!0x0e98 m_decalIndex
DT_NPC_Titan!0x19b0 m_inventory
DT_NPC_Titan!0x1a1e m_selectedOffhands
DT_NPC_Titan!0x1aa0 m_titanSoul
DT_NPC_Titan!0x1d40 m_grappleHook
DT_NPC_Titan!0x1d48 m_grapple
DT_NPC_Titan!0x1dd8 m_grappleActive
DT_NPC_Titan!0x1dd9 m_canStand
```
</details>
<details>
<summary><code>class DT_NearbyPushers</code></summary>

```
{
	m_nearbyPushers: DataTable,
}
```

### Offsets

```
DT_NearbyPushers!0x414c m_nearbyPushers
```
</details>
<details>
<summary><code>class DT_OverlayVars</code></summary>

```
{
	m_AnimOverlay: DataTable,
	m_AnimOverlayCount: Int,
}
```

### Offsets

```
DT_OverlayVars!0x1608 m_AnimOverlay
DT_OverlayVars!0x16e0 m_AnimOverlayCount
```
</details>
<details>
<summary><code>class DT_ParticleSystem</code></summary>

```
{
	m_localOrigin: Vector,
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_visibilityFlags: Int,
	m_iTeamNum: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_realmsBitMask: BitMask,
	m_iEffectIndex: Int,
	m_nStopType: Int,
	m_bActive: Int,
	m_bForceRenderAlways: Int,
	m_flStartTime: Time,
	m_bInSkybox: Int,
	m_killForReplay: Int,
	m_killIfOverLimit: Int,
	m_vServerControlPoints: DataTable,
	m_hControlPointEnts: DataTable,
	m_controlPointAttachTypes: DataTable,
	m_controlPoint1AttachmentIndex: Int,
	m_vServerControlPointColorIds: DataTable,
	m_parentAttachType: Int,
}
```

### Offsets

```
DT_ParticleSystem!0x0004 m_localOrigin
DT_ParticleSystem!0x001c moveparent
DT_ParticleSystem!0x0020 m_parentAttachmentType
DT_ParticleSystem!0x0024 m_parentAttachmentIndex
DT_ParticleSystem!0x0044 m_fEffects
DT_ParticleSystem!0x0428 m_visibilityFlags
DT_ParticleSystem!0x0430 m_iTeamNum
DT_ParticleSystem!0x046c m_localAngles
DT_ParticleSystem!0x0480 m_hOwnerEntity
DT_ParticleSystem!0x0834 m_parentAttachmentHitbox
DT_ParticleSystem!0x0838 m_parentAttachmentModel
DT_ParticleSystem!0x0a28 m_realmsBitMask
DT_ParticleSystem!0x0a40 m_iEffectIndex
DT_ParticleSystem!0x0a44 m_nStopType
DT_ParticleSystem!0x0a49 m_bActive
DT_ParticleSystem!0x0a4b m_bForceRenderAlways
DT_ParticleSystem!0x0a4c m_flStartTime
DT_ParticleSystem!0x0a55 m_bInSkybox
DT_ParticleSystem!0x0a56 m_killForReplay
DT_ParticleSystem!0x0a57 m_killIfOverLimit
DT_ParticleSystem!0x0a5c m_vServerControlPoints
DT_ParticleSystem!0x0a98 m_hControlPointEnts
DT_ParticleSystem!0x0aac m_controlPointAttachTypes
DT_ParticleSystem!0x0ac4 m_controlPoint1AttachmentIndex
DT_ParticleSystem!0x0ad4 m_vServerControlPointColorIds
DT_ParticleSystem!0x0adc m_parentAttachType
```
</details>
<details>
<summary><code>class DT_PhysicsProp extends DT_BreakableProp</code></summary>

```
{
	m_spawnflags: Int,
	m_bAwake: Int,
	m_ignoresCollisionWithCombatCharacters: Int,
	m_isRolling: Int,
	m_networkTableRollSoundId: Int,
	m_iPhysicsMode: Int,
	m_fMass: Float,
	m_collisionMins: Vector,
	m_collisionMaxs: Vector,
}
```

### Offsets

```
DT_PhysicsProp!0x0094 m_spawnflags
DT_PhysicsProp!0x1608 m_bAwake
DT_PhysicsProp!0x1609 m_ignoresCollisionWithCombatCharacters
DT_PhysicsProp!0x160a m_isRolling
DT_PhysicsProp!0x160c m_networkTableRollSoundId
DT_PhysicsProp!0x1638 m_iPhysicsMode
DT_PhysicsProp!0x163c m_fMass
DT_PhysicsProp!0x1640 m_collisionMins
DT_PhysicsProp!0x164c m_collisionMaxs
```
</details>
<details>
<summary><code>class DT_Player extends DT_BaseCombatCharacter</code></summary>

```
{
	localdata: DT_LocalPlayerExclusive,
	teamshareddata: DT_PlayerTeamShared,
	m_passives: Array,
	portalnonlocaldata: DT_PortalNonLocalPlayerExclusive,
	m_vecAbsOrigin: Vector,
	isLocalOriginLocal: Int,
	m_fFlags: Int,
	m_hGroundEntity: Int,
	m_iHealth: Int,
	m_flMaxspeed: Float,
	m_iMaxHealth: Int,
	m_lifeState: Int,
	m_decalIndex: Int,
	m_inventory: DT_WeaponInventory,
	m_selectedOffhands: DataTable,
	m_selectedOffhandsPendingHybridAction: DataTable,
	m_titanSoul: Int,
	m_bZooming: Int,
	m_zoomToggleOnStartTime: Time,
	m_zoomBaseFrac: Float,
	m_zoomBaseTime: Time,
	m_zoomFullStartTime: Time,
	m_currentFramePlayer: DT_CurrentData_Player,
	pl: DT_PlayerState,
	m_ammoPoolCapacity: Int,
	m_hasBadReputation: Int,
	m_happyHourActive: Int,
	m_communityName: String,
	m_communityClanTag: String,
	m_factionName: String,
	m_hardwareIcon: String,
	m_hardware: Int,
	m_platformUserId: BitMask,
	m_EadpUserId: BitMask,
	m_crossPlayChat: Int,
	m_crossPlayChatFriends: Int,
	m_crossPlay: Int,
	m_classModsActive: BitMask,
	m_passives[ 0 ]: BitMask,
	m_bleedoutState: Int,
	m_bleedoutStartTime: Float,
	m_statusEffectsTimedPlayerNV: DataTable,
	m_statusEffectsEndlessPlayerNV: DataTable,
	m_damageComboLatestUpdateTime: Time,
	m_damageComboStartHealth: Int,
	m_gestureSequences: DataTable,
	m_gestureStartTimes: DataTable,
	m_gestureBlendInDuration: DataTable,
	m_gestureBlendOutDuration: DataTable,
	m_gestureFadeOutStartTime: DataTable,
	m_gestureFadeOutDuration: DataTable,
	m_gestureAutoKillBitfield: Int,
	m_autoSprintForced: Int,
	m_fIsSprinting: Int,
	m_playerSettingForStickySprintForward: Int,
	m_lastSprintPressTime: Time,
	m_stickySprintForwardEnableTime: Time,
	m_stickySprintForwardDisableTime: Time,
	m_damageImpulseNoDecelEndTime: Time,
	m_playerVehicles: DataTable,
	m_playerVehicleCount: Int,
	m_playerVehicleDriven: Int,
	m_playerVehicleUseTime: Time,
	m_duckState: Int,
	m_leanState: Int,
	m_canStand: Int,
	m_StandHullMin: Vector,
	m_StandHullMax: Vector,
	m_DuckHullMin: Vector,
	m_DuckHullMax: Vector,
	m_entitySyncingWithMe: Int,
	m_upDir: Vector,
	m_traversalState: Int,
	m_traversalType: Int,
	m_traversalForwardDir: Vector,
	m_traversalRefPos: Vector,
	m_traversalYawDelta: Float,
	m_traversalYawPoseParameter: Int,
	m_wallClimbSetUp: Int,
	m_wallHanging: Int,
	m_grapple: DT_GrappleData,
	m_grappleActive: Int,
	m_turret: Int,
	m_hViewModels: DataTable,
	m_viewOffsetEntity: DT_Player_ViewOffsetEntityData,
	m_animViewEntity: DT_Player_AnimViewEntityData,
	m_activeZipline: Int,
	m_ziplineValid3pWeaponLayerAnim: Int,
	m_ziplineState: Int,
	m_ziplineGrenadeBeginStationEntity: Int,
	m_ziplineGrenadeBeginStationAttachmentIndex: Int,
	m_isPerformingBoostAction: Int,
	m_lastJumpPadTouched: Int,
	m_launchCount: Int,
	m_melee: DT_PlayerMelee_PlayerData,
	m_useCredit: Int,
	m_playerFlags: Int,
	m_hasMic: Int,
	m_inPartyChat: Int,
	m_playerMoveSpeedScale: Float,
	m_bShouldDrawPlayerWhileUsingViewEntity: Int,
	m_iSpawnParity: Int,
	m_iObserverMode: Int,
	m_hObserverTarget: Int,
	m_flDeathTime: Time,
	m_lastDodgeTime: Time,
	m_timeJetpackHeightActivateCheckPassed: Time,
	m_grappleHook: Int,
	m_petTitan: Int,
	m_xp: Int,
	m_skill_mu: Float,
	m_bHasMatchAdminRole: Int,
	m_ubEFNoInterpParity: Int,
	m_hColorCorrectionCtrl: Int,
	m_title: String,
	m_Shared: DT_PlayerShared,
	m_pilotClassIndex: Int,
	m_pilotClassActivityModifier: Int,
	m_playerScriptNetDataGlobal: Int,
	m_helmetType: Int,
	m_armorType: Int,
	m_controllerModeActive: Int,
	m_skydiveForwardPoseValueTarget: Float,
	m_skydiveSidePoseValueTarget: Float,
	m_skydiveState: Int,
	m_skydiveDiveAngle: Float,
	m_skydiveIsDiving: Int,
	m_skydiveSpeed: Float,
	m_skydiveStrafeAngle: Float,
	m_skydivePlayerPitch: Float,
	m_skydivePlayerYaw: Float,
}
```

### Offsets

```
DT_Player!0x0000 localdata
DT_Player!0x0000 teamshareddata
DT_Player!0x0000 m_passives
DT_Player!0x0000 portalnonlocaldata
DT_Player!0x0004 m_vecAbsOrigin
DT_Player!0x0010 isLocalOriginLocal
DT_Player!0x0098 m_fFlags
DT_Player!0x041c m_hGroundEntity
DT_Player!0x0420 m_iHealth
DT_Player!0x0424 m_flMaxspeed
DT_Player!0x0550 m_iMaxHealth
DT_Player!0x0770 m_lifeState
DT_Player!0x0e98 m_decalIndex
DT_Player!0x19b0 m_inventory
DT_Player!0x1a1e m_selectedOffhands
DT_Player!0x1a21 m_selectedOffhandsPendingHybridAction
DT_Player!0x1aa0 m_titanSoul
DT_Player!0x1b81 m_bZooming
DT_Player!0x1b84 m_zoomToggleOnStartTime
DT_Player!0x1b88 m_zoomBaseFrac
DT_Player!0x1b8c m_zoomBaseTime
DT_Player!0x1b90 m_zoomFullStartTime
DT_Player!0x2020 m_currentFramePlayer
DT_Player!0x2430 pl
DT_Player!0x24b4 m_ammoPoolCapacity
DT_Player!0x24b8 m_hasBadReputation
DT_Player!0x24b9 m_happyHourActive
DT_Player!0x24c1 m_communityName
DT_Player!0x2501 m_communityClanTag
DT_Player!0x2511 m_factionName
DT_Player!0x2521 m_hardwareIcon
DT_Player!0x2531 m_hardware
DT_Player!0x2538 m_platformUserId
DT_Player!0x2540 m_EadpUserId
DT_Player!0x2548 m_crossPlayChat
DT_Player!0x2549 m_crossPlayChatFriends
DT_Player!0x254a m_crossPlay
DT_Player!0x2550 m_classModsActive
DT_Player!0x2650 m_passives[ 0 ]
DT_Player!0x2670 m_bleedoutState
DT_Player!0x2674 m_bleedoutStartTime
DT_Player!0x2678 m_statusEffectsTimedPlayerNV
DT_Player!0x2768 m_statusEffectsEndlessPlayerNV
DT_Player!0x281c m_damageComboLatestUpdateTime
DT_Player!0x2820 m_damageComboStartHealth
DT_Player!0x2824 m_gestureSequences
DT_Player!0x2844 m_gestureStartTimes
DT_Player!0x2864 m_gestureBlendInDuration
DT_Player!0x2884 m_gestureBlendOutDuration
DT_Player!0x28a4 m_gestureFadeOutStartTime
DT_Player!0x28c4 m_gestureFadeOutDuration
DT_Player!0x28e4 m_gestureAutoKillBitfield
DT_Player!0x2928 m_autoSprintForced
DT_Player!0x292c m_fIsSprinting
DT_Player!0x292e m_playerSettingForStickySprintForward
DT_Player!0x2930 m_lastSprintPressTime
DT_Player!0x2934 m_stickySprintForwardEnableTime
DT_Player!0x2938 m_stickySprintForwardDisableTime
DT_Player!0x2950 m_damageImpulseNoDecelEndTime
DT_Player!0x295c m_playerVehicles
DT_Player!0x2964 m_playerVehicleCount
DT_Player!0x2968 m_playerVehicleDriven
DT_Player!0x296c m_playerVehicleUseTime
DT_Player!0x2970 m_duckState
DT_Player!0x2974 m_leanState
DT_Player!0x2979 m_canStand
DT_Player!0x297c m_StandHullMin
DT_Player!0x2988 m_StandHullMax
DT_Player!0x2994 m_DuckHullMin
DT_Player!0x29a0 m_DuckHullMax
DT_Player!0x29ac m_entitySyncingWithMe
DT_Player!0x29b0 m_upDir
DT_Player!0x2a2c m_traversalState
DT_Player!0x2a30 m_traversalType
DT_Player!0x2a5c m_traversalForwardDir
DT_Player!0x2a68 m_traversalRefPos
DT_Player!0x2a94 m_traversalYawDelta
DT_Player!0x2a98 m_traversalYawPoseParameter
DT_Player!0x2ab0 m_wallClimbSetUp
DT_Player!0x2ab1 m_wallHanging
DT_Player!0x2ab8 m_grapple
DT_Player!0x2b48 m_grappleActive
DT_Player!0x2b90 m_turret
DT_Player!0x2b94 m_hViewModels
DT_Player!0x2ba8 m_viewOffsetEntity
DT_Player!0x2be8 m_animViewEntity
DT_Player!0x2ce8 m_activeZipline
DT_Player!0x2cf4 m_ziplineValid3pWeaponLayerAnim
DT_Player!0x2cf8 m_ziplineState
DT_Player!0x2d8c m_ziplineGrenadeBeginStationEntity
DT_Player!0x2d90 m_ziplineGrenadeBeginStationAttachmentIndex
DT_Player!0x2dd1 m_isPerformingBoostAction
DT_Player!0x2ec8 m_lastJumpPadTouched
DT_Player!0x2ed0 m_launchCount
DT_Player!0x3000 m_melee
DT_Player!0x3038 m_useCredit
DT_Player!0x3048 m_playerFlags
DT_Player!0x3050 m_hasMic
DT_Player!0x3051 m_inPartyChat
DT_Player!0x3054 m_playerMoveSpeedScale
DT_Player!0x3290 m_bShouldDrawPlayerWhileUsingViewEntity
DT_Player!0x32fc m_iSpawnParity
DT_Player!0x3304 m_iObserverMode
DT_Player!0x3308 m_hObserverTarget
DT_Player!0x3394 m_flDeathTime
DT_Player!0x33d0 m_lastDodgeTime
DT_Player!0x33f0 m_timeJetpackHeightActivateCheckPassed
DT_Player!0x3478 m_grappleHook
DT_Player!0x347c m_petTitan
DT_Player!0x34a4 m_xp
DT_Player!0x34ac m_skill_mu
DT_Player!0x34b0 m_bHasMatchAdminRole
DT_Player!0x3d90 m_ubEFNoInterpParity
DT_Player!0x3d94 m_hColorCorrectionCtrl
DT_Player!0x3db8 m_title
DT_Player!0x40f8 m_Shared
DT_Player!0x4140 m_pilotClassIndex
DT_Player!0x4144 m_pilotClassActivityModifier
DT_Player!0x42c4 m_playerScriptNetDataGlobal
DT_Player!0x42cc m_helmetType
DT_Player!0x42d0 m_armorType
DT_Player!0x42d8 m_controllerModeActive
DT_Player!0x42f4 m_skydiveForwardPoseValueTarget
DT_Player!0x4300 m_skydiveSidePoseValueTarget
DT_Player!0x4328 m_skydiveState
DT_Player!0x4340 m_skydiveDiveAngle
DT_Player!0x4344 m_skydiveIsDiving
DT_Player!0x4348 m_skydiveSpeed
DT_Player!0x434c m_skydiveStrafeAngle
DT_Player!0x4360 m_skydivePlayerPitch
DT_Player!0x4364 m_skydivePlayerYaw
```
</details>
<details>
<summary><code>class DT_PlayerDecoy extends DT_BaseAnimating</code></summary>

```
{
	m_cloakEndTime: Float,
	m_cloakFadeInEndTime: Time,
	m_cloakFadeOutStartTime: Float,
	m_cloakFadeInDuration: Float,
	m_cloakFlickerAmount: Float,
	m_cloakFlickerEndTime: Time,
	m_iHealth: Int,
	m_iMaxHealth: Int,
	m_nameVisibilityFlags: Int,
	m_currentState: Int,
	m_decoyFlags: Int,
	m_lastPulseTime: Time,
	m_currentClass: BitMask,
	m_classModsActive: BitMask,
}
```

### Offsets

```
DT_PlayerDecoy!0x019c m_cloakEndTime
DT_PlayerDecoy!0x01a0 m_cloakFadeInEndTime
DT_PlayerDecoy!0x01a4 m_cloakFadeOutStartTime
DT_PlayerDecoy!0x01a8 m_cloakFadeInDuration
DT_PlayerDecoy!0x01ac m_cloakFlickerAmount
DT_PlayerDecoy!0x01b0 m_cloakFlickerEndTime
DT_PlayerDecoy!0x0420 m_iHealth
DT_PlayerDecoy!0x0550 m_iMaxHealth
DT_PlayerDecoy!0x0998 m_nameVisibilityFlags
DT_PlayerDecoy!0x1600 m_currentState
DT_PlayerDecoy!0x1604 m_decoyFlags
DT_PlayerDecoy!0x160c m_lastPulseTime
DT_PlayerDecoy!0x1610 m_currentClass
DT_PlayerDecoy!0x1618 m_classModsActive
```
</details>
<details>
<summary><code>class DT_PlayerMelee_PlayerData</code></summary>

```
{
	meleeAttackParity: Int,
	attackActive: Int,
	attackRecoveryShouldBeQuick: Int,
	isSprintAttack: Int,
	attackStartTime: Time,
	attackHitEntity: Int,
	attackHitEntityTime: Time,
	attackLastHitNonWorldEntity: Time,
	scriptedState: Int,
	pendingMeleePress: Int,
	lungeBoost: Vector,
}
```

### Offsets

```
DT_PlayerMelee_PlayerData!0x0008 meleeAttackParity
DT_PlayerMelee_PlayerData!0x000c attackActive
DT_PlayerMelee_PlayerData!0x000d attackRecoveryShouldBeQuick
DT_PlayerMelee_PlayerData!0x000e isSprintAttack
DT_PlayerMelee_PlayerData!0x0010 attackStartTime
DT_PlayerMelee_PlayerData!0x0014 attackHitEntity
DT_PlayerMelee_PlayerData!0x0018 attackHitEntityTime
DT_PlayerMelee_PlayerData!0x001c attackLastHitNonWorldEntity
DT_PlayerMelee_PlayerData!0x0020 scriptedState
DT_PlayerMelee_PlayerData!0x0024 pendingMeleePress
DT_PlayerMelee_PlayerData!0x0028 lungeBoost
```
</details>
<details>
<summary><code>class DT_PlayerResource</code></summary>

```
{
	m_boolStats: DataTable,
	m_iPing: DataTable,
	m_bConnected: DataTable,
}
```

### Offsets

```
DT_PlayerResource!0x1450 m_boolStats
DT_PlayerResource!0x2c80 m_iPing
DT_PlayerResource!0x2e84 m_bConnected
```
</details>
<details>
<summary><code>class DT_PlayerTasklist extends DT_BaseEntity</code></summary>

```
{
	m_notifyTime: Time,
	m_customInt: Int,
	m_taskStatus: DataTable,
	m_taskType: DataTable,
	m_taskCountGoal: DataTable,
	m_taskCountNow: DataTable,
	m_taskFlags: DataTable,
	m_taskGameTimes: DataTable,
	m_taskInts: DataTable,
	m_taskFloats: DataTable,
	m_taskEnts: DataTable,
	m_taskStringA: String,
	m_taskStringB: String,
	m_taskStringC: String,
	m_taskStringD: String,
	m_taskStringE: String,
	m_taskStringF: String,
	m_taskStringG: String,
	m_taskStringH: String,
	m_taskStringI: String,
	m_taskStringJ: String,
	m_taskStringK: String,
	m_taskStringL: String,
	m_taskStringM: String,
}
```

### Offsets

```
DT_PlayerTasklist!0x0a40 m_notifyTime
DT_PlayerTasklist!0x0a44 m_customInt
DT_PlayerTasklist!0x0a48 m_taskStatus
DT_PlayerTasklist!0x0a7c m_taskType
DT_PlayerTasklist!0x0ab0 m_taskCountGoal
DT_PlayerTasklist!0x0ae4 m_taskCountNow
DT_PlayerTasklist!0x0b18 m_taskFlags
DT_PlayerTasklist!0x0b4c m_taskGameTimes
DT_PlayerTasklist!0x0b80 m_taskInts
DT_PlayerTasklist!0x0bb4 m_taskFloats
DT_PlayerTasklist!0x0be8 m_taskEnts
DT_PlayerTasklist!0x0c1c m_taskStringA
DT_PlayerTasklist!0x0c5c m_taskStringB
DT_PlayerTasklist!0x0c9c m_taskStringC
DT_PlayerTasklist!0x0cdc m_taskStringD
DT_PlayerTasklist!0x0d1c m_taskStringE
DT_PlayerTasklist!0x0d5c m_taskStringF
DT_PlayerTasklist!0x0d9c m_taskStringG
DT_PlayerTasklist!0x0ddc m_taskStringH
DT_PlayerTasklist!0x0e1c m_taskStringI
DT_PlayerTasklist!0x0e5c m_taskStringJ
DT_PlayerTasklist!0x0e9c m_taskStringK
DT_PlayerTasklist!0x0edc m_taskStringL
DT_PlayerTasklist!0x0f1c m_taskStringM
```
</details>
<details>
<summary><code>class DT_PlayerTeamShared</code></summary>

```
{
	m_healResources_healthTarget: Int,
	m_lastTimeDamagedByOtherPlayer: Time,
	m_lastTimeDamagedByNPC: Time,
	m_lastTimeDidDamageToOtherPlayer: Time,
	m_lastTimeDidDamageToNPC: Time,
}
```

### Offsets

```
DT_PlayerTeamShared!0x24bc m_healResources_healthTarget
DT_PlayerTeamShared!0x2e70 m_lastTimeDamagedByOtherPlayer
DT_PlayerTeamShared!0x2e74 m_lastTimeDamagedByNPC
DT_PlayerTeamShared!0x2e78 m_lastTimeDidDamageToOtherPlayer
DT_PlayerTeamShared!0x2e7c m_lastTimeDidDamageToNPC
```
</details>
<details>
<summary><code>class DT_PlayerVehicle extends DT_BaseAnimatingOverlay</code></summary>

```
{
	vehicledriverdata: DT_VehicleDriverExclusive,
	vehiclenondriverdata: DT_VehicleNonDriverExclusive,
	m_vehiclePlayers: Array,
	m_hoverVehicleHoverOffsetPrev: Array,
	m_vecViewOffset.x: Float,
	m_vecViewOffset.y: Float,
	m_vecViewOffset.z: Float,
	m_iHealth: Int,
	m_localAngles: Vector,
	m_iMaxHealth: Int,
	m_vehicleDriver: Int,
	m_driverActivationTime: Time,
	m_driverDeactivationTime: Time,
	m_vehiclePlayers[0]: Int,
	m_vehiclePlayerCount: Int,
	m_vehicleActivated: Int,
	m_blockDuckInput: Int,
	m_vehicleFlags: Int,
	m_vehicleType: Int,
	m_vehicleLaunchTime: Float,
	m_vehicleVelocity: Vector,
	m_vehicleGroundEntity: Int,
	m_vehicleGroundNormal: Vector,
	m_hoverVehicleHoverOffsetPrev[0]: Float,
	m_hoverVehicleSmoothTilt: Vector,
	m_hoverVehicleSmoothTiltVelocity: Vector,
	m_hoverVehicleSmoothYaw: Float,
	m_hoverVehicleSmoothYawVelocity: Float,
	m_hoverVehicleLookAheadAcceleration: Vector,
	m_hoverVehicleLookAheadTilt: Vector,
	m_hoverVehicleLastBoostTime: Time,
	m_hoverVehicleBoostVelocity: Vector,
	m_hoverVehicleDebugFlyMode: Int,
	m_hoverVehicleStunTimeEnd: Float,
	m_hoverVehicleThrottle: Float,
	m_hoverVehicleBanking: Float,
	m_hoverVehicleFrictionLastTime: Float,
	m_hoverVehicleFrictionSurfPropOther: Int,
	m_hoverVehicleFrictionNormal: Vector,
	m_hoverVehicleFrictionPos: Vector,
	m_overrideVehicleAngles: Vector,
	m_overrideVehicleAnglesUntilTick: Int,
	m_pushingEnt: Int,
}
```

### Offsets

```
DT_PlayerVehicle!0x0000 vehicledriverdata
DT_PlayerVehicle!0x0000 vehiclenondriverdata
DT_PlayerVehicle!0x0000 m_vehiclePlayers
DT_PlayerVehicle!0x0000 m_hoverVehicleHoverOffsetPrev
DT_PlayerVehicle!0x0038 m_vecViewOffset.x
DT_PlayerVehicle!0x003c m_vecViewOffset.y
DT_PlayerVehicle!0x0040 m_vecViewOffset.z
DT_PlayerVehicle!0x0420 m_iHealth
DT_PlayerVehicle!0x046c m_localAngles
DT_PlayerVehicle!0x0550 m_iMaxHealth
DT_PlayerVehicle!0x1984 m_vehicleDriver
DT_PlayerVehicle!0x198c m_driverActivationTime
DT_PlayerVehicle!0x1990 m_driverDeactivationTime
DT_PlayerVehicle!0x1994 m_vehiclePlayers[0]
DT_PlayerVehicle!0x19a4 m_vehiclePlayerCount
DT_PlayerVehicle!0x19a8 m_vehicleActivated
DT_PlayerVehicle!0x19a9 m_blockDuckInput
DT_PlayerVehicle!0x19ac m_vehicleFlags
DT_PlayerVehicle!0x19b0 m_vehicleType
DT_PlayerVehicle!0x19b8 m_vehicleLaunchTime
DT_PlayerVehicle!0x19c0 m_vehicleVelocity
DT_PlayerVehicle!0x19cc m_vehicleGroundEntity
DT_PlayerVehicle!0x19d0 m_vehicleGroundNormal
DT_PlayerVehicle!0x19e0 m_hoverVehicleHoverOffsetPrev[0]
DT_PlayerVehicle!0x1a50 m_hoverVehicleSmoothTilt
DT_PlayerVehicle!0x1a5c m_hoverVehicleSmoothTiltVelocity
DT_PlayerVehicle!0x1a68 m_hoverVehicleSmoothYaw
DT_PlayerVehicle!0x1a6c m_hoverVehicleSmoothYawVelocity
DT_PlayerVehicle!0x1a70 m_hoverVehicleLookAheadAcceleration
DT_PlayerVehicle!0x1a7c m_hoverVehicleLookAheadTilt
DT_PlayerVehicle!0x1a88 m_hoverVehicleLastBoostTime
DT_PlayerVehicle!0x1a90 m_hoverVehicleBoostVelocity
DT_PlayerVehicle!0x1a9c m_hoverVehicleDebugFlyMode
DT_PlayerVehicle!0x1aa0 m_hoverVehicleStunTimeEnd
DT_PlayerVehicle!0x1aa4 m_hoverVehicleThrottle
DT_PlayerVehicle!0x1aa8 m_hoverVehicleBanking
DT_PlayerVehicle!0x1aac m_hoverVehicleFrictionLastTime
DT_PlayerVehicle!0x1ab0 m_hoverVehicleFrictionSurfPropOther
DT_PlayerVehicle!0x1ab4 m_hoverVehicleFrictionNormal
DT_PlayerVehicle!0x1ac0 m_hoverVehicleFrictionPos
DT_PlayerVehicle!0x1b60 m_overrideVehicleAngles
DT_PlayerVehicle!0x1b6c m_overrideVehicleAnglesUntilTick
DT_PlayerVehicle!0x1b94 m_pushingEnt
```
</details>
<details>
<summary><code>class DT_PlayerWaypoint</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentIndex: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_iTeamNum: Int,
	m_teamMemberIndex: Int,
	m_hOwnerEntity: Int,
	m_iSignifierName: String,
	m_parentAttachmentModel: Int,
	m_realmsBitMask: BitMask,
	m_waypointType: Int,
	m_waypointBitfield: Int,
	m_waypointEnts: DataTable,
	m_waypointVectors: DataTable,
	m_waypointGameTimes: DataTable,
	m_waypointInts: DataTable,
	m_waypointFloats: DataTable,
	m_objectivePackedInt: Int,
	m_waypointGroupName: String,
	m_waypointGroupFlags: Int,
	m_waypointCustomType: String,
	m_waypointStringA: String,
	m_waypointStringB: String,
	m_waypointAssetA: String,
	m_waypointAssetB: String,
}
```

### Offsets

```
DT_PlayerWaypoint!0x001c moveparent
DT_PlayerWaypoint!0x0024 m_parentAttachmentIndex
DT_PlayerWaypoint!0x004c m_cellX
DT_PlayerWaypoint!0x0050 m_cellY
DT_PlayerWaypoint!0x0054 m_cellZ
DT_PlayerWaypoint!0x0058 m_localOrigin
DT_PlayerWaypoint!0x03d4 m_networkedFlags
DT_PlayerWaypoint!0x0428 m_visibilityFlags
DT_PlayerWaypoint!0x0430 m_iTeamNum
DT_PlayerWaypoint!0x0438 m_teamMemberIndex
DT_PlayerWaypoint!0x0480 m_hOwnerEntity
DT_PlayerWaypoint!0x0558 m_iSignifierName
DT_PlayerWaypoint!0x0838 m_parentAttachmentModel
DT_PlayerWaypoint!0x0a28 m_realmsBitMask
DT_PlayerWaypoint!0x0a40 m_waypointType
DT_PlayerWaypoint!0x0a44 m_waypointBitfield
DT_PlayerWaypoint!0x0a48 m_waypointEnts
DT_PlayerWaypoint!0x0a68 m_waypointVectors
DT_PlayerWaypoint!0x0ac8 m_waypointGameTimes
DT_PlayerWaypoint!0x0ae8 m_waypointInts
DT_PlayerWaypoint!0x0b08 m_waypointFloats
DT_PlayerWaypoint!0x0b28 m_objectivePackedInt
DT_PlayerWaypoint!0x0b2c m_waypointGroupName
DT_PlayerWaypoint!0x0b4c m_waypointGroupFlags
DT_PlayerWaypoint!0x0b50 m_waypointCustomType
DT_PlayerWaypoint!0x0b70 m_waypointStringA
DT_PlayerWaypoint!0x0bb0 m_waypointStringB
DT_PlayerWaypoint!0x0bf8 m_waypointAssetA
DT_PlayerWaypoint!0x0c78 m_waypointAssetB
```
</details>
<details>
<summary><code>class DT_PlayerZipline</code></summary>

```
{
	m_ziplineReenableWeapons: Int,
	m_mountingZiplineDuration: Float,
	m_mountingZiplineAlpha: Float,
	m_ziplineStartTime: Time,
	m_ziplineEndTime: Time,
	m_mountingZiplineSourcePosition: Vector,
	m_mountingZiplineSourceVelocity: Vector,
	m_mountingZiplineTargetPosition: Vector,
	m_ziplineUsePosition: Vector,
	m_slidingZiplineAlpha: Float,
	m_lastMoveDir2D: Vector,
	m_ziplineReverse: Int,
}
```

### Offsets

```
DT_PlayerZipline!0x0008 m_ziplineReenableWeapons
DT_PlayerZipline!0x000c m_mountingZiplineDuration
DT_PlayerZipline!0x0010 m_mountingZiplineAlpha
DT_PlayerZipline!0x0014 m_ziplineStartTime
DT_PlayerZipline!0x0018 m_ziplineEndTime
DT_PlayerZipline!0x001c m_mountingZiplineSourcePosition
DT_PlayerZipline!0x0028 m_mountingZiplineSourceVelocity
DT_PlayerZipline!0x0034 m_mountingZiplineTargetPosition
DT_PlayerZipline!0x004c m_ziplineUsePosition
DT_PlayerZipline!0x0058 m_slidingZiplineAlpha
DT_PlayerZipline!0x005c m_lastMoveDir2D
DT_PlayerZipline!0x0068 m_ziplineReverse
```
</details>
<details>
<summary><code>class DT_Player_AnimViewEntityData</code></summary>

```
{
	animViewEntityHandle: Int,
	animViewEntityAngleLerpInDuration: Float,
	animViewEntityOriginLerpInDuration: Float,
	animViewEntityLerpOutDuration: Float,
	animViewEntityStabilizePlayerEyeAngles: Int,
	animViewEntityThirdPersonCameraParity: Int,
	animViewEntityThirdPersonCameraAttachment: DataTable,
	animViewEntityNumThirdPersonCameraAttachments: Int,
	animViewEntityThirdPersonCameraVisibilityChecks: Int,
	animViewEntityDrawPlayer: Int,
	fovTarget: Float,
	fovSmoothTime: Float,
	animViewEntityParity: Int,
}
```

### Offsets

```
DT_Player_AnimViewEntityData!0x0000 animViewEntityHandle
DT_Player_AnimViewEntityData!0x0004 animViewEntityAngleLerpInDuration
DT_Player_AnimViewEntityData!0x0008 animViewEntityOriginLerpInDuration
DT_Player_AnimViewEntityData!0x000c animViewEntityLerpOutDuration
DT_Player_AnimViewEntityData!0x0010 animViewEntityStabilizePlayerEyeAngles
DT_Player_AnimViewEntityData!0x0014 animViewEntityThirdPersonCameraParity
DT_Player_AnimViewEntityData!0x0018 animViewEntityThirdPersonCameraAttachment
DT_Player_AnimViewEntityData!0x0030 animViewEntityNumThirdPersonCameraAttachments
DT_Player_AnimViewEntityData!0x0034 animViewEntityThirdPersonCameraVisibilityChecks
DT_Player_AnimViewEntityData!0x0035 animViewEntityDrawPlayer
DT_Player_AnimViewEntityData!0x0038 fovTarget
DT_Player_AnimViewEntityData!0x003c fovSmoothTime
DT_Player_AnimViewEntityData!0x0048 animViewEntityParity
```
</details>
<details>
<summary><code>class DT_PortalNonLocalPlayerExclusive</code></summary>

```
{
	m_pusher: Int,
	m_originRelativeToPusher: Vector,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: VectorXY,
	m_localOrigin.z: Float,
}
```

### Offsets

```
DT_PortalNonLocalPlayerExclusive!0x0028 m_pusher
DT_PortalNonLocalPlayerExclusive!0x002c m_originRelativeToPusher
DT_PortalNonLocalPlayerExclusive!0x004c m_cellX
DT_PortalNonLocalPlayerExclusive!0x0050 m_cellY
DT_PortalNonLocalPlayerExclusive!0x0054 m_cellZ
DT_PortalNonLocalPlayerExclusive!0x0058 m_localOrigin
DT_PortalNonLocalPlayerExclusive!0x0060 m_localOrigin.z
```
</details>
<details>
<summary><code>class DT_PortalPointPush extends DT_BaseEntity</code></summary>

```
{
	m_bEnabled: Int,
	m_flMagnitude: Float,
	m_flRadius: Float,
	m_flInnerRadius: Float,
	m_flConeOfInfluence: Float,
}
```

### Offsets

```
DT_PortalPointPush!0x0a40 m_bEnabled
DT_PortalPointPush!0x0a44 m_flMagnitude
DT_PortalPointPush!0x0a48 m_flRadius
DT_PortalPointPush!0x0a4c m_flInnerRadius
DT_PortalPointPush!0x0a50 m_flConeOfInfluence
```
</details>
<details>
<summary><code>class DT_PostProcessController extends DT_BaseEntity</code></summary>

```
{
	m_flPostProcessParameters: DataTable,
	m_bMaster: Int,
}
```

### Offsets

```
DT_PostProcessController!0x0a40 m_flPostProcessParameters
DT_PostProcessController!0x0a58 m_bMaster
```
</details>
<details>
<summary><code>class DT_PredictedAnimEventData</code></summary>

```
{
	m_predictedAnimEventTimes: DataTable,
	m_predictedAnimEventIndices: DataTable,
	m_predictedAnimEventCount: Int,
	m_predictedAnimEventTarget: Int,
	m_predictedAnimEventSequence: Int,
	m_predictedAnimEventModel: Int,
	m_predictedAnimEventsReadyToFireTime: Time,
}
```

### Offsets

```
DT_PredictedAnimEventData!0x0008 m_predictedAnimEventTimes
DT_PredictedAnimEventData!0x0028 m_predictedAnimEventIndices
DT_PredictedAnimEventData!0x0048 m_predictedAnimEventCount
DT_PredictedAnimEventData!0x004c m_predictedAnimEventTarget
DT_PredictedAnimEventData!0x0050 m_predictedAnimEventSequence
DT_PredictedAnimEventData!0x0054 m_predictedAnimEventModel
DT_PredictedAnimEventData!0x0058 m_predictedAnimEventsReadyToFireTime
```
</details>
<details>
<summary><code>class DT_Projectile</code></summary>

```
{
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_iTeamNum: Int,
	m_vecVelocity: Vector,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_CollisionGroup: Int,
	m_PredictableID: Int,
	m_realmsBitMask: BitMask,
	m_weaponDataIsSet: Int,
	m_forceAdjustToGunBarrelDisabled: Int,
	m_weaponClassIndex: Int,
	m_destructionDistance: Float,
	m_passThroughDepthTotal: Int,
	m_modBitfield: Int,
	m_overrideMods: Int,
	m_projectileTrailIndex: Int,
	m_impactEffectTable: Int,
	m_reducedEffects: Int,
	m_projectileCreationTimeServer: Time,
	m_weaponSource: Int,
}
```

### Offsets

```
DT_Projectile!0x004c m_cellX
DT_Projectile!0x0050 m_cellY
DT_Projectile!0x0054 m_cellZ
DT_Projectile!0x0058 m_localOrigin
DT_Projectile!0x0064 m_nModelIndex
DT_Projectile!0x03d4 m_networkedFlags
DT_Projectile!0x0430 m_iTeamNum
DT_Projectile!0x0460 m_vecVelocity
DT_Projectile!0x046c m_localAngles
DT_Projectile!0x0480 m_hOwnerEntity
DT_Projectile!0x0518 m_CollisionGroup
DT_Projectile!0x07a4 m_PredictableID
DT_Projectile!0x0a28 m_realmsBitMask
DT_Projectile!0x1600 m_weaponDataIsSet
DT_Projectile!0x1601 m_forceAdjustToGunBarrelDisabled
DT_Projectile!0x1604 m_weaponClassIndex
DT_Projectile!0x1608 m_destructionDistance
DT_Projectile!0x160c m_passThroughDepthTotal
DT_Projectile!0x1610 m_modBitfield
DT_Projectile!0x1614 m_overrideMods
DT_Projectile!0x1618 m_projectileTrailIndex
DT_Projectile!0x161c m_impactEffectTable
DT_Projectile!0x1620 m_reducedEffects
DT_Projectile!0x1624 m_projectileCreationTimeServer
DT_Projectile!0x1628 m_weaponSource
```
</details>
<details>
<summary><code>class DT_PropSurvival</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_usableType: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_localAngles: Vector,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_iSignifierName: String,
	m_parentAttachmentModel: Int,
	m_usablePriority: Int,
	m_usableDistanceOverride: Float,
	m_usableFOV: Float,
	m_usePromptSize: Float,
	m_realmsBitMask: BitMask,
	m_nSkin: Int,
	m_skinMod: Int,
	m_nBody: Int,
	m_camoIndex: Int,
	m_ammoInClip: Int,
	m_customScriptInt: Int,
	m_survivalProperty: Int,
	m_weaponNameIndex: Int,
	m_modBitField: Int,
	m_survivalPropFadeDist: Float,
}
```

### Offsets

```
DT_PropSurvival!0x001c moveparent
DT_PropSurvival!0x0020 m_parentAttachmentType
DT_PropSurvival!0x0024 m_parentAttachmentIndex
DT_PropSurvival!0x0044 m_fEffects
DT_PropSurvival!0x0048 m_usableType
DT_PropSurvival!0x004c m_cellX
DT_PropSurvival!0x0050 m_cellY
DT_PropSurvival!0x0054 m_cellZ
DT_PropSurvival!0x0058 m_localOrigin
DT_PropSurvival!0x0064 m_nModelIndex
DT_PropSurvival!0x03d4 m_networkedFlags
DT_PropSurvival!0x0428 m_visibilityFlags
DT_PropSurvival!0x046c m_localAngles
DT_PropSurvival!0x0498 m_Collision
DT_PropSurvival!0x0518 m_CollisionGroup
DT_PropSurvival!0x0558 m_iSignifierName
DT_PropSurvival!0x0838 m_parentAttachmentModel
DT_PropSurvival!0x0908 m_usablePriority
DT_PropSurvival!0x090c m_usableDistanceOverride
DT_PropSurvival!0x0910 m_usableFOV
DT_PropSurvival!0x0914 m_usePromptSize
DT_PropSurvival!0x0a28 m_realmsBitMask
DT_PropSurvival!0x0e88 m_nSkin
DT_PropSurvival!0x0e8c m_skinMod
DT_PropSurvival!0x0e90 m_nBody
DT_PropSurvival!0x0e94 m_camoIndex
DT_PropSurvival!0x1604 m_ammoInClip
DT_PropSurvival!0x1608 m_customScriptInt
DT_PropSurvival!0x160c m_survivalProperty
DT_PropSurvival!0x1610 m_weaponNameIndex
DT_PropSurvival!0x1614 m_modBitField
DT_PropSurvival!0x161c m_survivalPropFadeDist
```
</details>
<details>
<summary><code>class DT_RopeKeyframe</code></summary>

```
{
	m_localOrigin: Vector,
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_visibilityFlags: Int,
	m_hOwnerEntity: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_ropeZiplineAutoDetachDistance: Float,
	m_ziplineSagEnable: Int,
	m_ziplineSagHeight: Float,
	m_ziplineMoveSpeedScale: Float,
	m_wiggleFadeStartTime: Time,
	m_wiggleEndTime: Time,
	m_wiggleMaxLen: Float,
	m_wiggleMagnitude: Float,
	m_wiggleSpeed: Float,
	m_flScrollSpeed: Float,
	m_RopeFlags: Int,
	m_iRopeMaterialModelIndex: Int,
	m_nSegments: Int,
	m_hStartPoint: Int,
	m_hEndPoint: Int,
	m_hPrevPoint: Int,
	m_iStartAttachment: Int,
	m_iEndAttachment: Int,
	m_subdivStackCount: Int,
	m_subdivSliceCount: Int,
	m_ropeLength: Int,
	m_constraintIterations: Int,
	m_ropeDampening: Float,
	m_Slack: Int,
	m_TextureScale: Float,
	m_TextureScale: Float,
	m_fLockedPoints: Int,
	m_lockDirectionCutoffLength: Int,
	m_lockDirectionStrength: Float,
	m_nChangeCount: Int,
	m_Width: Float,
	m_bConstrainBetweenEndpoints: Int,
}
```

### Offsets

```
DT_RopeKeyframe!0x0004 m_localOrigin
DT_RopeKeyframe!0x001c moveparent
DT_RopeKeyframe!0x0020 m_parentAttachmentType
DT_RopeKeyframe!0x0024 m_parentAttachmentIndex
DT_RopeKeyframe!0x0428 m_visibilityFlags
DT_RopeKeyframe!0x0480 m_hOwnerEntity
DT_RopeKeyframe!0x0834 m_parentAttachmentHitbox
DT_RopeKeyframe!0x0838 m_parentAttachmentModel
DT_RopeKeyframe!0x0844 m_fadeDist
DT_RopeKeyframe!0x0a40 m_ropeZiplineAutoDetachDistance
DT_RopeKeyframe!0x0a44 m_ziplineSagEnable
DT_RopeKeyframe!0x0a48 m_ziplineSagHeight
DT_RopeKeyframe!0x0b40 m_ziplineMoveSpeedScale
DT_RopeKeyframe!0x0b44 m_wiggleFadeStartTime
DT_RopeKeyframe!0x0b48 m_wiggleEndTime
DT_RopeKeyframe!0x0b4c m_wiggleMaxLen
DT_RopeKeyframe!0x0b50 m_wiggleMagnitude
DT_RopeKeyframe!0x0b54 m_wiggleSpeed
DT_RopeKeyframe!0x0b8c m_flScrollSpeed
DT_RopeKeyframe!0x0b90 m_RopeFlags
DT_RopeKeyframe!0x0b94 m_iRopeMaterialModelIndex
DT_RopeKeyframe!0x0e18 m_nSegments
DT_RopeKeyframe!0x0e1c m_hStartPoint
DT_RopeKeyframe!0x0e20 m_hEndPoint
DT_RopeKeyframe!0x0e24 m_hPrevPoint
DT_RopeKeyframe!0x0e28 m_iStartAttachment
DT_RopeKeyframe!0x0e2a m_iEndAttachment
DT_RopeKeyframe!0x0e54 m_subdivStackCount
DT_RopeKeyframe!0x0e58 m_subdivSliceCount
DT_RopeKeyframe!0x0e5c m_ropeLength
DT_RopeKeyframe!0x0e64 m_constraintIterations
DT_RopeKeyframe!0x0e68 m_ropeDampening
DT_RopeKeyframe!0x0e6c m_Slack
DT_RopeKeyframe!0x0e70 m_TextureScale
DT_RopeKeyframe!0x0e70 m_TextureScale
DT_RopeKeyframe!0x0e74 m_fLockedPoints
DT_RopeKeyframe!0x0e78 m_lockDirectionCutoffLength
DT_RopeKeyframe!0x0e7c m_lockDirectionStrength
DT_RopeKeyframe!0x0e80 m_nChangeCount
DT_RopeKeyframe!0x0e84 m_Width
DT_RopeKeyframe!0x0f18 m_bConstrainBetweenEndpoints
```
</details>
<details>
<summary><code>class DT_ScriptMover extends DT_ScriptProp</code></summary>

```
{
	m_parentAttachmentType: Int,
	m_vecAngVelocity: Vector,
	m_vecVelocity: Vector,
	m_localAngles: Vector,
	m_parentAttachmentHitbox: Int,
}
```

### Offsets

```
DT_ScriptMover!0x0020 m_parentAttachmentType
DT_ScriptMover!0x0128 m_vecAngVelocity
DT_ScriptMover!0x0460 m_vecVelocity
DT_ScriptMover!0x046c m_localAngles
DT_ScriptMover!0x0834 m_parentAttachmentHitbox
```
</details>
<details>
<summary><code>class DT_ScriptMoverLightweight</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_moverNetworkCellX: Int,
	m_moverNetworkCellY: Int,
	m_moverNetworkCellZ: Int,
	m_moverNetworkLocalOrigin: Vector,
	m_nModelIndex: Int,
	m_moverNetworkAngularVelocity: Vector,
	m_networkedFlags: Int,
	m_moverNetworkLinearVelocity: Vector,
	m_moverNetworkLocalAngles: Vector,
	m_scriptNameIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_moveModeNonPhysics: Int,
	m_moveModeIsLocal: Int,
	m_moveToStartPos: Vector,
	m_moveToEndPos: Vector,
	m_moveToTimeStart: Time,
	m_moveToTimeEnd: Time,
	m_moveToTimeEaseIn: Float,
	m_moveToTimeEaseOut: Float,
	m_moveVelocity: Vector,
	m_moveGravity: Vector,
	m_trainStartTime: Time,
	m_trainStopTime: Time,
	m_trainStartDistance: Float,
	m_trainCurrentNode: Int,
	m_trainStopNode: Int,
	m_trainInitialSpeed: Float,
	m_trainGoalSpeed: Float,
	m_trainAcceleration: Float,
	m_trainLastNode: Int,
	m_trainLastDistance: Float,
	m_trainLastSpeed: Float,
	m_trainFollowMover: Int,
	m_trainFollowDistance: Float,
	m_trainBreadcrumb: DataTable,
	m_trainBreadcrumbBegin: Int,
	m_trainBreadcrumbCount: Int,
	m_trainAutoRollStrength: Float,
	m_trainAutoRollLookAheadDistance: Float,
	m_trainAutoRollMax: Float,
	m_trainSimulateBeforeMeEntity: Int,
	m_rotateModeNonPhysics: Int,
	m_rotateModeIsLocal: Int,
	m_RotateToAnglesStart: Vector,
	m_RotateToAnglesEnd: Vector,
	m_rotateToTimeStart: Time,
	m_rotateToTimeEnd: Time,
	m_rotateToTimeEaseIn: Float,
	m_rotateToTimeEaseOut: Float,
	m_rotateAxis: Vector,
	m_rotateSpeed: Float,
	m_trainPitchMax: Float,
	m_useNonPhysicsMoveInterpolation: Int,
}
```

### Offsets

```
DT_ScriptMoverLightweight!0x001c moveparent
DT_ScriptMoverLightweight!0x0020 m_parentAttachmentType
DT_ScriptMoverLightweight!0x0024 m_parentAttachmentIndex
DT_ScriptMoverLightweight!0x0044 m_fEffects
DT_ScriptMoverLightweight!0x004c m_moverNetworkCellX
DT_ScriptMoverLightweight!0x0050 m_moverNetworkCellY
DT_ScriptMoverLightweight!0x0054 m_moverNetworkCellZ
DT_ScriptMoverLightweight!0x0058 m_moverNetworkLocalOrigin
DT_ScriptMoverLightweight!0x0064 m_nModelIndex
DT_ScriptMoverLightweight!0x0128 m_moverNetworkAngularVelocity
DT_ScriptMoverLightweight!0x03d4 m_networkedFlags
DT_ScriptMoverLightweight!0x0460 m_moverNetworkLinearVelocity
DT_ScriptMoverLightweight!0x046c m_moverNetworkLocalAngles
DT_ScriptMoverLightweight!0x0668 m_scriptNameIndex
DT_ScriptMoverLightweight!0x0834 m_parentAttachmentHitbox
DT_ScriptMoverLightweight!0x0838 m_parentAttachmentModel
DT_ScriptMoverLightweight!0x0844 m_fadeDist
DT_ScriptMoverLightweight!0x1744 m_moveModeNonPhysics
DT_ScriptMoverLightweight!0x1748 m_moveModeIsLocal
DT_ScriptMoverLightweight!0x174c m_moveToStartPos
DT_ScriptMoverLightweight!0x1758 m_moveToEndPos
DT_ScriptMoverLightweight!0x1764 m_moveToTimeStart
DT_ScriptMoverLightweight!0x1768 m_moveToTimeEnd
DT_ScriptMoverLightweight!0x176c m_moveToTimeEaseIn
DT_ScriptMoverLightweight!0x1770 m_moveToTimeEaseOut
DT_ScriptMoverLightweight!0x1774 m_moveVelocity
DT_ScriptMoverLightweight!0x1780 m_moveGravity
DT_ScriptMoverLightweight!0x178c m_trainStartTime
DT_ScriptMoverLightweight!0x1790 m_trainStopTime
DT_ScriptMoverLightweight!0x1794 m_trainStartDistance
DT_ScriptMoverLightweight!0x1798 m_trainCurrentNode
DT_ScriptMoverLightweight!0x179c m_trainStopNode
DT_ScriptMoverLightweight!0x17a0 m_trainInitialSpeed
DT_ScriptMoverLightweight!0x17a4 m_trainGoalSpeed
DT_ScriptMoverLightweight!0x17a8 m_trainAcceleration
DT_ScriptMoverLightweight!0x17ac m_trainLastNode
DT_ScriptMoverLightweight!0x17b0 m_trainLastDistance
DT_ScriptMoverLightweight!0x17b4 m_trainLastSpeed
DT_ScriptMoverLightweight!0x17b8 m_trainFollowMover
DT_ScriptMoverLightweight!0x17bc m_trainFollowDistance
DT_ScriptMoverLightweight!0x17c0 m_trainBreadcrumb
DT_ScriptMoverLightweight!0x17e0 m_trainBreadcrumbBegin
DT_ScriptMoverLightweight!0x17e4 m_trainBreadcrumbCount
DT_ScriptMoverLightweight!0x17e8 m_trainAutoRollStrength
DT_ScriptMoverLightweight!0x17ec m_trainAutoRollLookAheadDistance
DT_ScriptMoverLightweight!0x17f0 m_trainAutoRollMax
DT_ScriptMoverLightweight!0x17f4 m_trainSimulateBeforeMeEntity
DT_ScriptMoverLightweight!0x17f8 m_rotateModeNonPhysics
DT_ScriptMoverLightweight!0x17fc m_rotateModeIsLocal
DT_ScriptMoverLightweight!0x1800 m_RotateToAnglesStart
DT_ScriptMoverLightweight!0x180c m_RotateToAnglesEnd
DT_ScriptMoverLightweight!0x1818 m_rotateToTimeStart
DT_ScriptMoverLightweight!0x181c m_rotateToTimeEnd
DT_ScriptMoverLightweight!0x1820 m_rotateToTimeEaseIn
DT_ScriptMoverLightweight!0x1824 m_rotateToTimeEaseOut
DT_ScriptMoverLightweight!0x1828 m_rotateAxis
DT_ScriptMoverLightweight!0x1834 m_rotateSpeed
DT_ScriptMoverLightweight!0x1838 m_trainPitchMax
DT_ScriptMoverLightweight!0x1884 m_useNonPhysicsMoveInterpolation
```
</details>
<details>
<summary><code>class DT_ScriptMoverTrainNode</code></summary>

```
{
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_scriptNameIndex: Int,
	m_firstChildEntityLink: Int,
	m_firstParentEntityLink: Int,
	m_numSmoothPoints: Int,
	m_trainNodeMakeSmoothPointsParity: Int,
	m_tangentType: Int,
	m_perfectCircularRotation: Int,
}
```

### Offsets

```
DT_ScriptMoverTrainNode!0x004c m_cellX
DT_ScriptMoverTrainNode!0x0050 m_cellY
DT_ScriptMoverTrainNode!0x0054 m_cellZ
DT_ScriptMoverTrainNode!0x0058 m_localOrigin
DT_ScriptMoverTrainNode!0x0668 m_scriptNameIndex
DT_ScriptMoverTrainNode!0x0a20 m_firstChildEntityLink
DT_ScriptMoverTrainNode!0x0a24 m_firstParentEntityLink
DT_ScriptMoverTrainNode!0x0a40 m_numSmoothPoints
DT_ScriptMoverTrainNode!0x0a44 m_trainNodeMakeSmoothPointsParity
DT_ScriptMoverTrainNode!0x0a48 m_tangentType
DT_ScriptMoverTrainNode!0x0a4c m_perfectCircularRotation
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_DEATH_BOX extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_bools
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_ranges
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_int32s
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_times
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_entities
DT_ScriptNetData_SNDC_DEATH_BOX!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0c84 m_ranges[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0ca8 m_int32s[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0cb4 m_times[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0cc0 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_GLOBAL extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_bools
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_ranges
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_int32s
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_times
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_entities
DT_ScriptNetData_SNDC_GLOBAL!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0c92 m_ranges[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0cd8 m_int32s[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0d00 m_times[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0d68 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0000 m_bools
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0000 m_ranges
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0000 m_int32s
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0000 m_times
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0000 m_entities
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0c92 m_ranges[0]
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0cd8 m_int32s[0]
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0d00 m_times[0]
DT_ScriptNetData_SNDC_GLOBAL_NON_REWIND!0x0d68 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_bools
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_ranges
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_int32s
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_times
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_entities
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0c9a m_ranges[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0d58 m_int32s[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0da0 m_times[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0de8 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_PLAYER_GLOBAL extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_bools
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_ranges
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_int32s
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_times
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_entities
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0c8e m_ranges[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0cd4 m_int32s[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0d0c m_times[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0d34 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_TITAN_SOUL extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_bools
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_ranges
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_int32s
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_times
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_entities
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0c8a m_ranges[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0cb0 m_int32s[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0cc0 m_times[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0ce8 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptProp extends DT_DynamicProp</code></summary>

```
{
	m_networkedFlags: Int,
	m_iHealth: Int,
	m_iMaxHealth: Int,
	m_minimapData: DT_MinimapBaseEntityData,
	m_nameVisibilityFlags: Int,
	m_title: String,
	m_footstepType: String,
	m_renderColorFriendlyIsValid: Int,
	m_renderColorFriendly: Int,
	m_armorType: Int,
	m_scriptPropFlags: Int,
	m_scriptPropSmartAmmoLockType: Int,
}
```

### Offsets

```
DT_ScriptProp!0x03d4 m_networkedFlags
DT_ScriptProp!0x0420 m_iHealth
DT_ScriptProp!0x0550 m_iMaxHealth
DT_ScriptProp!0x0948 m_minimapData
DT_ScriptProp!0x0998 m_nameVisibilityFlags
DT_ScriptProp!0x1660 m_title
DT_ScriptProp!0x1680 m_footstepType
DT_ScriptProp!0x16c0 m_renderColorFriendlyIsValid
DT_ScriptProp!0x16c1 m_renderColorFriendly
DT_ScriptProp!0x16c8 m_armorType
DT_ScriptProp!0x16cc m_scriptPropFlags
DT_ScriptProp!0x16d0 m_scriptPropSmartAmmoLockType
```
</details>
<details>
<summary><code>class DT_ScriptTraceVolume extends DT_BaseEntity</code></summary>

```
{
	m_shapeType: Int,
	m_sphereRadius: Float,
	m_boxMins: Vector,
	m_boxMaxs: Vector,
	m_drawDebug: Int,
}
```

### Offsets

```
DT_ScriptTraceVolume!0x0a40 m_shapeType
DT_ScriptTraceVolume!0x0a44 m_sphereRadius
DT_ScriptTraceVolume!0x0a48 m_boxMins
DT_ScriptTraceVolume!0x0a54 m_boxMaxs
DT_ScriptTraceVolume!0x0a60 m_drawDebug
```
</details>
<details>
<summary><code>class DT_SequenceTransitioner</code></summary>

```
{
	m_sequenceTransitionerLayers: DataTable,
	m_sequenceTransitionerLayerCount: Int,
}
```

### Offsets

```
DT_SequenceTransitioner!0x0050 m_sequenceTransitionerLayers
DT_SequenceTransitioner!0x01a0 m_sequenceTransitionerLayerCount
```
</details>
<details>
<summary><code>class DT_SequenceTransitionerLayer</code></summary>

```
{
	m_sequenceTransitionerLayerActive: Int,
	m_sequenceTransitionerLayerStartCycle: Cycle,
	m_sequenceTransitionerLayerSequence: Int,
	m_sequenceTransitionerLayerPlaybackRate: Float,
	m_sequenceTransitionerLayerStartTime: Time,
	m_sequenceTransitionerLayerFadeOutDuration: Cycle,
}
```

### Offsets

```
DT_SequenceTransitionerLayer!0x0018 m_sequenceTransitionerLayerActive
DT_SequenceTransitionerLayer!0x001c m_sequenceTransitionerLayerStartCycle
DT_SequenceTransitionerLayer!0x0020 m_sequenceTransitionerLayerSequence
DT_SequenceTransitionerLayer!0x0028 m_sequenceTransitionerLayerPlaybackRate
DT_SequenceTransitionerLayer!0x002c m_sequenceTransitionerLayerStartTime
DT_SequenceTransitionerLayer!0x0030 m_sequenceTransitionerLayerFadeOutDuration
```
</details>
<details>
<summary><code>class DT_SoundData</code></summary>

```
{
	m_targetEnt: Int,
	m_soundID: BitMask,
	m_networkTableID: Int,
	m_soundIsStart: Int,
	m_seek: Float,
}
```

### Offsets

```
DT_SoundData!0x0000 m_targetEnt
DT_SoundData!0x0008 m_soundID
DT_SoundData!0x0010 m_networkTableID
DT_SoundData!0x0014 m_soundIsStart
DT_SoundData!0x0018 m_seek
```
</details>
<details>
<summary><code>class DT_StatueProp extends DT_PhysicsProp</code></summary>

```
{
	m_hInitBaseAnimating: Int,
	m_bShatter: Int,
	m_nShatterFlags: Int,
	m_vShatterPosition: Vector,
	m_vShatterForce: Vector,
}
```

### Offsets

```
DT_StatueProp!0x1680 m_hInitBaseAnimating
DT_StatueProp!0x1684 m_bShatter
DT_StatueProp!0x1688 m_nShatterFlags
DT_StatueProp!0x168c m_vShatterPosition
DT_StatueProp!0x1698 m_vShatterForce
```
</details>
<details>
<summary><code>class DT_StatusEffectPlugin</code></summary>

```
{
	m_hOwnerEntity: Int,
	m_statusEffectsTimedPluginNV: DataTable,
	m_statusEffectsEndlessPluginNV: DataTable,
}
```

### Offsets

```
DT_StatusEffectPlugin!0x0480 m_hOwnerEntity
DT_StatusEffectPlugin!0x0a40 m_statusEffectsTimedPluginNV
DT_StatusEffectPlugin!0x0a58 m_statusEffectsEndlessPluginNV
```
</details>
<details>
<summary><code>class DT_TEBreakModel extends DT_BaseTempEntity</code></summary>

```
{
	m_vecOrigin: Vector,
	m_angRotation.x: Float,
	m_angRotation.y: Float,
	m_angRotation.z: Float,
	m_vecSize: Vector,
	m_vecVelocity: Vector,
	m_nRandomization: Int,
	m_nModelIndex: Int,
	m_nCount: Int,
	m_fTime: Float,
	m_nFlags: Int,
}
```

### Offsets

```
DT_TEBreakModel!0x0028 m_vecOrigin
DT_TEBreakModel!0x0034 m_angRotation.x
DT_TEBreakModel!0x0038 m_angRotation.y
DT_TEBreakModel!0x003c m_angRotation.z
DT_TEBreakModel!0x0040 m_vecSize
DT_TEBreakModel!0x004c m_vecVelocity
DT_TEBreakModel!0x0058 m_nRandomization
DT_TEBreakModel!0x005c m_nModelIndex
DT_TEBreakModel!0x0060 m_nCount
DT_TEBreakModel!0x0064 m_fTime
DT_TEBreakModel!0x0068 m_nFlags
```
</details>
<details>
<summary><code>class DT_TEExplosion extends DT_TEParticleSystem</code></summary>

```
{
	m_fScale: Float,
	m_nFrameRate: Int,
	m_nFlags: Int,
	m_vecNormal: Vector,
	m_chMaterialType: Int,
	m_nRadius: Int,
	m_nInnerRadius: Int,
	m_nMagnitude: Int,
	m_impactEffectTableIndex: Int,
	m_surfaceProp: Int,
	m_owner: Int,
	m_victim: Int,
}
```

### Offsets

```
DT_TEExplosion!0x0038 m_fScale
DT_TEExplosion!0x003c m_nFrameRate
DT_TEExplosion!0x0040 m_nFlags
DT_TEExplosion!0x0044 m_vecNormal
DT_TEExplosion!0x0050 m_chMaterialType
DT_TEExplosion!0x0054 m_nRadius
DT_TEExplosion!0x0058 m_nInnerRadius
DT_TEExplosion!0x005c m_nMagnitude
DT_TEExplosion!0x0060 m_impactEffectTableIndex
DT_TEExplosion!0x0064 m_surfaceProp
DT_TEExplosion!0x0068 m_owner
DT_TEExplosion!0x006c m_victim
```
</details>
<details>
<summary><code>class DT_TEPhysicsProp extends DT_BaseTempEntity</code></summary>

```
{
	m_vecOrigin: Vector,
	m_angRotation.x: Float,
	m_angRotation.y: Float,
	m_angRotation.z: Float,
	m_vecVelocity: Vector,
	m_nModelIndex: Int,
	m_nSkin: Int,
	m_nFlags: Int,
	m_nEffects: Int,
}
```

### Offsets

```
DT_TEPhysicsProp!0x0028 m_vecOrigin
DT_TEPhysicsProp!0x0034 m_angRotation.x
DT_TEPhysicsProp!0x0038 m_angRotation.y
DT_TEPhysicsProp!0x003c m_angRotation.z
DT_TEPhysicsProp!0x0040 m_vecVelocity
DT_TEPhysicsProp!0x004c m_nModelIndex
DT_TEPhysicsProp!0x0050 m_nSkin
DT_TEPhysicsProp!0x0054 m_nFlags
DT_TEPhysicsProp!0x0058 m_nEffects
```
</details>
<details>
<summary><code>class DT_TEProjectileTrail extends DT_BaseTempEntity</code></summary>

```
{
	m_owner: Int,
	m_startPos: Vector,
	m_endPos: Vector,
	m_weaponClassIndex: Int,
	m_modBitfield: Int,
	m_projectileTrailIndex: Int,
	m_impactEffectTable: Int,
}
```

### Offsets

```
DT_TEProjectileTrail!0x0028 m_owner
DT_TEProjectileTrail!0x002c m_startPos
DT_TEProjectileTrail!0x0038 m_endPos
DT_TEProjectileTrail!0x0044 m_weaponClassIndex
DT_TEProjectileTrail!0x0048 m_modBitfield
DT_TEProjectileTrail!0x004c m_projectileTrailIndex
DT_TEProjectileTrail!0x0050 m_impactEffectTable
```
</details>
<details>
<summary><code>class DT_TEScriptParticleSystem extends DT_BaseTempEntity</code></summary>

```
{
	m_effectIndex: Int,
	m_origin: Vector,
	m_angles: Vector,
	m_controlPoint1: Vector,
}
```

### Offsets

```
DT_TEScriptParticleSystem!0x0028 m_effectIndex
DT_TEScriptParticleSystem!0x002c m_origin
DT_TEScriptParticleSystem!0x0038 m_angles
DT_TEScriptParticleSystem!0x0044 m_controlPoint1
```
</details>
<details>
<summary><code>class DT_TEScriptParticleSystemOnEntity extends DT_BaseTempEntity</code></summary>

```
{
	m_effectIndex: Int,
	m_ent: Int,
	m_attachType: Int,
	m_attachmentIndex: Int,
	m_attachType2: Int,
	m_attachmentIndex2: Int,
}
```

### Offsets

```
DT_TEScriptParticleSystemOnEntity!0x0028 m_effectIndex
DT_TEScriptParticleSystemOnEntity!0x002c m_ent
DT_TEScriptParticleSystemOnEntity!0x0030 m_attachType
DT_TEScriptParticleSystemOnEntity!0x0034 m_attachmentIndex
DT_TEScriptParticleSystemOnEntity!0x0038 m_attachType2
DT_TEScriptParticleSystemOnEntity!0x003c m_attachmentIndex2
```
</details>
<details>
<summary><code>class DT_TEShatterSurface extends DT_BaseTempEntity</code></summary>

```
{
	m_vecOrigin: Vector,
	m_vecAngles: Vector,
	m_vecForce: Vector,
	m_vecForcePos: Vector,
	m_flWidth: Float,
	m_flHeight: Float,
	m_flShardSize: Float,
	m_nSurfaceType: Int,
}
```

### Offsets

```
DT_TEShatterSurface!0x0028 m_vecOrigin
DT_TEShatterSurface!0x0034 m_vecAngles
DT_TEShatterSurface!0x0040 m_vecForce
DT_TEShatterSurface!0x004c m_vecForcePos
DT_TEShatterSurface!0x0058 m_flWidth
DT_TEShatterSurface!0x005c m_flHeight
DT_TEShatterSurface!0x0060 m_flShardSize
DT_TEShatterSurface!0x0070 m_nSurfaceType
```
</details>
<details>
<summary><code>class DT_Team</code></summary>

```
{
	player_array_element: Int,
	"player_array": Array,
	m_score: Int,
	m_score2: Int,
	m_kills: Int,
	m_deaths: Int,
	m_iRoundsWon: Int,
	m_iTeamTeamNum: Int,
	m_szTeamname: String,
	m_reservedPlayerCount: Int,
	m_connectingPlayerCount: Int,
	m_loadingPlayerCount: Int,
}
```

### Offsets

```
DT_Team!0x0000 player_array_element
DT_Team!0x0000 "player_array"
DT_Team!0x0a40 m_score
DT_Team!0x0a44 m_score2
DT_Team!0x0a48 m_kills
DT_Team!0x0a4c m_deaths
DT_Team!0x0a50 m_iRoundsWon
DT_Team!0x0a54 m_iTeamTeamNum
DT_Team!0x0a78 m_szTeamname
DT_Team!0x0b78 m_reservedPlayerCount
DT_Team!0x0b7c m_connectingPlayerCount
DT_Team!0x0b80 m_loadingPlayerCount
```
</details>
<details>
<summary><code>class DT_ThirdPersonView</code></summary>

```
{
	m_thirdPersonEntViewOffset.x: Float,
	m_thirdPersonEntViewOffset.y: Float,
	m_thirdPersonEntViewOffset.z: Float,
	m_thirdPersonEntShouldViewAnglesFollowThirdPersonEnt: Int,
	m_thirdPersonEntPitchIsFreelook: Int,
	m_thirdPersonEntYawIsFreelook: Int,
	m_thirdPersonEntUseFixedDist: Int,
	m_thirdPersonEntFixedClientOnly: Int,
	m_thirdPersonEntPushedInByGeo: Int,
	m_thirdPersonEntDrawViewmodel: Int,
	m_thirdPersonEntEnableCameraLag: Int,
	m_thirdPersonEntFreezeLookControls: Int,
	m_thirdPersonEntBlendInTotalDuration: Float,
	m_thirdPersonEntBlendInEaseInDuration: Float,
	m_thirdPersonEntBlendInEaseOutDuration: Float,
	m_thirdPersonEntBlendOutDuration: Float,
	m_thirdPersonEntFixedPitch: Float,
	m_thirdPersonEntFixedYaw: Float,
	m_thirdPersonEntFixedDist: Float,
	m_thirdPersonEntFixedHeight: Float,
	m_thirdPersonEntFixedRight: Float,
	m_thirdPersonEntMinYaw: Float,
	m_thirdPersonEntMaxYaw: Float,
	m_thirdPersonEntMinPitch: Float,
	m_thirdPersonEntMaxPitch: Float,
	m_thirdPersonEntSpringToCenterRate: Float,
	m_thirdPersonEntSpringToCenterDelay: Float,
	m_thirdPersonEntLookaheadLowerEntSpeed: Float,
	m_thirdPersonEntLookaheadUpperEntSpeed: Float,
	m_thirdPersonEntLookaheadMaxAngle: Float,
	m_thirdPersonEntLookaheadLerpAheadRate: Float,
	m_thirdPersonEntLookaheadLerpToCenterRate: Float,
}
```

### Offsets

```
DT_ThirdPersonView!0x0000 m_thirdPersonEntViewOffset.x
DT_ThirdPersonView!0x0004 m_thirdPersonEntViewOffset.y
DT_ThirdPersonView!0x0008 m_thirdPersonEntViewOffset.z
DT_ThirdPersonView!0x000c m_thirdPersonEntShouldViewAnglesFollowThirdPersonEnt
DT_ThirdPersonView!0x000d m_thirdPersonEntPitchIsFreelook
DT_ThirdPersonView!0x000e m_thirdPersonEntYawIsFreelook
DT_ThirdPersonView!0x000f m_thirdPersonEntUseFixedDist
DT_ThirdPersonView!0x0010 m_thirdPersonEntFixedClientOnly
DT_ThirdPersonView!0x0011 m_thirdPersonEntPushedInByGeo
DT_ThirdPersonView!0x0012 m_thirdPersonEntDrawViewmodel
DT_ThirdPersonView!0x0013 m_thirdPersonEntEnableCameraLag
DT_ThirdPersonView!0x0014 m_thirdPersonEntFreezeLookControls
DT_ThirdPersonView!0x0018 m_thirdPersonEntBlendInTotalDuration
DT_ThirdPersonView!0x001c m_thirdPersonEntBlendInEaseInDuration
DT_ThirdPersonView!0x0020 m_thirdPersonEntBlendInEaseOutDuration
DT_ThirdPersonView!0x0024 m_thirdPersonEntBlendOutDuration
DT_ThirdPersonView!0x0028 m_thirdPersonEntFixedPitch
DT_ThirdPersonView!0x002c m_thirdPersonEntFixedYaw
DT_ThirdPersonView!0x0030 m_thirdPersonEntFixedDist
DT_ThirdPersonView!0x0034 m_thirdPersonEntFixedHeight
DT_ThirdPersonView!0x0038 m_thirdPersonEntFixedRight
DT_ThirdPersonView!0x0050 m_thirdPersonEntMinYaw
DT_ThirdPersonView!0x0054 m_thirdPersonEntMaxYaw
DT_ThirdPersonView!0x0058 m_thirdPersonEntMinPitch
DT_ThirdPersonView!0x005c m_thirdPersonEntMaxPitch
DT_ThirdPersonView!0x0060 m_thirdPersonEntSpringToCenterRate
DT_ThirdPersonView!0x0064 m_thirdPersonEntSpringToCenterDelay
DT_ThirdPersonView!0x0068 m_thirdPersonEntLookaheadLowerEntSpeed
DT_ThirdPersonView!0x006c m_thirdPersonEntLookaheadUpperEntSpeed
DT_ThirdPersonView!0x0070 m_thirdPersonEntLookaheadMaxAngle
DT_ThirdPersonView!0x0074 m_thirdPersonEntLookaheadLerpAheadRate
DT_ThirdPersonView!0x0078 m_thirdPersonEntLookaheadLerpToCenterRate
```
</details>
<details>
<summary><code>class DT_TitanSoul</code></summary>

```
{
	statuseffectsdata_soul: DT_TitanSoul_StatusEffects,
	m_bossPlayer: Int,
	m_shieldHealth: Int,
	m_shieldHealthMax: Int,
	m_networkedFlags: Int,
	m_titan: Int,
	m_titanSoulScriptNetData: Int,
	m_lastRodeoHitTime: Time,
	m_nextCoreChargeAvailable: Time,
	m_coreChargeExpireTime: Time,
	m_coreChargeStartTime: Time,
	m_coreUseDuration: Time,
	m_damageComboLatestUpdateTime: Time,
	m_damageComboStartHealth: Int,
	m_stance: Int,
	m_doomed: Int,
	m_playerSettingsNum: BitMask,
	m_invalidHealthBarEnt: Int,
	m_bEjecting: Int,
	m_isValidRodeoTarget: Int,
}
```

### Offsets

```
DT_TitanSoul!0x0000 statuseffectsdata_soul
DT_TitanSoul!0x0124 m_bossPlayer
DT_TitanSoul!0x0170 m_shieldHealth
DT_TitanSoul!0x0174 m_shieldHealthMax
DT_TitanSoul!0x03d4 m_networkedFlags
DT_TitanSoul!0x0a40 m_titan
DT_TitanSoul!0x0a48 m_titanSoulScriptNetData
DT_TitanSoul!0x0be0 m_lastRodeoHitTime
DT_TitanSoul!0x0be8 m_nextCoreChargeAvailable
DT_TitanSoul!0x0bf0 m_coreChargeExpireTime
DT_TitanSoul!0x0bf8 m_coreChargeStartTime
DT_TitanSoul!0x0bfc m_coreUseDuration
DT_TitanSoul!0x0c00 m_damageComboLatestUpdateTime
DT_TitanSoul!0x0c04 m_damageComboStartHealth
DT_TitanSoul!0x0da8 m_stance
DT_TitanSoul!0x0dac m_doomed
DT_TitanSoul!0x0db0 m_playerSettingsNum
DT_TitanSoul!0x0db8 m_invalidHealthBarEnt
DT_TitanSoul!0x0db9 m_bEjecting
DT_TitanSoul!0x0dba m_isValidRodeoTarget
```
</details>
<details>
<summary><code>class DT_TitanSoul_StatusEffects</code></summary>

```
{
	m_statusEffectsTimedTitanSoulNV: DataTable,
	m_statusEffectsEndlessTitanSoulNV: DataTable,
}
```

### Offsets

```
DT_TitanSoul_StatusEffects!0x0c08 m_statusEffectsTimedTitanSoulNV
DT_TitanSoul_StatusEffects!0x0cf8 m_statusEffectsEndlessTitanSoulNV
```
</details>
<details>
<summary><code>class DT_TriggerCylinderHeavy extends DT_BaseTrigger</code></summary>

```
{
	m_triggerFilterMask: BitMask,
	m_radius: Float,
	m_aboveHeight: Float,
	m_belowHeight: Float,
	m_teslaTrapBaseHeight: Float,
	m_vertOverride: Float,
	m_launchPower: Float,
	m_punchSoftAmount: Float,
	m_punchHardAmount: Float,
	m_punchRandomBoost: Float,
	m_crowdPusherRadius: Float,
	m_crowdPusherArcDeg: Float,
	m_crowdPusherPower: Float,
	m_crowdPusherShovePower: Float,
	m_triggerType: Int,
	m_teslaTrapFXVisible: Int,
	m_teslaTrapObstructedEndTime: Time,
	m_teslaTrapStart: Int,
	m_teslaTrapEnd: Int,
	m_teslaTrapUp: Vector,
	m_launchDir: Vector,
}
```

### Offsets

```
DT_TriggerCylinderHeavy!0x0ac0 m_triggerFilterMask
DT_TriggerCylinderHeavy!0x0ac8 m_radius
DT_TriggerCylinderHeavy!0x0acc m_aboveHeight
DT_TriggerCylinderHeavy!0x0ad0 m_belowHeight
DT_TriggerCylinderHeavy!0x0ad4 m_teslaTrapBaseHeight
DT_TriggerCylinderHeavy!0x0ad8 m_vertOverride
DT_TriggerCylinderHeavy!0x0adc m_launchPower
DT_TriggerCylinderHeavy!0x0ae0 m_punchSoftAmount
DT_TriggerCylinderHeavy!0x0ae4 m_punchHardAmount
DT_TriggerCylinderHeavy!0x0ae8 m_punchRandomBoost
DT_TriggerCylinderHeavy!0x0aec m_crowdPusherRadius
DT_TriggerCylinderHeavy!0x0af0 m_crowdPusherArcDeg
DT_TriggerCylinderHeavy!0x0af4 m_crowdPusherPower
DT_TriggerCylinderHeavy!0x0af8 m_crowdPusherShovePower
DT_TriggerCylinderHeavy!0x0afc m_triggerType
DT_TriggerCylinderHeavy!0x0b00 m_teslaTrapFXVisible
DT_TriggerCylinderHeavy!0x0b08 m_teslaTrapObstructedEndTime
DT_TriggerCylinderHeavy!0x0b0c m_teslaTrapStart
DT_TriggerCylinderHeavy!0x0b10 m_teslaTrapEnd
DT_TriggerCylinderHeavy!0x0b14 m_teslaTrapUp
DT_TriggerCylinderHeavy!0x0b20 m_launchDir
```
</details>
<details>
<summary><code>class DT_TriggerPointGravity extends DT_BaseTrigger</code></summary>

```
{
	m_pullOuterRadius: Float,
	m_pullInnerRadius: Float,
	m_reduceSpeedOuterRadius: Float,
	m_reduceSpeedInnerRadius: Float,
	m_pullAccel: Float,
	m_pullSpeed: Float,
}
```

### Offsets

```
DT_TriggerPointGravity!0x0ac0 m_pullOuterRadius
DT_TriggerPointGravity!0x0ac4 m_pullInnerRadius
DT_TriggerPointGravity!0x0ac8 m_reduceSpeedOuterRadius
DT_TriggerPointGravity!0x0acc m_reduceSpeedInnerRadius
DT_TriggerPointGravity!0x0ad0 m_pullAccel
DT_TriggerPointGravity!0x0ad4 m_pullSpeed
```
</details>
<details>
<summary><code>class DT_Turret extends DT_BaseAnimatingOverlay</code></summary>

```
{
	m_iHealth: Int,
	m_iMaxHealth: Int,
	m_overlayEventParity: DataTable,
	m_settingsIndex: Int,
	m_driver: Int,
	m_forceAimPitch: Float,
	m_forceAimYaw: Float,
	m_driverDetachTime: Float,
	m_driverState: Int,
	m_turretWeapon: Int,
	m_title: String,
}
```

### Offsets

```
DT_Turret!0x0420 m_iHealth
DT_Turret!0x0550 m_iMaxHealth
DT_Turret!0x1711 m_overlayEventParity
DT_Turret!0x1988 m_settingsIndex
DT_Turret!0x19b8 m_driver
DT_Turret!0x19e0 m_forceAimPitch
DT_Turret!0x19e4 m_forceAimYaw
DT_Turret!0x19e8 m_driverDetachTime
DT_Turret!0x19ec m_driverState
DT_Turret!0x19f0 m_turretWeapon
DT_Turret!0x19fc m_title
```
</details>
<details>
<summary><code>class DT_VGuiScreen extends DT_BaseEntity</code></summary>

```
{
	m_flWidth: Float,
	m_flHeight: Float,
	m_nPanelName: Int,
	m_nAttachmentIndex: Int,
	m_nOverlayMaterial: Int,
	m_fScreenFlags: Int,
	m_hPlayerOwner: Int,
}
```

### Offsets

```
DT_VGuiScreen!0x0a40 m_flWidth
DT_VGuiScreen!0x0a44 m_flHeight
DT_VGuiScreen!0x0a50 m_nPanelName
DT_VGuiScreen!0x0a6c m_nAttachmentIndex
DT_VGuiScreen!0x0a70 m_nOverlayMaterial
DT_VGuiScreen!0x0a74 m_fScreenFlags
DT_VGuiScreen!0x0ad8 m_hPlayerOwner
```
</details>
<details>
<summary><code>class DT_VehicleDriverExclusive</code></summary>

```
{
	m_localOrigin: VectorXY,
	m_localOrigin.z: Float,
	m_pushedFixedPointOffset: DataTable,
}
```

### Offsets

```
DT_VehicleDriverExclusive!0x0004 m_localOrigin
DT_VehicleDriverExclusive!0x000c m_localOrigin.z
DT_VehicleDriverExclusive!0x1b7c m_pushedFixedPointOffset
```
</details>
<details>
<summary><code>class DT_VehicleNonDriverExclusive</code></summary>

```
{
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: VectorXY,
	m_localOrigin.z: Float,
}
```

### Offsets

```
DT_VehicleNonDriverExclusive!0x004c m_cellX
DT_VehicleNonDriverExclusive!0x0050 m_cellY
DT_VehicleNonDriverExclusive!0x0054 m_cellZ
DT_VehicleNonDriverExclusive!0x0058 m_localOrigin
DT_VehicleNonDriverExclusive!0x0060 m_localOrigin.z
```
</details>
<details>
<summary><code>class DT_VortexSphere extends DT_BaseEntity</code></summary>

```
{
	m_spawnflags: Int,
	m_iHealth: Int,
	m_enabled: Int,
	m_radius: Float,
	m_height: Float,
	m_bulletFov: Float,
	m_bulletAbsorbedCount: Int,
	m_projectileAbsorbedCount: Int,
	m_ownerWeapon: Int,
	m_vortexEffect: Int,
	m_vortexLocalAngles: Vector,
	m_gunAttachment: String,
}
```

### Offsets

```
DT_VortexSphere!0x0094 m_spawnflags
DT_VortexSphere!0x0420 m_iHealth
DT_VortexSphere!0x0a40 m_enabled
DT_VortexSphere!0x0a44 m_radius
DT_VortexSphere!0x0a48 m_height
DT_VortexSphere!0x0a4c m_bulletFov
DT_VortexSphere!0x0a50 m_bulletAbsorbedCount
DT_VortexSphere!0x0a54 m_projectileAbsorbedCount
DT_VortexSphere!0x0a58 m_ownerWeapon
DT_VortexSphere!0x0a5c m_vortexEffect
DT_VortexSphere!0x0a60 m_vortexLocalAngles
DT_VortexSphere!0x0a70 m_gunAttachment
```
</details>
<details>
<summary><code>class DT_WeaponInventory</code></summary>

```
{
	weapons: DataTable,
	offhandWeapons: DataTable,
	activeWeapons: DataTable,
}
```

### Offsets

```
DT_WeaponInventory!0x0008 weapons
DT_WeaponInventory!0x0030 offhandWeapons
DT_WeaponInventory!0x0068 activeWeapons
```
</details>
<details>
<summary><code>class DT_WeaponInventoryActiveWeaponOnly</code></summary>

```
{
	activeWeapons: DataTable,
}
```

### Offsets

```
DT_WeaponInventoryActiveWeaponOnly!0x0068 activeWeapons
```
</details>
<details>
<summary><code>class DT_WeaponPlayerData</code></summary>

```
{
	m_moveSpread: Float,
	m_spreadStartTime: Time,
	m_spreadStartFracHip: Float,
	m_spreadStartFracADS: Float,
	m_kickSpreadHipfire: Float,
	m_kickSpreadADS: Float,
	m_kickTime: Time,
	m_kickScaleBasePitch: Float,
	m_kickScaleBaseYaw: Float,
	m_kickPatternScaleBase: Float,
	m_kickSpringHeatBaseTime: Time,
	m_kickSpringHeatBaseValue: Float,
	m_semiAutoTriggerHoldTime: Time,
	m_semiAutoTriggerDown: Int,
	m_pendingTriggerPull: Int,
	m_pendingTriggerUpForCharge: Int,
	m_semiAutoNeedsRechamber: Int,
	m_pendingReloadAttempt: Int,
	m_offhandHybridNormalMode: Int,
	m_pendingoffhandHybridToss: Int,
	m_fastHolster: Int,
	m_didFirstDeploy: Int,
	m_shouldCatch: Int,
	m_clipModelIsHidden: Int,
	m_segmentedReloadEndSeqRequired: Int,
	m_reloadStartedEmpty: Int,
	m_segmentedAnimStartedOneHanded: Int,
	m_segmentedReloadCanRestartLoop: Int,
	m_segmentedReloadLoopFireLocked: Int,
	m_realtimeModCmds: DataTable,
	m_realtimeModCmdHead: Int,
	m_realtimeModCmdCount: Int,
	m_customActivityAttachedModelIndex: Int,
	m_customActivityAttachedModelAttachmentIndex: Int,
	m_fireRateLerp_startTime: Time,
	m_fireRateLerp_startFraction: Float,
	m_fireRateLerp_stopTime: Time,
	m_fireRateLerp_stopFraction: Float,
	m_chargeAnimIndex: Int,
	m_chargeAnimIndexOld: Int,
	m_proScreen_owner: Int,
	m_proScreen_int0: Int,
	m_proScreen_int1: Int,
	m_proScreen_int2: Int,
	m_proScreen_float0: Float,
	m_proScreen_float1: Float,
	m_proScreen_float2: Float,
	m_reloadMilestone: Int,
	m_rechamberMilestone: Int,
	m_cooldownMilestone: Int,
	m_prevSeqWeight: Int,
	m_fullReloadStartTime: Time,
	m_scriptTime0: Time,
	m_scriptFlags0: Int,
	m_scriptInt0: Int,
	m_curZoomFOV: Float,
	m_targetZoomFOV: Float,
	m_zoomFOVLerpTime: Float,
	m_zoomFOVLerpEndTime: Time,
	m_latestDryfireTime: Time,
	m_requestedAttackEndTime: Time,
	m_currentAltFireAnimIndex: Int,
	m_legendaryModelIndex: Int,
	m_charmModelIndex: Int,
	m_charmAttachment: Int,
	m_charmScriptIndex: Int,
}
```

### Offsets

```
DT_WeaponPlayerData!0x0008 m_moveSpread
DT_WeaponPlayerData!0x000c m_spreadStartTime
DT_WeaponPlayerData!0x0010 m_spreadStartFracHip
DT_WeaponPlayerData!0x0014 m_spreadStartFracADS
DT_WeaponPlayerData!0x0018 m_kickSpreadHipfire
DT_WeaponPlayerData!0x001c m_kickSpreadADS
DT_WeaponPlayerData!0x0020 m_kickTime
DT_WeaponPlayerData!0x0024 m_kickScaleBasePitch
DT_WeaponPlayerData!0x0028 m_kickScaleBaseYaw
DT_WeaponPlayerData!0x002c m_kickPatternScaleBase
DT_WeaponPlayerData!0x0030 m_kickSpringHeatBaseTime
DT_WeaponPlayerData!0x0034 m_kickSpringHeatBaseValue
DT_WeaponPlayerData!0x0038 m_semiAutoTriggerHoldTime
DT_WeaponPlayerData!0x003c m_semiAutoTriggerDown
DT_WeaponPlayerData!0x003d m_pendingTriggerPull
DT_WeaponPlayerData!0x003e m_pendingTriggerUpForCharge
DT_WeaponPlayerData!0x003f m_semiAutoNeedsRechamber
DT_WeaponPlayerData!0x0040 m_pendingReloadAttempt
DT_WeaponPlayerData!0x0041 m_offhandHybridNormalMode
DT_WeaponPlayerData!0x0042 m_pendingoffhandHybridToss
DT_WeaponPlayerData!0x0043 m_fastHolster
DT_WeaponPlayerData!0x0044 m_didFirstDeploy
DT_WeaponPlayerData!0x0045 m_shouldCatch
DT_WeaponPlayerData!0x0046 m_clipModelIsHidden
DT_WeaponPlayerData!0x0047 m_segmentedReloadEndSeqRequired
DT_WeaponPlayerData!0x0048 m_reloadStartedEmpty
DT_WeaponPlayerData!0x0049 m_segmentedAnimStartedOneHanded
DT_WeaponPlayerData!0x004a m_segmentedReloadCanRestartLoop
DT_WeaponPlayerData!0x004b m_segmentedReloadLoopFireLocked
DT_WeaponPlayerData!0x004c m_realtimeModCmds
DT_WeaponPlayerData!0x0054 m_realtimeModCmdHead
DT_WeaponPlayerData!0x0055 m_realtimeModCmdCount
DT_WeaponPlayerData!0x0058 m_customActivityAttachedModelIndex
DT_WeaponPlayerData!0x005c m_customActivityAttachedModelAttachmentIndex
DT_WeaponPlayerData!0x0060 m_fireRateLerp_startTime
DT_WeaponPlayerData!0x0064 m_fireRateLerp_startFraction
DT_WeaponPlayerData!0x0068 m_fireRateLerp_stopTime
DT_WeaponPlayerData!0x006c m_fireRateLerp_stopFraction
DT_WeaponPlayerData!0x0070 m_chargeAnimIndex
DT_WeaponPlayerData!0x0074 m_chargeAnimIndexOld
DT_WeaponPlayerData!0x0078 m_proScreen_owner
DT_WeaponPlayerData!0x007c m_proScreen_int0
DT_WeaponPlayerData!0x0080 m_proScreen_int1
DT_WeaponPlayerData!0x0084 m_proScreen_int2
DT_WeaponPlayerData!0x0088 m_proScreen_float0
DT_WeaponPlayerData!0x008c m_proScreen_float1
DT_WeaponPlayerData!0x0090 m_proScreen_float2
DT_WeaponPlayerData!0x0094 m_reloadMilestone
DT_WeaponPlayerData!0x0098 m_rechamberMilestone
DT_WeaponPlayerData!0x009c m_cooldownMilestone
DT_WeaponPlayerData!0x00a0 m_prevSeqWeight
DT_WeaponPlayerData!0x00a4 m_fullReloadStartTime
DT_WeaponPlayerData!0x00a8 m_scriptTime0
DT_WeaponPlayerData!0x00ac m_scriptFlags0
DT_WeaponPlayerData!0x00b0 m_scriptInt0
DT_WeaponPlayerData!0x00b4 m_curZoomFOV
DT_WeaponPlayerData!0x00b8 m_targetZoomFOV
DT_WeaponPlayerData!0x00bc m_zoomFOVLerpTime
DT_WeaponPlayerData!0x00c0 m_zoomFOVLerpEndTime
DT_WeaponPlayerData!0x00c4 m_latestDryfireTime
DT_WeaponPlayerData!0x00c8 m_requestedAttackEndTime
DT_WeaponPlayerData!0x00cc m_currentAltFireAnimIndex
DT_WeaponPlayerData!0x00d0 m_legendaryModelIndex
DT_WeaponPlayerData!0x00d4 m_charmModelIndex
DT_WeaponPlayerData!0x00d8 m_charmAttachment
DT_WeaponPlayerData!0x00dc m_charmScriptIndex
```
</details>
<details>
<summary><code>class DT_WeaponX extends DT_BaseAnimating</code></summary>

```
{
	LocalWeaponData: DT_WeaponX_LocalWeaponData,
	predictingClientOnly: DT_WeaponX_PredictingClientOnly,
	m_networkedFlags: Int,
	m_bClientSideAnimation: Int,
	m_weaponOwner: Int,
	m_worldModelIndexOverride: Int,
	m_iWorldModelIndex: Int,
	m_holsterModelIndex: Int,
	m_droppedModelIndex: Int,
	m_nIdealSequence: Int,
	m_IdealActivity: Int,
	m_weaponActivity: Int,
	m_ActiveState: Int,
	m_weapState: Int,
	m_allowedToUse: Int,
	m_discarded: Int,
	m_forcedADS: Int,
	m_tossRelease: Int,
	m_customActivity: Int,
	m_customActivitySequence: Int,
	m_customActivityOwner: Int,
	m_customActivityEndTime: Time,
	m_customActivityFlags: Int,
	m_playerData: DT_WeaponPlayerData,
	m_lastTossedGrenade: Int,
	m_targetingLaserEnabledScript: Int,
	m_needsReloadCheck: Int,
	m_needsEmptyCycleCheck: Int,
	m_skinOverride: Int,
	m_skinOverrideIsValid: Int,
	m_chargeStartTime: Time,
	m_chargeEndTime: Time,
	m_lastChargeFrac: Float,
	m_sustainedDischargeEndTime: Time,
	m_sustainedLaserCurrentSpread: Float,
	m_sustainedDischargeIsInPrimaryAttack: Int,
	m_sustainedLaserNextRandomSeed: Int,
	m_modBitfieldFromPlayer: Int,
	m_modBitfieldInternal: Int,
	m_modBitfieldCurrent: Int,
	m_curSharedEnergyCost: Int,
	m_grappleWeaponNeedsDryfire: Int,
	m_scriptActivated: Int,
	m_isLoadoutPickup: Int,
	m_utilityEnt: Int,
	m_weaponNameIndex: Int,
	m_shouldPlayIdleAnims: Int,
	m_oaActiveOverride: Int,
	m_parentTurret: Int,
}
```

### Offsets

```
DT_WeaponX!0x0000 LocalWeaponData
DT_WeaponX!0x0000 predictingClientOnly
DT_WeaponX!0x03d4 m_networkedFlags
DT_WeaponX!0x1020 m_bClientSideAnimation
DT_WeaponX!0x1600 m_weaponOwner
DT_WeaponX!0x1614 m_worldModelIndexOverride
DT_WeaponX!0x1618 m_iWorldModelIndex
DT_WeaponX!0x161c m_holsterModelIndex
DT_WeaponX!0x1620 m_droppedModelIndex
DT_WeaponX!0x1624 m_nIdealSequence
DT_WeaponX!0x1628 m_IdealActivity
DT_WeaponX!0x162c m_weaponActivity
DT_WeaponX!0x1630 m_ActiveState
DT_WeaponX!0x1644 m_weapState
DT_WeaponX!0x1648 m_allowedToUse
DT_WeaponX!0x1649 m_discarded
DT_WeaponX!0x164c m_forcedADS
DT_WeaponX!0x1650 m_tossRelease
DT_WeaponX!0x1654 m_customActivity
DT_WeaponX!0x1658 m_customActivitySequence
DT_WeaponX!0x165c m_customActivityOwner
DT_WeaponX!0x1660 m_customActivityEndTime
DT_WeaponX!0x1664 m_customActivityFlags
DT_WeaponX!0x1668 m_playerData
DT_WeaponX!0x1748 m_lastTossedGrenade
DT_WeaponX!0x174c m_targetingLaserEnabledScript
DT_WeaponX!0x174d m_needsReloadCheck
DT_WeaponX!0x174e m_needsEmptyCycleCheck
DT_WeaponX!0x1750 m_skinOverride
DT_WeaponX!0x1754 m_skinOverrideIsValid
DT_WeaponX!0x1758 m_chargeStartTime
DT_WeaponX!0x175c m_chargeEndTime
DT_WeaponX!0x1760 m_lastChargeFrac
DT_WeaponX!0x1788 m_sustainedDischargeEndTime
DT_WeaponX!0x178c m_sustainedLaserCurrentSpread
DT_WeaponX!0x1790 m_sustainedDischargeIsInPrimaryAttack
DT_WeaponX!0x1791 m_sustainedLaserNextRandomSeed
DT_WeaponX!0x1794 m_modBitfieldFromPlayer
DT_WeaponX!0x1798 m_modBitfieldInternal
DT_WeaponX!0x179c m_modBitfieldCurrent
DT_WeaponX!0x17a0 m_curSharedEnergyCost
DT_WeaponX!0x17a4 m_grappleWeaponNeedsDryfire
DT_WeaponX!0x17a5 m_scriptActivated
DT_WeaponX!0x17a6 m_isLoadoutPickup
DT_WeaponX!0x17a8 m_utilityEnt
DT_WeaponX!0x17b0 m_weaponNameIndex
DT_WeaponX!0x17bc m_shouldPlayIdleAnims
DT_WeaponX!0x17c0 m_oaActiveOverride
DT_WeaponX!0x17c4 m_parentTurret
```
</details>
<details>
<summary><code>class DT_WeaponX_LocalWeaponData</code></summary>

```
{
	m_nNextThinkTick: Int,
	m_lastPrimaryAttack: Time,
	m_nextReadyTime: Time,
	m_nextPrimaryAttackTime: Time,
	m_attackTimeThisFrame: Time,
	m_ammoInClip: Int,
	m_ammoInStockpile: Int,
	m_lifetimeShots: Int,
	m_flTimeWeaponIdle: Time,
	m_bInReload: Int,
}
```

### Offsets

```
DT_WeaponX_LocalWeaponData!0x054c m_nNextThinkTick
DT_WeaponX_LocalWeaponData!0x1604 m_lastPrimaryAttack
DT_WeaponX_LocalWeaponData!0x1608 m_nextReadyTime
DT_WeaponX_LocalWeaponData!0x160c m_nextPrimaryAttackTime
DT_WeaponX_LocalWeaponData!0x1610 m_attackTimeThisFrame
DT_WeaponX_LocalWeaponData!0x1634 m_ammoInClip
DT_WeaponX_LocalWeaponData!0x1638 m_ammoInStockpile
DT_WeaponX_LocalWeaponData!0x163c m_lifetimeShots
DT_WeaponX_LocalWeaponData!0x1640 m_flTimeWeaponIdle
DT_WeaponX_LocalWeaponData!0x164a m_bInReload
```
</details>
<details>
<summary><code>class DT_WeaponX_PredictingClientOnly</code></summary>

```
{
	m_lastRegenTime: Time,
	m_cooldownEndTime: Time,
	m_stockPileWasDraining: Int,
	m_weaponIsCharging: Int,
	m_weaponChargeLevelIncreasedAnimPlaying: Int,
	m_lastChargeLevel: Int,
	m_chargeEnergyDepleteStepCounter: Int,
	m_burstFireCount: Int,
	m_burstFireIndex: Int,
	m_shotIndexForSpread: Int,
	m_shotCount: Int,
	m_animModelIndexPredictingClientOnly: Int,
	m_animSequencePredictingClientOnly: Int,
}
```

### Offsets

```
DT_WeaponX_PredictingClientOnly!0x1764 m_lastRegenTime
DT_WeaponX_PredictingClientOnly!0x1768 m_cooldownEndTime
DT_WeaponX_PredictingClientOnly!0x176c m_stockPileWasDraining
DT_WeaponX_PredictingClientOnly!0x176d m_weaponIsCharging
DT_WeaponX_PredictingClientOnly!0x176e m_weaponChargeLevelIncreasedAnimPlaying
DT_WeaponX_PredictingClientOnly!0x1770 m_lastChargeLevel
DT_WeaponX_PredictingClientOnly!0x1774 m_chargeEnergyDepleteStepCounter
DT_WeaponX_PredictingClientOnly!0x1778 m_burstFireCount
DT_WeaponX_PredictingClientOnly!0x177c m_burstFireIndex
DT_WeaponX_PredictingClientOnly!0x1780 m_shotIndexForSpread
DT_WeaponX_PredictingClientOnly!0x1784 m_shotCount
DT_WeaponX_PredictingClientOnly!0x17b4 m_animModelIndexPredictingClientOnly
DT_WeaponX_PredictingClientOnly!0x17b8 m_animSequencePredictingClientOnly
```
</details>
<details>
<summary><code>class DT_World extends DT_BaseEntity</code></summary>

```
{
	m_WorldMins: Vector,
	m_WorldMaxs: Vector,
	m_bStartDark: Int,
	m_statusEffectsGenerationNV: Int,
	m_worldFlags: Int,
	m_timeshiftArmDeviceSkin: Int,
	m_spTitanLoadoutUnlocks: Int,
	m_deathFieldIsActive: DataTable,
	m_deathFieldOrigin: DataTable,
	m_deathFieldRadiusStart: DataTable,
	m_deathFieldRadiusEnd: DataTable,
	m_deathFieldTimeStart: DataTable,
	m_deathFieldTimeEnd: DataTable,
	m_teamRelationRulesForPVE: Int,
	m_civilTeamsMaskA: DataTable,
	m_civilTeamsMaskB: DataTable,
	m_rabidTeamsMask: DataTable,
}
```

### Offsets

```
DT_World!0x0a40 m_WorldMins
DT_World!0x0a4c m_WorldMaxs
DT_World!0x0a58 m_bStartDark
DT_World!0x0a6c m_statusEffectsGenerationNV
DT_World!0x0a74 m_worldFlags
DT_World!0x0a78 m_timeshiftArmDeviceSkin
DT_World!0x0a7c m_spTitanLoadoutUnlocks
DT_World!0x0a80 m_deathFieldIsActive
DT_World!0x0ac0 m_deathFieldOrigin
DT_World!0x0dc0 m_deathFieldRadiusStart
DT_World!0x0ec0 m_deathFieldRadiusEnd
DT_World!0x0fc0 m_deathFieldTimeStart
DT_World!0x10c0 m_deathFieldTimeEnd
DT_World!0x11c0 m_teamRelationRulesForPVE
DT_World!0x11c8 m_civilTeamsMaskA
DT_World!0x11d8 m_civilTeamsMaskB
DT_World!0x11f0 m_rabidTeamsMask
```
</details>
<details>
<summary><code>class DT_Zipline extends DT_BaseEntity</code></summary>

```
{
	m_numZiplinePoints: Int,
	m_ziplinePositions: DataTable,
	m_ziplinePhysics: DT_ZiplinePhysics,
	m_ziplineMaterialIndex: Int,
	m_prevZipline: Int,
	m_nextZipline: Int,
	m_detachEndOnUse: Int,
	m_dropToBottom: Int,
	m_ziplineAutoDetachDistance: Float,
	m_ziplineVerticalPushOffInDirectionX: Int,
	m_ziplineVerticalPreserveVelocity: Int,
	m_ziplineWidth: Float,
	m_ziplineEnabled: Int,
	m_ziplineRestPositions: DataTable,
	m_numZiplineRestPositions: Int,
	m_ziplineFadeDist: Float,
	m_ziplineSpeedScale: Float,
}
```

### Offsets

```
DT_Zipline!0x0008 m_numZiplinePoints
DT_Zipline!0x000c m_ziplinePositions
DT_Zipline!0x0a40 m_ziplinePhysics
DT_Zipline!0x0d88 m_ziplineMaterialIndex
DT_Zipline!0x0d8c m_prevZipline
DT_Zipline!0x0d90 m_nextZipline
DT_Zipline!0x0d94 m_detachEndOnUse
DT_Zipline!0x0d95 m_dropToBottom
DT_Zipline!0x0d98 m_ziplineAutoDetachDistance
DT_Zipline!0x0d9c m_ziplineVerticalPushOffInDirectionX
DT_Zipline!0x0d9d m_ziplineVerticalPreserveVelocity
DT_Zipline!0x0da0 m_ziplineWidth
DT_Zipline!0x0da4 m_ziplineEnabled
DT_Zipline!0x0da8 m_ziplineRestPositions
DT_Zipline!0x0e68 m_numZiplineRestPositions
DT_Zipline!0x0e6c m_ziplineFadeDist
DT_Zipline!0x0e70 m_ziplineSpeedScale
```
</details>
<details>
<summary><code>class DT_ZiplinePhysics</code></summary>

```
{
	ziplinephysicsexclusive: DT_ZiplinePhysicsExlusive,
	m_isInit: Int,
	m_ziplineType: Int,
	m_ziplineStart: Vector,
	m_ziplineEnd: Vector,
	m_springDistance: Float,
	m_springDistanceScale: Float,
	m_outerZiplineEntity: Int,
	m_attachedEntities: DataTable,
	m_numAttachedEntities: Int,
	m_ziplineOwner: Int,
}
```

### Offsets

```
DT_ZiplinePhysics!0x0000 ziplinephysicsexclusive
DT_ZiplinePhysics!0x0008 m_isInit
DT_ZiplinePhysics!0x000c m_ziplineType
DT_ZiplinePhysics!0x0010 m_ziplineStart
DT_ZiplinePhysics!0x001c m_ziplineEnd
DT_ZiplinePhysics!0x022c m_springDistance
DT_ZiplinePhysics!0x0230 m_springDistanceScale
DT_ZiplinePhysics!0x0238 m_outerZiplineEntity
DT_ZiplinePhysics!0x0240 m_attachedEntities
DT_ZiplinePhysics!0x0340 m_numAttachedEntities
DT_ZiplinePhysics!0x0344 m_ziplineOwner
```
</details>
<details>
<summary><code>class DT_ZiplinePhysicsExlusive</code></summary>

```
{
	m_nodes: DataTable,
	m_numNodes: Int,
	m_remainingUnsimulatedTime: Float,
}
```

### Offsets

```
DT_ZiplinePhysicsExlusive!0x0028 m_nodes
DT_ZiplinePhysicsExlusive!0x0228 m_numNodes
DT_ZiplinePhysicsExlusive!0x0234 m_remainingUnsimulatedTime
```
</details>

## Datamaps

<details>
<summary><code>class CBaseGrenade extends C_BaseAnimating</code></summary>

```
{
	m_vecVelocity: Vector,
	m_doesExplode: Bool,
	m_DmgRadius: Bool,
	m_grenadeCreationTime: Float,
	m_grenadeCreationOrigin: Vector,
	m_useMaskAbility: Bool,
	m_grenadeStatusFlags: Int,
	m_flDamage: Float,
	m_hThrower: EHANDLE,
}
```

### Offsets

```
CBaseGrenade!0x0460 m_vecVelocity
CBaseGrenade!0x2c41 m_doesExplode
CBaseGrenade!0x2c44 m_DmgRadius
CBaseGrenade!0x2c54 m_grenadeCreationTime
CBaseGrenade!0x2c58 m_grenadeCreationOrigin
CBaseGrenade!0x2c64 m_useMaskAbility
CBaseGrenade!0x2c68 m_grenadeStatusFlags
CBaseGrenade!0x2cf0 m_flDamage
CBaseGrenade!0x2cf4 m_hThrower
```
</details>
<details>
<summary><code>class CBaseViewModel</code></summary>

```
{
	m_currentFrame.modelIndex: Short,
	m_currentFrame.animCycle: Float,
	m_angAbsRotation: Vector,
	m_vecAbsOrigin: Vector,
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_fEffects: Int,
	m_angNetworkAngles: Vector,
	m_nBody: Int,
	m_nResetEventsParity: Int,
	m_bSequenceFinished: Bool,
	m_currentFrameBaseAnimating.animStartTime: Float,
	m_currentFrameBaseAnimating.animStartCycle: Float,
	m_currentFrameBaseAnimating.animPlaybackRate: Float,
	m_currentFrameBaseAnimating.animModelIndex: Int,
	m_currentFrameBaseAnimating.animSequence: Int,
	m_currentFrameBaseAnimating.animSequenceParity: Int,
	m_currentFrameBaseAnimating.m_flPoseParameters: Float,
	m_currentFrameAnimatingOverlay.animOverlayIsActive: Bool,
	m_currentFrameAnimatingOverlay.animOverlayStartTime: Float,
	m_currentFrameAnimatingOverlay.animOverlayStartCycle: Float,
	m_currentFrameAnimatingOverlay.animOverlayPlaybackRate: Float,
	m_currentFrameAnimatingOverlay.animOverlayModelIndex: Int,
	m_currentFrameAnimatingOverlay.animOverlaySequence: Int,
	m_currentFrameAnimatingOverlay.animOverlayWeight: Float,
	m_currentFrameAnimatingOverlay.animOverlayAnimTime: Float,
	m_currentFrameAnimatingOverlay.animOverlayFadeInDuration: Float,
	m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration: Float,
	m_currentFrameAnimatingOverlay.animOverlayCycle: Float,
	m_viewModelOwner: EHANDLE,
	m_projectileIsVisible: Bool,
	m_bBlockEventLayer: Bool,
	m_isAdsTransition: Bool,
	m_hWeapon: EHANDLE,
	m_tracerAttachments: Int,
	m_tracerAttachments: Int,
	m_tracerAttachmentsScoped: Int,
	m_tracerAttachmentsScoped: Int,
}
```

### Offsets

```
CBaseViewModel!0x00a8 m_currentFrame.modelIndex
CBaseViewModel!0x00c4 m_currentFrame.animCycle
CBaseViewModel!0x0134 m_angAbsRotation
CBaseViewModel!0x014c m_vecAbsOrigin
CBaseViewModel!0x0158 m_localOrigin
CBaseViewModel!0x0164 m_localAngles
CBaseViewModel!0x042c m_fEffects
CBaseViewModel!0x046c m_angNetworkAngles
CBaseViewModel!0x0e90 m_nBody
CBaseViewModel!0x0e9c m_nResetEventsParity
CBaseViewModel!0x0f34 m_bSequenceFinished
CBaseViewModel!0x0f4c m_currentFrameBaseAnimating.animStartTime
CBaseViewModel!0x0f50 m_currentFrameBaseAnimating.animStartCycle
CBaseViewModel!0x0f54 m_currentFrameBaseAnimating.animPlaybackRate
CBaseViewModel!0x0f5c m_currentFrameBaseAnimating.animModelIndex
CBaseViewModel!0x0f60 m_currentFrameBaseAnimating.animSequence
CBaseViewModel!0x0f64 m_currentFrameBaseAnimating.animSequenceParity
CBaseViewModel!0x0f68 m_currentFrameBaseAnimating.m_flPoseParameters
CBaseViewModel!0x1724 m_currentFrameAnimatingOverlay.animOverlayIsActive
CBaseViewModel!0x1730 m_currentFrameAnimatingOverlay.animOverlayStartTime
CBaseViewModel!0x1754 m_currentFrameAnimatingOverlay.animOverlayStartCycle
CBaseViewModel!0x1778 m_currentFrameAnimatingOverlay.animOverlayPlaybackRate
CBaseViewModel!0x179c m_currentFrameAnimatingOverlay.animOverlayModelIndex
CBaseViewModel!0x17c0 m_currentFrameAnimatingOverlay.animOverlaySequence
CBaseViewModel!0x17e4 m_currentFrameAnimatingOverlay.animOverlayWeight
CBaseViewModel!0x182c m_currentFrameAnimatingOverlay.animOverlayAnimTime
CBaseViewModel!0x1850 m_currentFrameAnimatingOverlay.animOverlayFadeInDuration
CBaseViewModel!0x1874 m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration
CBaseViewModel!0x1898 m_currentFrameAnimatingOverlay.animOverlayCycle
CBaseViewModel!0x19d8 m_viewModelOwner
CBaseViewModel!0x19dc m_projectileIsVisible
CBaseViewModel!0x1dc0 m_bBlockEventLayer
CBaseViewModel!0x1dc1 m_isAdsTransition
CBaseViewModel!0x1dc4 m_hWeapon
CBaseViewModel!0x1dc8 m_tracerAttachments
CBaseViewModel!0x1dc8 m_tracerAttachments
CBaseViewModel!0x1dd0 m_tracerAttachmentsScoped
CBaseViewModel!0x1dd0 m_tracerAttachmentsScoped
```
</details>
<details>
<summary><code>class CCollisionProperty</code></summary>

```
{
	m_vecMins: Vector,
	m_vecMaxs: Vector,
	m_usSolidFlags: Int,
	m_nSolidType: Char,
	m_triggerBloat: Char,
	m_collisionDetailLevel: Char,
}
```

### Offsets

```
CCollisionProperty!0x0010 m_vecMins
CCollisionProperty!0x001c m_vecMaxs
CCollisionProperty!0x0028 m_usSolidFlags
CCollisionProperty!0x002c m_nSolidType
CCollisionProperty!0x002d m_triggerBloat
CCollisionProperty!0x002e m_collisionDetailLevel
```
</details>
<details>
<summary><code>class CGrappleHook</code></summary>

```
{
	m_pMoveParent: EHANDLE,
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_visibilityFlags: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_grappleZipline: EHANDLE,
}
```

### Offsets

```
CGrappleHook!0x0118 m_pMoveParent
CGrappleHook!0x0158 m_localOrigin
CGrappleHook!0x0164 m_localAngles
CGrappleHook!0x0428 m_visibilityFlags
CGrappleHook!0x082c m_parentAttachmentType
CGrappleHook!0x0830 m_parentAttachmentIndex
CGrappleHook!0x0834 m_parentAttachmentHitbox
CGrappleHook!0x1600 m_grappleZipline
```
</details>
<details>
<summary><code>class CPlayerShared</code></summary>

```
{
	m_nPlayerCond: Int,
}
```

### Offsets

```
CPlayerShared!0x0008 m_nPlayerCond
```
</details>
<details>
<summary><code>class CPlayerState</code></summary>

```
{
	deadflag: Bool,
}
```

### Offsets

```
CPlayerState!0x006c deadflag
```
</details>
<details>
<summary><code>class CPredictedFirstPersonProxy extends C_BaseAnimating</code></summary>

```
{
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_vecVelocity: Vector,
	m_angNetworkAngles: Vector,
	m_SequenceTransitioner: C_SequenceTransitioner,
	m_camoIndex: Int,
}
```

### Offsets

```
CPredictedFirstPersonProxy!0x0158 m_localOrigin
CPredictedFirstPersonProxy!0x0164 m_localAngles
CPredictedFirstPersonProxy!0x0460 m_vecVelocity
CPredictedFirstPersonProxy!0x046c m_angNetworkAngles
CPredictedFirstPersonProxy!0x0c00 m_SequenceTransitioner
CPredictedFirstPersonProxy!0x0e94 m_camoIndex
```
</details>
<details>
<summary><code>class CRagdoll</code></summary>

```
{
	m_ragdoll.listCount: Int,
	m_ragdoll.allowStretch: Bool,
	m_ragdoll.list[0 + 0].originParentSpace: Vector,
	m_ragdoll.list[0 + 0].pObject: Custom,
	m_ragdoll.list[0 + 0].pConstraint: Custom,
	m_ragdoll.list[0 + 0].parentIndex: Int,
	m_ragdoll.list[0 + 1].originParentSpace: Vector,
	m_ragdoll.list[0 + 1].pObject: Custom,
	m_ragdoll.list[0 + 1].pConstraint: Custom,
	m_ragdoll.list[0 + 1].parentIndex: Int,
	m_ragdoll.list[0 + 2].originParentSpace: Vector,
	m_ragdoll.list[0 + 2].pObject: Custom,
	m_ragdoll.list[0 + 2].pConstraint: Custom,
	m_ragdoll.list[0 + 2].parentIndex: Int,
	m_ragdoll.list[0 + 3].originParentSpace: Vector,
	m_ragdoll.list[0 + 3].pObject: Custom,
	m_ragdoll.list[0 + 3].pConstraint: Custom,
	m_ragdoll.list[0 + 3].parentIndex: Int,
	m_ragdoll.list[0 + 4].originParentSpace: Vector,
	m_ragdoll.list[0 + 4].pObject: Custom,
	m_ragdoll.list[0 + 4].pConstraint: Custom,
	m_ragdoll.list[0 + 4].parentIndex: Int,
	m_ragdoll.list[0 + 5].originParentSpace: Vector,
	m_ragdoll.list[0 + 5].pObject: Custom,
	m_ragdoll.list[0 + 5].pConstraint: Custom,
	m_ragdoll.list[0 + 5].parentIndex: Int,
	m_ragdoll.list[0 + 6].originParentSpace: Vector,
	m_ragdoll.list[0 + 6].pObject: Custom,
	m_ragdoll.list[0 + 6].pConstraint: Custom,
	m_ragdoll.list[0 + 6].parentIndex: Int,
	m_ragdoll.list[0 + 7].originParentSpace: Vector,
	m_ragdoll.list[0 + 7].pObject: Custom,
	m_ragdoll.list[0 + 7].pConstraint: Custom,
	m_ragdoll.list[0 + 7].parentIndex: Int,
	m_ragdoll.list[8 + 0].originParentSpace: Vector,
	m_ragdoll.list[8 + 0].pObject: Custom,
	m_ragdoll.list[8 + 0].pConstraint: Custom,
	m_ragdoll.list[8 + 0].parentIndex: Int,
	m_ragdoll.list[8 + 1].originParentSpace: Vector,
	m_ragdoll.list[8 + 1].pObject: Custom,
	m_ragdoll.list[8 + 1].pConstraint: Custom,
	m_ragdoll.list[8 + 1].parentIndex: Int,
	m_ragdoll.list[8 + 2].originParentSpace: Vector,
	m_ragdoll.list[8 + 2].pObject: Custom,
	m_ragdoll.list[8 + 2].pConstraint: Custom,
	m_ragdoll.list[8 + 2].parentIndex: Int,
	m_ragdoll.list[8 + 3].originParentSpace: Vector,
	m_ragdoll.list[8 + 3].pObject: Custom,
	m_ragdoll.list[8 + 3].pConstraint: Custom,
	m_ragdoll.list[8 + 3].parentIndex: Int,
	m_ragdoll.list[8 + 4].originParentSpace: Vector,
	m_ragdoll.list[8 + 4].pObject: Custom,
	m_ragdoll.list[8 + 4].pConstraint: Custom,
	m_ragdoll.list[8 + 4].parentIndex: Int,
	m_ragdoll.list[8 + 5].originParentSpace: Vector,
	m_ragdoll.list[8 + 5].pObject: Custom,
	m_ragdoll.list[8 + 5].pConstraint: Custom,
	m_ragdoll.list[8 + 5].parentIndex: Int,
	m_ragdoll.list[8 + 6].originParentSpace: Vector,
	m_ragdoll.list[8 + 6].pObject: Custom,
	m_ragdoll.list[8 + 6].pConstraint: Custom,
	m_ragdoll.list[8 + 6].parentIndex: Int,
	m_ragdoll.list[8 + 7].originParentSpace: Vector,
	m_ragdoll.list[8 + 7].pObject: Custom,
	m_ragdoll.list[8 + 7].pConstraint: Custom,
	m_ragdoll.list[8 + 7].parentIndex: Int,
	m_ragdoll.list[16 + 0].originParentSpace: Vector,
	m_ragdoll.list[16 + 0].pObject: Custom,
	m_ragdoll.list[16 + 0].pConstraint: Custom,
	m_ragdoll.list[16 + 0].parentIndex: Int,
	m_ragdoll.list[16 + 1].originParentSpace: Vector,
	m_ragdoll.list[16 + 1].pObject: Custom,
	m_ragdoll.list[16 + 1].pConstraint: Custom,
	m_ragdoll.list[16 + 1].parentIndex: Int,
	m_ragdoll.list[16 + 2].originParentSpace: Vector,
	m_ragdoll.list[16 + 2].pObject: Custom,
	m_ragdoll.list[16 + 2].pConstraint: Custom,
	m_ragdoll.list[16 + 2].parentIndex: Int,
	m_ragdoll.list[16 + 3].originParentSpace: Vector,
	m_ragdoll.list[16 + 3].pObject: Custom,
	m_ragdoll.list[16 + 3].pConstraint: Custom,
	m_ragdoll.list[16 + 3].parentIndex: Int,
	m_ragdoll.list[16 + 4].originParentSpace: Vector,
	m_ragdoll.list[16 + 4].pObject: Custom,
	m_ragdoll.list[16 + 4].pConstraint: Custom,
	m_ragdoll.list[16 + 4].parentIndex: Int,
	m_ragdoll.list[16 + 5].originParentSpace: Vector,
	m_ragdoll.list[16 + 5].pObject: Custom,
	m_ragdoll.list[16 + 5].pConstraint: Custom,
	m_ragdoll.list[16 + 5].parentIndex: Int,
	m_ragdoll.list[16 + 6].originParentSpace: Vector,
	m_ragdoll.list[16 + 6].pObject: Custom,
	m_ragdoll.list[16 + 6].pConstraint: Custom,
	m_ragdoll.list[16 + 6].parentIndex: Int,
	m_ragdoll.list[16 + 7].originParentSpace: Vector,
	m_ragdoll.list[16 + 7].pObject: Custom,
	m_ragdoll.list[16 + 7].pConstraint: Custom,
	m_ragdoll.list[16 + 7].parentIndex: Int,
	m_ragdoll.list[24 + 0].originParentSpace: Vector,
	m_ragdoll.list[24 + 0].pObject: Custom,
	m_ragdoll.list[24 + 0].pConstraint: Custom,
	m_ragdoll.list[24 + 0].parentIndex: Int,
	m_ragdoll.list[24 + 1].originParentSpace: Vector,
	m_ragdoll.list[24 + 1].pObject: Custom,
	m_ragdoll.list[24 + 1].pConstraint: Custom,
	m_ragdoll.list[24 + 1].parentIndex: Int,
	m_ragdoll.list[24 + 2].originParentSpace: Vector,
	m_ragdoll.list[24 + 2].pObject: Custom,
	m_ragdoll.list[24 + 2].pConstraint: Custom,
	m_ragdoll.list[24 + 2].parentIndex: Int,
	m_ragdoll.list[24 + 3].originParentSpace: Vector,
	m_ragdoll.list[24 + 3].pObject: Custom,
	m_ragdoll.list[24 + 3].pConstraint: Custom,
	m_ragdoll.list[24 + 3].parentIndex: Int,
	m_ragdoll.list[24 + 4].originParentSpace: Vector,
	m_ragdoll.list[24 + 4].pObject: Custom,
	m_ragdoll.list[24 + 4].pConstraint: Custom,
	m_ragdoll.list[24 + 4].parentIndex: Int,
	m_ragdoll.list[24 + 5].originParentSpace: Vector,
	m_ragdoll.list[24 + 5].pObject: Custom,
	m_ragdoll.list[24 + 5].pConstraint: Custom,
	m_ragdoll.list[24 + 5].parentIndex: Int,
	m_ragdoll.list[24 + 6].originParentSpace: Vector,
	m_ragdoll.list[24 + 6].pObject: Custom,
	m_ragdoll.list[24 + 6].pConstraint: Custom,
	m_ragdoll.list[24 + 6].parentIndex: Int,
	m_ragdoll.list[24 + 7].originParentSpace: Vector,
	m_ragdoll.list[24 + 7].pObject: Custom,
	m_ragdoll.list[24 + 7].pConstraint: Custom,
	m_ragdoll.list[24 + 7].parentIndex: Int,
	m_ragdoll.boneIndex: Int,
}
```

### Offsets

```
CRagdoll!0x0000 m_ragdoll.listCount
CRagdoll!0x0004 m_ragdoll.allowStretch
CRagdoll!0x0008 m_ragdoll.list[0 + 0].originParentSpace
CRagdoll!0x0018 m_ragdoll.list[0 + 0].pObject
CRagdoll!0x0020 m_ragdoll.list[0 + 0].pConstraint
CRagdoll!0x0028 m_ragdoll.list[0 + 0].parentIndex
CRagdoll!0x0030 m_ragdoll.list[0 + 1].originParentSpace
CRagdoll!0x0040 m_ragdoll.list[0 + 1].pObject
CRagdoll!0x0048 m_ragdoll.list[0 + 1].pConstraint
CRagdoll!0x0050 m_ragdoll.list[0 + 1].parentIndex
CRagdoll!0x0058 m_ragdoll.list[0 + 2].originParentSpace
CRagdoll!0x0068 m_ragdoll.list[0 + 2].pObject
CRagdoll!0x0070 m_ragdoll.list[0 + 2].pConstraint
CRagdoll!0x0078 m_ragdoll.list[0 + 2].parentIndex
CRagdoll!0x0080 m_ragdoll.list[0 + 3].originParentSpace
CRagdoll!0x0090 m_ragdoll.list[0 + 3].pObject
CRagdoll!0x0098 m_ragdoll.list[0 + 3].pConstraint
CRagdoll!0x00a0 m_ragdoll.list[0 + 3].parentIndex
CRagdoll!0x00a8 m_ragdoll.list[0 + 4].originParentSpace
CRagdoll!0x00b8 m_ragdoll.list[0 + 4].pObject
CRagdoll!0x00c0 m_ragdoll.list[0 + 4].pConstraint
CRagdoll!0x00c8 m_ragdoll.list[0 + 4].parentIndex
CRagdoll!0x00d0 m_ragdoll.list[0 + 5].originParentSpace
CRagdoll!0x00e0 m_ragdoll.list[0 + 5].pObject
CRagdoll!0x00e8 m_ragdoll.list[0 + 5].pConstraint
CRagdoll!0x00f0 m_ragdoll.list[0 + 5].parentIndex
CRagdoll!0x00f8 m_ragdoll.list[0 + 6].originParentSpace
CRagdoll!0x0108 m_ragdoll.list[0 + 6].pObject
CRagdoll!0x0110 m_ragdoll.list[0 + 6].pConstraint
CRagdoll!0x0118 m_ragdoll.list[0 + 6].parentIndex
CRagdoll!0x0120 m_ragdoll.list[0 + 7].originParentSpace
CRagdoll!0x0130 m_ragdoll.list[0 + 7].pObject
CRagdoll!0x0138 m_ragdoll.list[0 + 7].pConstraint
CRagdoll!0x0140 m_ragdoll.list[0 + 7].parentIndex
CRagdoll!0x0148 m_ragdoll.list[8 + 0].originParentSpace
CRagdoll!0x0158 m_ragdoll.list[8 + 0].pObject
CRagdoll!0x0160 m_ragdoll.list[8 + 0].pConstraint
CRagdoll!0x0168 m_ragdoll.list[8 + 0].parentIndex
CRagdoll!0x0170 m_ragdoll.list[8 + 1].originParentSpace
CRagdoll!0x0180 m_ragdoll.list[8 + 1].pObject
CRagdoll!0x0188 m_ragdoll.list[8 + 1].pConstraint
CRagdoll!0x0190 m_ragdoll.list[8 + 1].parentIndex
CRagdoll!0x0198 m_ragdoll.list[8 + 2].originParentSpace
CRagdoll!0x01a8 m_ragdoll.list[8 + 2].pObject
CRagdoll!0x01b0 m_ragdoll.list[8 + 2].pConstraint
CRagdoll!0x01b8 m_ragdoll.list[8 + 2].parentIndex
CRagdoll!0x01c0 m_ragdoll.list[8 + 3].originParentSpace
CRagdoll!0x01d0 m_ragdoll.list[8 + 3].pObject
CRagdoll!0x01d8 m_ragdoll.list[8 + 3].pConstraint
CRagdoll!0x01e0 m_ragdoll.list[8 + 3].parentIndex
CRagdoll!0x01e8 m_ragdoll.list[8 + 4].originParentSpace
CRagdoll!0x01f8 m_ragdoll.list[8 + 4].pObject
CRagdoll!0x0200 m_ragdoll.list[8 + 4].pConstraint
CRagdoll!0x0208 m_ragdoll.list[8 + 4].parentIndex
CRagdoll!0x0210 m_ragdoll.list[8 + 5].originParentSpace
CRagdoll!0x0220 m_ragdoll.list[8 + 5].pObject
CRagdoll!0x0228 m_ragdoll.list[8 + 5].pConstraint
CRagdoll!0x0230 m_ragdoll.list[8 + 5].parentIndex
CRagdoll!0x0238 m_ragdoll.list[8 + 6].originParentSpace
CRagdoll!0x0248 m_ragdoll.list[8 + 6].pObject
CRagdoll!0x0250 m_ragdoll.list[8 + 6].pConstraint
CRagdoll!0x0258 m_ragdoll.list[8 + 6].parentIndex
CRagdoll!0x0260 m_ragdoll.list[8 + 7].originParentSpace
CRagdoll!0x0270 m_ragdoll.list[8 + 7].pObject
CRagdoll!0x0278 m_ragdoll.list[8 + 7].pConstraint
CRagdoll!0x0280 m_ragdoll.list[8 + 7].parentIndex
CRagdoll!0x0288 m_ragdoll.list[16 + 0].originParentSpace
CRagdoll!0x0298 m_ragdoll.list[16 + 0].pObject
CRagdoll!0x02a0 m_ragdoll.list[16 + 0].pConstraint
CRagdoll!0x02a8 m_ragdoll.list[16 + 0].parentIndex
CRagdoll!0x02b0 m_ragdoll.list[16 + 1].originParentSpace
CRagdoll!0x02c0 m_ragdoll.list[16 + 1].pObject
CRagdoll!0x02c8 m_ragdoll.list[16 + 1].pConstraint
CRagdoll!0x02d0 m_ragdoll.list[16 + 1].parentIndex
CRagdoll!0x02d8 m_ragdoll.list[16 + 2].originParentSpace
CRagdoll!0x02e8 m_ragdoll.list[16 + 2].pObject
CRagdoll!0x02f0 m_ragdoll.list[16 + 2].pConstraint
CRagdoll!0x02f8 m_ragdoll.list[16 + 2].parentIndex
CRagdoll!0x0300 m_ragdoll.list[16 + 3].originParentSpace
CRagdoll!0x0310 m_ragdoll.list[16 + 3].pObject
CRagdoll!0x0318 m_ragdoll.list[16 + 3].pConstraint
CRagdoll!0x0320 m_ragdoll.list[16 + 3].parentIndex
CRagdoll!0x0328 m_ragdoll.list[16 + 4].originParentSpace
CRagdoll!0x0338 m_ragdoll.list[16 + 4].pObject
CRagdoll!0x0340 m_ragdoll.list[16 + 4].pConstraint
CRagdoll!0x0348 m_ragdoll.list[16 + 4].parentIndex
CRagdoll!0x0350 m_ragdoll.list[16 + 5].originParentSpace
CRagdoll!0x0360 m_ragdoll.list[16 + 5].pObject
CRagdoll!0x0368 m_ragdoll.list[16 + 5].pConstraint
CRagdoll!0x0370 m_ragdoll.list[16 + 5].parentIndex
CRagdoll!0x0378 m_ragdoll.list[16 + 6].originParentSpace
CRagdoll!0x0388 m_ragdoll.list[16 + 6].pObject
CRagdoll!0x0390 m_ragdoll.list[16 + 6].pConstraint
CRagdoll!0x0398 m_ragdoll.list[16 + 6].parentIndex
CRagdoll!0x03a0 m_ragdoll.list[16 + 7].originParentSpace
CRagdoll!0x03b0 m_ragdoll.list[16 + 7].pObject
CRagdoll!0x03b8 m_ragdoll.list[16 + 7].pConstraint
CRagdoll!0x03c0 m_ragdoll.list[16 + 7].parentIndex
CRagdoll!0x03c8 m_ragdoll.list[24 + 0].originParentSpace
CRagdoll!0x03d8 m_ragdoll.list[24 + 0].pObject
CRagdoll!0x03e0 m_ragdoll.list[24 + 0].pConstraint
CRagdoll!0x03e8 m_ragdoll.list[24 + 0].parentIndex
CRagdoll!0x03f0 m_ragdoll.list[24 + 1].originParentSpace
CRagdoll!0x0400 m_ragdoll.list[24 + 1].pObject
CRagdoll!0x0408 m_ragdoll.list[24 + 1].pConstraint
CRagdoll!0x0410 m_ragdoll.list[24 + 1].parentIndex
CRagdoll!0x0418 m_ragdoll.list[24 + 2].originParentSpace
CRagdoll!0x0428 m_ragdoll.list[24 + 2].pObject
CRagdoll!0x0430 m_ragdoll.list[24 + 2].pConstraint
CRagdoll!0x0438 m_ragdoll.list[24 + 2].parentIndex
CRagdoll!0x0440 m_ragdoll.list[24 + 3].originParentSpace
CRagdoll!0x0450 m_ragdoll.list[24 + 3].pObject
CRagdoll!0x0458 m_ragdoll.list[24 + 3].pConstraint
CRagdoll!0x0460 m_ragdoll.list[24 + 3].parentIndex
CRagdoll!0x0468 m_ragdoll.list[24 + 4].originParentSpace
CRagdoll!0x0478 m_ragdoll.list[24 + 4].pObject
CRagdoll!0x0480 m_ragdoll.list[24 + 4].pConstraint
CRagdoll!0x0488 m_ragdoll.list[24 + 4].parentIndex
CRagdoll!0x0490 m_ragdoll.list[24 + 5].originParentSpace
CRagdoll!0x04a0 m_ragdoll.list[24 + 5].pObject
CRagdoll!0x04a8 m_ragdoll.list[24 + 5].pConstraint
CRagdoll!0x04b0 m_ragdoll.list[24 + 5].parentIndex
CRagdoll!0x04b8 m_ragdoll.list[24 + 6].originParentSpace
CRagdoll!0x04c8 m_ragdoll.list[24 + 6].pObject
CRagdoll!0x04d0 m_ragdoll.list[24 + 6].pConstraint
CRagdoll!0x04d8 m_ragdoll.list[24 + 6].parentIndex
CRagdoll!0x04e0 m_ragdoll.list[24 + 7].originParentSpace
CRagdoll!0x04f0 m_ragdoll.list[24 + 7].pObject
CRagdoll!0x04f8 m_ragdoll.list[24 + 7].pConstraint
CRagdoll!0x0500 m_ragdoll.list[24 + 7].parentIndex
CRagdoll!0x0508 m_ragdoll.boneIndex
```
</details>
<details>
<summary><code>class CTurret extends C_BaseAnimatingOverlay</code></summary>

```
{
	m_aimAngle: Float,
	m_minConeAngle: Float,
	m_maxConeAngle: Float,
}
```

### Offsets

```
CTurret!0x19bc m_aimAngle
CTurret!0x19c8 m_minConeAngle
CTurret!0x19d4 m_maxConeAngle
```
</details>
<details>
<summary><code>class CWeaponX extends C_BaseAnimating</code></summary>

```
{
	m_localOrigin: Vector,
	m_nNextThinkTick: Int,
	m_SequenceTransitioner: C_SequenceTransitioner,
	m_weaponOwner: EHANDLE,
	m_lastPrimaryAttack: Time,
	m_nextReadyTime: Time,
	m_nextPrimaryAttackTime: Time,
	m_attackTimeThisFrame: Time,
	m_worldModelIndexOverride: Int,
	m_iWorldModelIndex: Int,
	m_holsterModelIndex: Int,
	m_droppedModelIndex: Int,
	m_nIdealSequence: Int,
	m_IdealActivity: Int,
	m_weaponActivity: Int,
	m_ActiveState: Int,
	m_ammoInClip: Int,
	m_ammoInStockpile: Int,
	m_lifetimeShots: Int,
	m_flTimeWeaponIdle: Time,
	m_weapState: Int,
	m_discarded: Bool,
	m_bInReload: Bool,
	m_tossRelease: Int,
	m_customActivity: Int,
	m_customActivitySequence: Int,
	m_customActivityOwner: EHANDLE,
	m_customActivityEndTime: Time,
	m_customActivityFlags: Char,
	m_playerData: WeaponPlayerData,
	m_targetingLaserEnabledScript: Bool,
	m_needsReloadCheck: Bool,
	m_needsEmptyCycleCheck: Bool,
	m_skinOverride: Int,
	m_skinOverrideIsValid: Bool,
	m_chargeStartTime: Time,
	m_chargeEndTime: Time,
	m_lastChargeFrac: Float,
	m_lastRegenTime: Time,
	m_cooldownEndTime: Time,
	m_stockPileWasDraining: Bool,
	m_weaponIsCharging: Bool,
	m_weaponChargeLevelIncreasedAnimPlaying: Bool,
	m_lastChargeLevel: Int,
	m_chargeEnergyDepleteStepCounter: Int,
	m_burstFireCount: Int,
	m_burstFireIndex: Int,
	m_shotIndexForSpread: Int,
	m_shotCount: Int,
	m_sustainedDischargeEndTime: Time,
	m_sustainedLaserCurrentSpread: Float,
	m_sustainedDischargeIsInPrimaryAttack: Bool,
	m_sustainedLaserNextRandomSeed: Char,
	m_modBitfieldFromPlayer: Int,
	m_modBitfieldInternal: Int,
	m_modBitfieldCurrent: Int,
	m_curSharedEnergyCost: Int,
	m_grappleWeaponNeedsDryfire: Bool,
	m_scriptActivated: Bool,
	m_flNextEmptySoundTime: Float,
	m_bRemoveable: Bool,
}
```

### Offsets

```
CWeaponX!0x0158 m_localOrigin
CWeaponX!0x054c m_nNextThinkTick
CWeaponX!0x0c00 m_SequenceTransitioner
CWeaponX!0x1600 m_weaponOwner
CWeaponX!0x1604 m_lastPrimaryAttack
CWeaponX!0x1608 m_nextReadyTime
CWeaponX!0x160c m_nextPrimaryAttackTime
CWeaponX!0x1610 m_attackTimeThisFrame
CWeaponX!0x1614 m_worldModelIndexOverride
CWeaponX!0x1618 m_iWorldModelIndex
CWeaponX!0x161c m_holsterModelIndex
CWeaponX!0x1620 m_droppedModelIndex
CWeaponX!0x1624 m_nIdealSequence
CWeaponX!0x1628 m_IdealActivity
CWeaponX!0x162c m_weaponActivity
CWeaponX!0x1630 m_ActiveState
CWeaponX!0x1634 m_ammoInClip
CWeaponX!0x1638 m_ammoInStockpile
CWeaponX!0x163c m_lifetimeShots
CWeaponX!0x1640 m_flTimeWeaponIdle
CWeaponX!0x1644 m_weapState
CWeaponX!0x1649 m_discarded
CWeaponX!0x164a m_bInReload
CWeaponX!0x1650 m_tossRelease
CWeaponX!0x1654 m_customActivity
CWeaponX!0x1658 m_customActivitySequence
CWeaponX!0x165c m_customActivityOwner
CWeaponX!0x1660 m_customActivityEndTime
CWeaponX!0x1664 m_customActivityFlags
CWeaponX!0x1668 m_playerData
CWeaponX!0x174c m_targetingLaserEnabledScript
CWeaponX!0x174d m_needsReloadCheck
CWeaponX!0x174e m_needsEmptyCycleCheck
CWeaponX!0x1750 m_skinOverride
CWeaponX!0x1754 m_skinOverrideIsValid
CWeaponX!0x1758 m_chargeStartTime
CWeaponX!0x175c m_chargeEndTime
CWeaponX!0x1760 m_lastChargeFrac
CWeaponX!0x1764 m_lastRegenTime
CWeaponX!0x1768 m_cooldownEndTime
CWeaponX!0x176c m_stockPileWasDraining
CWeaponX!0x176d m_weaponIsCharging
CWeaponX!0x176e m_weaponChargeLevelIncreasedAnimPlaying
CWeaponX!0x1770 m_lastChargeLevel
CWeaponX!0x1774 m_chargeEnergyDepleteStepCounter
CWeaponX!0x1778 m_burstFireCount
CWeaponX!0x177c m_burstFireIndex
CWeaponX!0x1780 m_shotIndexForSpread
CWeaponX!0x1784 m_shotCount
CWeaponX!0x1788 m_sustainedDischargeEndTime
CWeaponX!0x178c m_sustainedLaserCurrentSpread
CWeaponX!0x1790 m_sustainedDischargeIsInPrimaryAttack
CWeaponX!0x1791 m_sustainedLaserNextRandomSeed
CWeaponX!0x1794 m_modBitfieldFromPlayer
CWeaponX!0x1798 m_modBitfieldInternal
CWeaponX!0x179c m_modBitfieldCurrent
CWeaponX!0x17a0 m_curSharedEnergyCost
CWeaponX!0x17a4 m_grappleWeaponNeedsDryfire
CWeaponX!0x17a5 m_scriptActivated
CWeaponX!0x2c08 m_flNextEmptySoundTime
CWeaponX!0x2c2e m_bRemoveable
```
</details>
<details>
<summary><code>class C_BaseAnimating extends C_BaseEntity</code></summary>

```
{
	m_currentFrame.animCycle: Float,
	m_animNetworkFlags: Int,
	m_networkAnimActive: Bool,
	m_animActive: Bool,
	m_animCollisionEnabled: Bool,
	m_animPlantingEnabled: Bool,
	m_predictedAnimEventData: PredictedAnimEventData,
	m_SequenceTransitioner: C_SequenceTransitioner,
	m_nSkin: Int,
	m_skinMod: Short,
	m_nBody: Int,
	m_nResetEventsParity: Int,
	m_bSequenceFinished: Bool,
	m_bSequenceLooped: Bool,
	m_bSequenceLoops: Bool,
	m_flModelScale: Float,
	m_currentFrameBaseAnimating.animStartTime: Float,
	m_currentFrameBaseAnimating.animStartCycle: Float,
	m_currentFrameBaseAnimating.animPlaybackRate: Float,
	m_currentFrameBaseAnimating.animModelIndex: Int,
	m_currentFrameBaseAnimating.animSequence: Int,
	m_currentFrameBaseAnimating.animSequenceParity: Int,
	m_currentFrameBaseAnimating.m_flPoseParameters: Float,
}
```

### Offsets

```
C_BaseAnimating!0x00c4 m_currentFrame.animCycle
C_BaseAnimating!0x0a68 m_animNetworkFlags
C_BaseAnimating!0x0a6c m_networkAnimActive
C_BaseAnimating!0x0a6e m_animActive
C_BaseAnimating!0x0a6f m_animCollisionEnabled
C_BaseAnimating!0x0a70 m_animPlantingEnabled
C_BaseAnimating!0x0b68 m_predictedAnimEventData
C_BaseAnimating!0x0c00 m_SequenceTransitioner
C_BaseAnimating!0x0e88 m_nSkin
C_BaseAnimating!0x0e8c m_skinMod
C_BaseAnimating!0x0e90 m_nBody
C_BaseAnimating!0x0e9c m_nResetEventsParity
C_BaseAnimating!0x0f34 m_bSequenceFinished
C_BaseAnimating!0x0f3c m_bSequenceLooped
C_BaseAnimating!0x0f3d m_bSequenceLoops
C_BaseAnimating!0x0f40 m_flModelScale
C_BaseAnimating!0x0f4c m_currentFrameBaseAnimating.animStartTime
C_BaseAnimating!0x0f50 m_currentFrameBaseAnimating.animStartCycle
C_BaseAnimating!0x0f54 m_currentFrameBaseAnimating.animPlaybackRate
C_BaseAnimating!0x0f5c m_currentFrameBaseAnimating.animModelIndex
C_BaseAnimating!0x0f60 m_currentFrameBaseAnimating.animSequence
C_BaseAnimating!0x0f64 m_currentFrameBaseAnimating.animSequenceParity
C_BaseAnimating!0x0f68 m_currentFrameBaseAnimating.m_flPoseParameters
```
</details>
<details>
<summary><code>class C_BaseAnimatingOverlay extends C_BaseAnimating</code></summary>

```
{
	m_AnimOverlay: C_AnimationLayer,
	m_AnimOverlayCount: Int,
	m_currentFrameAnimatingOverlay.animOverlayIsActive: Bool,
	m_currentFrameAnimatingOverlay.animOverlayStartTime: Float,
	m_currentFrameAnimatingOverlay.animOverlayStartCycle: Float,
	m_currentFrameAnimatingOverlay.animOverlayPlaybackRate: Float,
	m_currentFrameAnimatingOverlay.animOverlayModelIndex: Int,
	m_currentFrameAnimatingOverlay.animOverlaySequence: Int,
	m_currentFrameAnimatingOverlay.animOverlayWeight: Float,
	m_currentFrameAnimatingOverlay.animOverlayOrder: Int,
	m_currentFrameAnimatingOverlay.animOverlayAnimTime: Float,
	m_currentFrameAnimatingOverlay.animOverlayFadeInDuration: Float,
	m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration: Float,
	m_currentFrameAnimatingOverlay.animOverlayCycle: Float,
}
```

### Offsets

```
C_BaseAnimatingOverlay!0x1608 m_AnimOverlay
C_BaseAnimatingOverlay!0x16e0 m_AnimOverlayCount
C_BaseAnimatingOverlay!0x1724 m_currentFrameAnimatingOverlay.animOverlayIsActive
C_BaseAnimatingOverlay!0x1730 m_currentFrameAnimatingOverlay.animOverlayStartTime
C_BaseAnimatingOverlay!0x1754 m_currentFrameAnimatingOverlay.animOverlayStartCycle
C_BaseAnimatingOverlay!0x1778 m_currentFrameAnimatingOverlay.animOverlayPlaybackRate
C_BaseAnimatingOverlay!0x179c m_currentFrameAnimatingOverlay.animOverlayModelIndex
C_BaseAnimatingOverlay!0x17c0 m_currentFrameAnimatingOverlay.animOverlaySequence
C_BaseAnimatingOverlay!0x17e4 m_currentFrameAnimatingOverlay.animOverlayWeight
C_BaseAnimatingOverlay!0x1808 m_currentFrameAnimatingOverlay.animOverlayOrder
C_BaseAnimatingOverlay!0x182c m_currentFrameAnimatingOverlay.animOverlayAnimTime
C_BaseAnimatingOverlay!0x1850 m_currentFrameAnimatingOverlay.animOverlayFadeInDuration
C_BaseAnimatingOverlay!0x1874 m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration
C_BaseAnimatingOverlay!0x1898 m_currentFrameAnimatingOverlay.animOverlayCycle
```
</details>
<details>
<summary><code>class C_BaseCombatCharacter extends C_BaseAnimatingOverlay</code></summary>

```
{
	m_currentFrame.weaponGettingSwitchedOut: EHANDLE,
	m_currentFrame.showActiveWeapon3p: Bool,
	m_deathVelocity: Float,
	m_phaseShiftFlags: Int,
	m_flNextAttack: Time,
	m_lastFiredTime: Time,
	m_lastFiredWeapon: EHANDLE,
	m_raiseFromMeleeEndTime: Time,
	m_sharedEnergyCount: Int,
	m_sharedEnergyTotal: Int,
	m_sharedEnergyLockoutThreshold: Int,
	m_lastSharedEnergyRegenTime: Time,
	m_sharedEnergyRegenRate: Time,
	m_sharedEnergyRegenDelay: Float,
	m_lastSharedEnergyTakeTime: Time,
	m_inventory: WeaponInventory_Client,
	m_selectedWeapons: Char,
	m_latestPrimaryWeapons: EHANDLE,
	m_latestPrimaryWeaponsIndexZeroOrOne: EHANDLE,
	m_latestNonOffhandWeapons: Char,
	m_selectedOffhands: Char,
	m_selectedOffhandsPendingHybridAction: Char,
	m_lastCycleSlot: Char,
	m_latestMeleeWeapon: EHANDLE,
	m_weaponPermission: Int,
	m_weaponDelayEnableTime: Time,
	m_weaponDisabledInScript: Bool,
	m_weaponDisabledFlags: Char,
	m_weaponTypeDisabledFlags: Int,
	m_weaponTypeDisabledRefCount: Char,
	m_hudInfo_visibilityTestAlwaysPasses: Bool,
	m_contextAction: Int,
	m_phaseShiftTimeStart: Time,
	m_phaseShiftTimeEnd: Time,
}
```

### Offsets

```
C_BaseCombatCharacter!0x00c8 m_currentFrame.weaponGettingSwitchedOut
C_BaseCombatCharacter!0x00d0 m_currentFrame.showActiveWeapon3p
C_BaseCombatCharacter!0x0454 m_deathVelocity
C_BaseCombatCharacter!0x0790 m_phaseShiftFlags
C_BaseCombatCharacter!0x1980 m_flNextAttack
C_BaseCombatCharacter!0x1984 m_lastFiredTime
C_BaseCombatCharacter!0x1988 m_lastFiredWeapon
C_BaseCombatCharacter!0x198c m_raiseFromMeleeEndTime
C_BaseCombatCharacter!0x1990 m_sharedEnergyCount
C_BaseCombatCharacter!0x1994 m_sharedEnergyTotal
C_BaseCombatCharacter!0x1998 m_sharedEnergyLockoutThreshold
C_BaseCombatCharacter!0x199c m_lastSharedEnergyRegenTime
C_BaseCombatCharacter!0x19a0 m_sharedEnergyRegenRate
C_BaseCombatCharacter!0x19a4 m_sharedEnergyRegenDelay
C_BaseCombatCharacter!0x19a8 m_lastSharedEnergyTakeTime
C_BaseCombatCharacter!0x19b0 m_inventory
C_BaseCombatCharacter!0x1a08 m_selectedWeapons
C_BaseCombatCharacter!0x1a0c m_latestPrimaryWeapons
C_BaseCombatCharacter!0x1a14 m_latestPrimaryWeaponsIndexZeroOrOne
C_BaseCombatCharacter!0x1a1c m_latestNonOffhandWeapons
C_BaseCombatCharacter!0x1a1e m_selectedOffhands
C_BaseCombatCharacter!0x1a21 m_selectedOffhandsPendingHybridAction
C_BaseCombatCharacter!0x1a24 m_lastCycleSlot
C_BaseCombatCharacter!0x1a28 m_latestMeleeWeapon
C_BaseCombatCharacter!0x1a2c m_weaponPermission
C_BaseCombatCharacter!0x1a30 m_weaponDelayEnableTime
C_BaseCombatCharacter!0x1a34 m_weaponDisabledInScript
C_BaseCombatCharacter!0x1a59 m_weaponDisabledFlags
C_BaseCombatCharacter!0x1a5c m_weaponTypeDisabledFlags
C_BaseCombatCharacter!0x1a60 m_weaponTypeDisabledRefCount
C_BaseCombatCharacter!0x1a69 m_hudInfo_visibilityTestAlwaysPasses
C_BaseCombatCharacter!0x1a7c m_contextAction
C_BaseCombatCharacter!0x1aa8 m_phaseShiftTimeStart
C_BaseCombatCharacter!0x1aac m_phaseShiftTimeEnd
```
</details>
<details>
<summary><code>class C_BaseEntity</code></summary>

```
{
	m_ModelName: String,
	m_fFlags: Int,
	m_angAbsRotation: Vector,
	m_vecAbsOrigin: PositionVector,
	m_vecPrevAbsOrigin: PositionVector,
	m_flGravity: Float,
	m_rgflCoordinateFrame: Float,
}
```

### Offsets

```
C_BaseEntity!0x0030 m_ModelName
C_BaseEntity!0x0098 m_fFlags
C_BaseEntity!0x0134 m_angAbsRotation
C_BaseEntity!0x014c m_vecAbsOrigin
C_BaseEntity!0x03fc m_vecPrevAbsOrigin
C_BaseEntity!0x0408 m_flGravity
C_BaseEntity!0x08b0 m_rgflCoordinateFrame
```
</details>
<details>
<summary><code>class C_BaseEntity</code></summary>

```
{
	m_iEFlags: Int,
	m_fFlags: Int,
	m_currentFrame.modelIndex: Short,
	m_currentFrame.viewOffset: Vector,
	m_vecAngVelocity: Vector,
	m_angAbsRotation: Vector,
	m_vecAbsVelocity: Vector,
	m_vecAbsOrigin: Vector,
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_flGravity: Float,
	m_flProxyRandomValue: Float,
	m_hGroundEntity: EHANDLE,
	m_flMaxspeed: Float,
	m_visibilityFlags: Int,
	m_fEffects: Int,
	m_iTeamNum: Int,
	m_passThroughFlags: Int,
	m_passThroughThickness: Int,
	m_passThroughDirection: Float,
	m_deathVelocity: Vector,
	m_vecVelocity: Vector,
	m_angNetworkAngles: Vector,
	m_flFriction: Float,
	m_hOwnerEntity: EHANDLE,
	m_bRenderWithViewModels: Bool,
	m_nRenderFX: Char,
	m_nRenderMode: Char,
	m_MoveType: Char,
	m_MoveCollide: Char,
	m_Collision: CCollisionProperty,
}
```

### Offsets

```
C_BaseEntity!0x0058 m_iEFlags
C_BaseEntity!0x0098 m_fFlags
C_BaseEntity!0x00a8 m_currentFrame.modelIndex
C_BaseEntity!0x00b8 m_currentFrame.viewOffset
C_BaseEntity!0x0128 m_vecAngVelocity
C_BaseEntity!0x0134 m_angAbsRotation
C_BaseEntity!0x0140 m_vecAbsVelocity
C_BaseEntity!0x014c m_vecAbsOrigin
C_BaseEntity!0x0158 m_localOrigin
C_BaseEntity!0x0164 m_localAngles
C_BaseEntity!0x0408 m_flGravity
C_BaseEntity!0x040c m_flProxyRandomValue
C_BaseEntity!0x041c m_hGroundEntity
C_BaseEntity!0x0424 m_flMaxspeed
C_BaseEntity!0x0428 m_visibilityFlags
C_BaseEntity!0x042c m_fEffects
C_BaseEntity!0x0430 m_iTeamNum
C_BaseEntity!0x0448 m_passThroughFlags
C_BaseEntity!0x044c m_passThroughThickness
C_BaseEntity!0x0450 m_passThroughDirection
C_BaseEntity!0x0454 m_deathVelocity
C_BaseEntity!0x0460 m_vecVelocity
C_BaseEntity!0x046c m_angNetworkAngles
C_BaseEntity!0x0478 m_flFriction
C_BaseEntity!0x0480 m_hOwnerEntity
C_BaseEntity!0x0484 m_bRenderWithViewModels
C_BaseEntity!0x0485 m_nRenderFX
C_BaseEntity!0x0491 m_nRenderMode
C_BaseEntity!0x0492 m_MoveType
C_BaseEntity!0x0493 m_MoveCollide
C_BaseEntity!0x0498 m_Collision
```
</details>
<details>
<summary><code>class C_BreakableSurface extends C_BaseEntity</code></summary>

```
{
	m_nPanelBits: Char,
}
```

### Offsets

```
C_BreakableSurface!0x0cc8 m_nPanelBits
```
</details>
<details>
<summary><code>class C_ClientRagdoll extends C_BaseEntity</code></summary>

```
{
	m_clrRender: Color32,
	m_nRenderFX: Char,
	m_nRenderMode: Char,
	m_pRagdoll: CRagdoll,
	m_nSkin: Int,
	m_skinMod: Short,
	m_nBody: Int,
	m_bFadeOut: Bool,
	m_bImportant: Bool,
	m_flEffectTime: Time,
	m_iCurrentFriction: Int,
	m_iMinFriction: Int,
	m_iMaxFriction: Int,
	m_flFrictionModTime: Float,
	m_flFrictionTime: Time,
	m_iFrictionAnimState: Int,
	m_bReleaseRagdoll: Bool,
	m_bFadingOut: Bool,
	m_flScaleEnd: Float,
	m_flScaleTimeStart: Float,
	m_flScaleTimeEnd: Float,
}
```

### Offsets

```
C_ClientRagdoll!0x0050 m_clrRender
C_ClientRagdoll!0x0485 m_nRenderFX
C_ClientRagdoll!0x0491 m_nRenderMode
C_ClientRagdoll!0x0bc8 m_pRagdoll
C_ClientRagdoll!0x0e88 m_nSkin
C_ClientRagdoll!0x0e8c m_skinMod
C_ClientRagdoll!0x0e90 m_nBody
C_ClientRagdoll!0x1600 m_bFadeOut
C_ClientRagdoll!0x1601 m_bImportant
C_ClientRagdoll!0x1604 m_flEffectTime
C_ClientRagdoll!0x1608 m_iCurrentFriction
C_ClientRagdoll!0x160c m_iMinFriction
C_ClientRagdoll!0x1610 m_iMaxFriction
C_ClientRagdoll!0x1614 m_flFrictionModTime
C_ClientRagdoll!0x1618 m_flFrictionTime
C_ClientRagdoll!0x161c m_iFrictionAnimState
C_ClientRagdoll!0x1620 m_bReleaseRagdoll
C_ClientRagdoll!0x1621 m_bFadingOut
C_ClientRagdoll!0x1624 m_flScaleEnd
C_ClientRagdoll!0x164c m_flScaleTimeStart
C_ClientRagdoll!0x1674 m_flScaleTimeEnd
```
</details>
<details>
<summary><code>class C_CrossbowBolt extends C_Projectile</code></summary>

```
{
	m_bounceCount: Int,
	m_maxBounceCount: Int,
	m_doesGrow: Bool,
	m_growStartSize: Float,
	m_growStage1Tick: Tick,
	m_growStage1Size: Float,
	m_growStage2Tick: Tick,
	m_growStage2Size: Float,
	m_growStageFinalTick: Tick,
	m_growStageFinalSize: Float,
}
```

### Offsets

```
C_CrossbowBolt!0x2c40 m_bounceCount
C_CrossbowBolt!0x2c44 m_maxBounceCount
C_CrossbowBolt!0x2c48 m_doesGrow
C_CrossbowBolt!0x2c4c m_growStartSize
C_CrossbowBolt!0x2c50 m_growStage1Tick
C_CrossbowBolt!0x2c54 m_growStage1Size
C_CrossbowBolt!0x2c58 m_growStage2Tick
C_CrossbowBolt!0x2c5c m_growStage2Size
C_CrossbowBolt!0x2c60 m_growStageFinalTick
C_CrossbowBolt!0x2c64 m_growStageFinalSize
```
</details>
<details>
<summary><code>class C_DynamicProp extends C_BaseEntity</code></summary>

```
{
	m_bClientSide: Bool,
}
```

### Offsets

```
C_DynamicProp!0x1600 m_bClientSide
```
</details>
<details>
<summary><code>class C_EnvWindShared</code></summary>

```
{
	m_flStartTime: Float,
	m_iWindSeed: Int,
	m_iMinWind: Int,
	m_iMaxWind: Int,
	m_windRadius: Int,
	m_iMinGust: Int,
	m_iMaxGust: Int,
	m_flMinGustDelay: Float,
	m_flMaxGustDelay: Float,
	m_flGustDuration: Float,
	m_iGustDirChange: Int,
	m_location: Vector,
	m_iszGustSound: Int,
	m_iWindDir: Int,
	m_flWindSpeed: Float,
	m_currentWindVector: Vector,
	m_CurrentSwayVector: Vector,
	m_PrevSwayVector: Vector,
	m_iInitialWindDir: Int,
	m_flInitialWindSpeed: Float,
	m_flVariationTime: Float,
	m_flSimTime: Float,
	m_flSwitchTime: Float,
	m_flAveWindSpeed: Float,
	m_bGusting: Bool,
	m_flWindAngleVariation: Float,
	m_flWindSpeedVariation: Float,
	m_iEntIndex: Int,
	m_Stream: Void,
	m_WindVariationStream: Void,
	m_WindAveQueue: Void,
	m_WindVariationQueue: Void,
}
```

### Offsets

```
C_EnvWindShared!0x0008 m_flStartTime
C_EnvWindShared!0x000c m_iWindSeed
C_EnvWindShared!0x0010 m_iMinWind
C_EnvWindShared!0x0014 m_iMaxWind
C_EnvWindShared!0x0018 m_windRadius
C_EnvWindShared!0x001c m_iMinGust
C_EnvWindShared!0x0020 m_iMaxGust
C_EnvWindShared!0x0024 m_flMinGustDelay
C_EnvWindShared!0x0028 m_flMaxGustDelay
C_EnvWindShared!0x002c m_flGustDuration
C_EnvWindShared!0x0030 m_iGustDirChange
C_EnvWindShared!0x0034 m_location
C_EnvWindShared!0x0040 m_iszGustSound
C_EnvWindShared!0x0044 m_iWindDir
C_EnvWindShared!0x0048 m_flWindSpeed
C_EnvWindShared!0x004c m_currentWindVector
C_EnvWindShared!0x0058 m_CurrentSwayVector
C_EnvWindShared!0x0064 m_PrevSwayVector
C_EnvWindShared!0x0070 m_iInitialWindDir
C_EnvWindShared!0x0074 m_flInitialWindSpeed
C_EnvWindShared!0x0078 m_flVariationTime
C_EnvWindShared!0x007c m_flSimTime
C_EnvWindShared!0x0080 m_flSwitchTime
C_EnvWindShared!0x0084 m_flAveWindSpeed
C_EnvWindShared!0x0088 m_bGusting
C_EnvWindShared!0x008c m_flWindAngleVariation
C_EnvWindShared!0x0090 m_flWindSpeedVariation
C_EnvWindShared!0x0094 m_iEntIndex
C_EnvWindShared!0x0098 m_Stream
C_EnvWindShared!0x00d0 m_WindVariationStream
C_EnvWindShared!0x0108 m_WindAveQueue
C_EnvWindShared!0x0140 m_WindVariationQueue
```
</details>
<details>
<summary><code>class C_ExposureController extends C_BaseEntity</code></summary>

```
{
	m_autoCompensation: Float,
	m_autoForceValue: Float,
	m_autoMin: Float,
	m_autoMax: Float,
	m_autoSpeed: Float,
	m_minFadeTime: Float,
	m_isDirectional: Bool,
	m_directionalHalfAngle: Float,
}
```

### Offsets

```
C_ExposureController!0x0a40 m_autoCompensation
C_ExposureController!0x0a44 m_autoForceValue
C_ExposureController!0x0a48 m_autoMin
C_ExposureController!0x0a4c m_autoMax
C_ExposureController!0x0a50 m_autoSpeed
C_ExposureController!0x0a54 m_minFadeTime
C_ExposureController!0x0a58 m_isDirectional
C_ExposureController!0x0a5c m_directionalHalfAngle
```
</details>
<details>
<summary><code>class C_ExposureVolume extends C_BaseEntity</code></summary>

```
{
	m_volumeTester: Outer,
	m_exposureTarget: ClassPtr,
	m_exposureTargetName: String,
	m_exposurePriority: Float,
}
```

### Offsets

```
C_ExposureVolume!0x0a40 m_volumeTester
C_ExposureVolume!0x0a48 m_exposureTarget
C_ExposureVolume!0x0a50 m_exposureTargetName
C_ExposureVolume!0x0a58 m_exposurePriority
```
</details>
<details>
<summary><code>class C_FogController extends C_BaseEntity</code></summary>

```
{
	m_fogParams: fogplayerparamsstate_t,
	m_fogAngles: Vector,
	m_useAbsAngles: Bool,
}
```

### Offsets

```
C_FogController!0x0a40 m_fogParams
C_FogController!0x0aa8 m_fogAngles
C_FogController!0x0ab4 m_useAbsAngles
```
</details>
<details>
<summary><code>class C_FogVolume extends C_BaseEntity</code></summary>

```
{
	m_volumeTester: Outer,
	m_fogTarget: ClassPtr,
	m_fogTargetName: String,
	m_fogPriority: Int,
}
```

### Offsets

```
C_FogVolume!0x0a40 m_volumeTester
C_FogVolume!0x0a48 m_fogTarget
C_FogVolume!0x0a50 m_fogTargetName
C_FogVolume!0x0a58 m_fogPriority
```
</details>
<details>
<summary><code>class C_GlobalNonRewinding extends C_BaseEntity</code></summary>

```
{
	m_playerObserver: C_ObserverMode,
	m_playerMiscData: C_NonRewindMiscData,
}
```

### Offsets

```
C_GlobalNonRewinding!0x0a40 m_playerObserver
C_GlobalNonRewinding!0x0e40 m_playerMiscData
```
</details>
<details>
<summary><code>class C_KnockBack</code></summary>

```
{
	velocity: Vector,
	beginTime: Time,
	endTime: Time,
}
```

### Offsets

```
C_KnockBack!0x0008 velocity
C_KnockBack!0x0014 beginTime
C_KnockBack!0x0018 endTime
```
</details>
<details>
<summary><code>class C_Missile extends C_Projectile</code></summary>

```
{
	m_hasPlayedWhizby: Bool,
	m_whizByStart: Vector,
	m_whizBySoundName: Char,
	m_homingSpeed: Float,
	m_homingSpeedDodgingPlayer: Float,
	m_launchDir: Vector,
	m_hSpecificTarget: EHANDLE,
	m_targetOffset: Vector,
	m_targetPosition: Vector,
	m_useTargetPosition: Bool,
	m_postIgnitionSpeed: Float,
	m_flGracePeriodEndsAt: Time,
	m_pathSettingsInitialized: Bool,
	m_expandContractMissile: Bool,
	m_spiralMissile: Bool,
	m_spiralSettings: Void,
	m_expandContractSettings: MissilePathExpandContractSettings_Client,
	m_lastThinkTime: Time,
	m_explosionIgnoreEntity: EHANDLE,
}
```

### Offsets

```
C_Missile!0x2c40 m_hasPlayedWhizby
C_Missile!0x2c44 m_whizByStart
C_Missile!0x2c50 m_whizBySoundName
C_Missile!0x2c90 m_homingSpeed
C_Missile!0x2c94 m_homingSpeedDodgingPlayer
C_Missile!0x2c98 m_launchDir
C_Missile!0x2ca4 m_hSpecificTarget
C_Missile!0x2ca8 m_targetOffset
C_Missile!0x2cb4 m_targetPosition
C_Missile!0x2cc0 m_useTargetPosition
C_Missile!0x2cc4 m_postIgnitionSpeed
C_Missile!0x2cc8 m_flGracePeriodEndsAt
C_Missile!0x2ccc m_pathSettingsInitialized
C_Missile!0x2ccd m_expandContractMissile
C_Missile!0x2cce m_spiralMissile
C_Missile!0x2d08 m_spiralSettings
C_Missile!0x2d38 m_expandContractSettings
C_Missile!0x2d90 m_lastThinkTime
C_Missile!0x2d94 m_explosionIgnoreEntity
```
</details>
<details>
<summary><code>class C_NPC_SentryTurret extends C_BaseEntity</code></summary>

```
{
	m_killCount: Int,
	m_titanKillCount: Int,
}
```

### Offsets

```
C_NPC_SentryTurret!0x1d44 m_killCount
C_NPC_SentryTurret!0x1d48 m_titanKillCount
```
</details>
<details>
<summary><code>class C_NonRewindMiscData</code></summary>

```
{
	m_nextRespawnTime: Float,
	m_musicPackAssigned: Int,
}
```

### Offsets

```
C_NonRewindMiscData!0x0000 m_nextRespawnTime
C_NonRewindMiscData!0x0004 m_musicPackAssigned
```
</details>
<details>
<summary><code>class C_ObserverMode</code></summary>

```
{
	m_observerMode: Int,
	m_observerTarget: EHANDLE,
}
```

### Offsets

```
C_ObserverMode!0x0000 m_observerMode
C_ObserverMode!0x0004 m_observerTarget
```
</details>
<details>
<summary><code>class C_ParticleSystem extends C_BaseEntity</code></summary>

```
{
	m_bClientSide: Bool,
	m_bActive: Bool,
	m_warmUpTime: Float,
	m_pauseAfterWarmup: Bool,
	m_bInSkybox: Bool,
	m_killForReplay: Bool,
	m_killIfOverLimit: Bool,
}
```

### Offsets

```
C_ParticleSystem!0x0a48 m_bClientSide
C_ParticleSystem!0x0a49 m_bActive
C_ParticleSystem!0x0a50 m_warmUpTime
C_ParticleSystem!0x0a54 m_pauseAfterWarmup
C_ParticleSystem!0x0a55 m_bInSkybox
C_ParticleSystem!0x0a56 m_killForReplay
C_ParticleSystem!0x0a57 m_killIfOverLimit
```
</details>
<details>
<summary><code>class C_Player extends C_BaseCombatCharacter</code></summary>

```
{
	m_fFlags: Int,
	m_pMoveParent: EHANDLE,
	m_vecAbsVelocity: Vector,
	m_hGroundEntity: EHANDLE,
	m_flMaxspeed: Int,
	m_vecVelocity: Vector,
	m_flFriction: Float,
	m_nNextThinkTick: Int,
	m_SequenceTransitioner: C_SequenceTransitioner,
	m_bZooming: Bool,
	m_zoomToggleOnStartTime: Time,
	m_zoomBaseFrac: Float,
	m_zoomBaseTime: Time,
	m_zoomFullStartTime: Time,
	m_lastUCmdSimulationTicks: Int,
	m_lastUCmdSimulationRemainderTime: Float,
	m_Local: C_PlayerLocalData,
	m_currentFramePlayer.timeBase: Float,
	m_currentFramePlayer.statusEffectsTimedPlayerCUR: StatusEffectTimedData,
	m_currentFramePlayer.statusEffectsEndlessPlayerCUR: StatusEffectEndlessData,
	m_currentFramePlayer.m_flHullHeight: Float,
	m_currentFramePlayer.m_traversalAnimProgress: Float,
	m_currentFramePlayer.m_sprintTiltFrac: Float,
	m_currentFramePlayer.m_ammoPoolCount: Int,
	m_currentFrameLocalPlayer.m_stepSmoothingOffset: Vector,
	m_currentFrameLocalPlayer.m_duckTransitionRemainderMsec: Int,
	m_currentFrameLocalPlayer.m_vecPunchBase_Angle: Vector,
	m_currentFrameLocalPlayer.m_vecPunchBase_AngleVel: Vector,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle: Vector,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.x: Float,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.y: Float,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.z: Float,
	m_currentFrameLocalPlayer.m_localGravityRotation: Quaternion,
	pl: CPlayerState,
	m_ammoPoolCapacity: Int,
	m_classModsActive: 29,
	m_gestureSequences: Int,
	m_gestureStartTimes: Time,
	m_gestureBlendInDuration: Float,
	m_gestureBlendOutDuration: Float,
	m_gestureFadeOutStartTime: Time,
	m_gestureFadeOutDuration: Float,
	m_gestureAutoKillBitfield: Int,
	m_afButtonLast: Int,
	m_afButtonPressed: Int,
	m_afButtonReleased: Int,
	m_nButtons: Int,
	m_nImpulse: Int,
	m_flPhysics: Int,
	m_flStepSoundTime: Float,
	m_flTimeAllSuitDevicesOff: Float,
	m_fStickySprintMinTime: Float,
	m_bPlayedSprintStartEffects: Bool,
	m_fIsSprinting: Bool,
	m_fIsWalking: Bool,
	m_lastSprintPressTime: Time,
	m_stickySprintForwardEnableTime: Time,
	m_stickySprintForwardDisableTime: Time,
	m_sprintStartedTime: Time,
	m_sprintStartedFrac: Float,
	m_sprintEndedTime: Time,
	m_sprintEndedFrac: Float,
	m_stickySprintStartTime: Time,
	m_damageImpulseNoDecelEndTime: Time,
	m_duckState: Int,
	m_leanState: Int,
	m_doingHalfDuck: Bool,
	m_canStand: Bool,
	m_StandHullMin: Vector,
	m_StandHullMax: Vector,
	m_DuckHullMin: Vector,
	m_DuckHullMax: Vector,
	m_upDir: Vector,
	m_upDirPredicted: Vector,
	m_lastWallRunStartPos: Vector,
	m_wallRunCount: Int,
	m_wallRunWeak: Bool,
	m_shouldBeOneHanded: Bool,
	m_oneHandFraction: Float,
	m_animAimPitch: Float,
	m_animAimYaw: Float,
	m_wallRunPushAwayTime: Float,
	m_wallrunRetryTime: Time,
	m_wallrunRetryPos: Vector,
	m_wallrunRetryNormal: Vector,
	m_wallHangTime: Float,
	m_traversalState: Int,
	m_traversalType: Int,
	m_traversalBegin: Vector,
	m_traversalMid: Vector,
	m_traversalEnd: Vector,
	m_traversalMidFrac: Float,
	m_traversalForwardDir: Vector,
	m_traversalRefPos: Vector,
	m_traversalProgress: Float,
	m_traversalStartTime: Time,
	m_traversalHandAppearTime: Time,
	m_traversalReleaseTime: Time,
	m_traversalBlendOutStartTime: Time,
	m_traversalBlendOutStartOffset: Vector,
	m_traversalYawDelta: Float,
	m_wallDangleJumpOffTime: Time,
	m_wallDangleMayHangHere: Bool,
	m_wallDangleForceFallOff: Bool,
	m_wallDangleLastPushedForward: Bool,
	m_wallDangleDisableWeapon: Int,
	m_wallDangleClimbProgressFloor: Float,
	m_wallClimbSetUp: Bool,
	m_wallHanging: Bool,
	m_grapple: GrappleData,
	m_grapple: GrappleData,
	m_grappleActive: Bool,
	m_grappleActive: Bool,
	m_grappleNeedWindowCheck: Bool,
	m_grappleNextWindowHint: EHANDLE,
	m_slowMoEnabled: Bool,
	m_sliding: Bool,
	m_slideLongJumpAllowed: Bool,
	m_lastSlideTime: Time,
	m_lastSlideBoost: Float,
	m_gravityGrenadeStatusEffect: Int,
	m_bIsStickySprinting: Bool,
	m_prevMoveYaw: Float,
	m_sprintTiltVel: Float,
	m_turret: EHANDLE,
	m_hViewModels: EHANDLE,
	m_viewOffsetEntity: Player_ViewOffsetEntityData,
	m_activeZipline: EHANDLE,
	m_lastZipline: EHANDLE,
	m_lastZiplineDetachTime: Time,
	m_ziplineValid3pWeaponLayerAnim: Bool,
	m_ziplineState: Int,
	m_zipline: PlayerZiplineData_Client,
	m_ziplineViewOffsetPosition: Vector,
	m_ziplineViewOffsetVelocity: Vector,
	m_ziplineGrenadeEntity: EHANDLE,
	m_ziplineGrenadeBeginStationEntity: EHANDLE,
	m_ziplineGrenadeBeginStationAttachmentIndex: Int,
	m_sameZiplineCooldownTime: Float,
	m_playAnimationType: Int,
	m_detachGrappleOnPlayAnimationEnd: Bool,
	m_playAnimationNext: Int,
	m_boosting: Bool,
	m_activateBoost: Bool,
	m_repeatedBoost: Bool,
	m_boostMeter: Float,
	m_jetpack: Bool,
	m_activateJetpack: Bool,
	m_jetpackAfterburner: Bool,
	m_gliding: Bool,
	m_glideMeter: Float,
	m_glideRechargeDelayAccumulator: Float,
	m_hovering: Bool,
	m_isPerformingBoostAction: Bool,
	m_lastJumpHeight: Float,
	m_touchingUpdraftTriggers: EHANDLE,
	m_touchingUpdraftTriggersCount: Int,
	m_touchingSlipTriggers: EHANDLE,
	m_touchingSlipTriggersCount: Int,
	m_slipAirRestrictDirection: Vector,
	m_slipAirRestrictTime: Time,
	m_melee: PlayerMelee_PlayerData,
	m_useCredit: Bool,
	m_wallRunStartTime: Time,
	m_wallRunClearTime: Time,
	m_onSlopeTime: Float,
	m_lastWallNormal: Vector,
	m_dodging: Bool,
	m_lastDodgeTime: Time,
	m_vecPreviouslyPredictedOrigin: Vector,
	m_flTimeLastTouchedWall: Float,
	m_timeJetpackHeightActivateCheckPassed: Time,
	m_flTimeLastTouchedGround: Float,
	m_flTimeLastJumped: Float,
	m_flTimeLastLanded: Float,
	m_flLastLandFromHeight: Float,
	m_usePressedTime: Float,
	m_lastUseTime: Float,
	m_lastFakeFloorPos: Vector,
	m_bHasJumpedSinceTouchedGround: Bool,
	m_bDoMultiJumpPenalty: Bool,
	m_dodgingInAir: Bool,
	m_activeViewmodelModifiers: Bool,
	m_lastMoveInputTime: Time,
	m_ignoreEntityForMovementUntilNotTouching: EHANDLE,
	m_gameMovementUtil.m_surfaceFriction: Float,
	m_lungeTargetEntity: EHANDLE,
	m_isLungingToPosition: Bool,
	m_lungeTargetPosition: Vector,
	m_lungeStartPositionOffset: Vector,
	m_lungeEndPositionOffset: Vector,
	m_lungeStartTime: Time,
	m_lungeEndTime: Time,
	m_lungeCanFly: Bool,
	m_lungeLockPitch: Bool,
	m_lungeStartPitch: Float,
	m_lungeSmoothTime: Float,
	m_lungeMaxTime: Float,
	m_lungeMaxEndSpeed: Float,
	m_vPrevGroundNormal: Vector,
	m_pushAwayFromTopAcceleration: Vector,
	m_controllerModeActive: Bool,
	m_skydiveForwardPoseValueVelocity: Float,
	m_skydiveForwardPoseValueTarget: Float,
	m_skydiveForwardPoseValueCurrent: Float,
	m_skydiveSidePoseValueVelocity: Float,
	m_skydiveSidePoseValueTarget: Float,
	m_skydiveSidePoseValueCurrent: Float,
	m_skydiveYawVelocity: Float,
	m_skydiveIsNearLeviathan: Bool,
	m_skydiveState: Int,
	m_skydiveStartTime: Time,
	m_skydiveEndTime: Time,
	m_skydiveAnticipateStartTime: Time,
	m_skydiveAnticipateEndTime: Time,
	m_skydiveDistanceToLand: Float,
	m_skydiveDiveAngle: Float,
	m_skydiveIsDiving: Bool,
	m_skydiveSpeed: Float,
	m_skydiveStrafeAngle: Float,
	m_skydiveFreelookEnabled: Bool,
	m_skydiveFreelookLockedAngle: Vector,
	m_skydivePlayerPitch: Float,
	m_skydivePlayerYaw: Float,
	m_skydiveFollowing: Bool,
	m_skydiveUnfollowVelocity: Vector,
	m_skydiveLeviathanHitPosition: Vector,
	m_skydiveLeviathanHitNormal: Vector,
	m_skydiveSlipVelocity: Vector,
	m_playerKnockBacks: C_KnockBack,
	m_updraftCount: Int,
	m_updraftStage: Int,
	m_updraftEnterTime: Time,
	m_updraftLeaveTime: Time,
	m_updraftMinShakeActivationHeight: Float,
	m_updraftMaxShakeActivationHeight: Float,
	m_updraftLiftActivationHeight: Float,
	m_updraftLiftSpeed: Float,
	m_updraftLiftAcceleration: Float,
	m_updraftLiftExitDuration: Float,
	m_updraftSlowTime: Time,
}
```

### Offsets

```
C_Player!0x0098 m_fFlags
C_Player!0x0118 m_pMoveParent
C_Player!0x0140 m_vecAbsVelocity
C_Player!0x041c m_hGroundEntity
C_Player!0x0424 m_flMaxspeed
C_Player!0x0460 m_vecVelocity
C_Player!0x0478 m_flFriction
C_Player!0x054c m_nNextThinkTick
C_Player!0x0c00 m_SequenceTransitioner
C_Player!0x1b81 m_bZooming
C_Player!0x1b84 m_zoomToggleOnStartTime
C_Player!0x1b88 m_zoomBaseFrac
C_Player!0x1b8c m_zoomBaseTime
C_Player!0x1b90 m_zoomFullStartTime
C_Player!0x1c14 m_lastUCmdSimulationTicks
C_Player!0x1c18 m_lastUCmdSimulationRemainderTime
C_Player!0x1d38 m_Local
C_Player!0x2020 m_currentFramePlayer.timeBase
C_Player!0x2028 m_currentFramePlayer.statusEffectsTimedPlayerCUR
C_Player!0x2118 m_currentFramePlayer.statusEffectsEndlessPlayerCUR
C_Player!0x21b8 m_currentFramePlayer.m_flHullHeight
C_Player!0x21bc m_currentFramePlayer.m_traversalAnimProgress
C_Player!0x21c0 m_currentFramePlayer.m_sprintTiltFrac
C_Player!0x21d0 m_currentFramePlayer.m_ammoPoolCount
C_Player!0x23a0 m_currentFrameLocalPlayer.m_stepSmoothingOffset
C_Player!0x23ac m_currentFrameLocalPlayer.m_duckTransitionRemainderMsec
C_Player!0x23b0 m_currentFrameLocalPlayer.m_vecPunchBase_Angle
C_Player!0x23bc m_currentFrameLocalPlayer.m_vecPunchBase_AngleVel
C_Player!0x23c8 m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle
C_Player!0x23d4 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.x
C_Player!0x23d8 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.y
C_Player!0x23dc m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.z
C_Player!0x2410 m_currentFrameLocalPlayer.m_localGravityRotation
C_Player!0x2430 pl
C_Player!0x24b4 m_ammoPoolCapacity
C_Player!0x2550 m_classModsActive
C_Player!0x2824 m_gestureSequences
C_Player!0x2844 m_gestureStartTimes
C_Player!0x2864 m_gestureBlendInDuration
C_Player!0x2884 m_gestureBlendOutDuration
C_Player!0x28a4 m_gestureFadeOutStartTime
C_Player!0x28c4 m_gestureFadeOutDuration
C_Player!0x28e4 m_gestureAutoKillBitfield
C_Player!0x2900 m_afButtonLast
C_Player!0x2904 m_afButtonPressed
C_Player!0x2908 m_afButtonReleased
C_Player!0x290c m_nButtons
C_Player!0x2910 m_nImpulse
C_Player!0x2914 m_flPhysics
C_Player!0x2918 m_flStepSoundTime
C_Player!0x291c m_flTimeAllSuitDevicesOff
C_Player!0x2920 m_fStickySprintMinTime
C_Player!0x2924 m_bPlayedSprintStartEffects
C_Player!0x292c m_fIsSprinting
C_Player!0x292d m_fIsWalking
C_Player!0x2930 m_lastSprintPressTime
C_Player!0x2934 m_stickySprintForwardEnableTime
C_Player!0x2938 m_stickySprintForwardDisableTime
C_Player!0x293c m_sprintStartedTime
C_Player!0x2940 m_sprintStartedFrac
C_Player!0x2944 m_sprintEndedTime
C_Player!0x2948 m_sprintEndedFrac
C_Player!0x294c m_stickySprintStartTime
C_Player!0x2950 m_damageImpulseNoDecelEndTime
C_Player!0x2970 m_duckState
C_Player!0x2974 m_leanState
C_Player!0x2978 m_doingHalfDuck
C_Player!0x2979 m_canStand
C_Player!0x297c m_StandHullMin
C_Player!0x2988 m_StandHullMax
C_Player!0x2994 m_DuckHullMin
C_Player!0x29a0 m_DuckHullMax
C_Player!0x29b0 m_upDir
C_Player!0x29bc m_upDirPredicted
C_Player!0x29c8 m_lastWallRunStartPos
C_Player!0x29d4 m_wallRunCount
C_Player!0x29d8 m_wallRunWeak
C_Player!0x29d9 m_shouldBeOneHanded
C_Player!0x29dc m_oneHandFraction
C_Player!0x29e0 m_animAimPitch
C_Player!0x29e4 m_animAimYaw
C_Player!0x29e8 m_wallRunPushAwayTime
C_Player!0x29f4 m_wallrunRetryTime
C_Player!0x29f8 m_wallrunRetryPos
C_Player!0x2a04 m_wallrunRetryNormal
C_Player!0x2a28 m_wallHangTime
C_Player!0x2a2c m_traversalState
C_Player!0x2a30 m_traversalType
C_Player!0x2a34 m_traversalBegin
C_Player!0x2a40 m_traversalMid
C_Player!0x2a4c m_traversalEnd
C_Player!0x2a58 m_traversalMidFrac
C_Player!0x2a5c m_traversalForwardDir
C_Player!0x2a68 m_traversalRefPos
C_Player!0x2a74 m_traversalProgress
C_Player!0x2a78 m_traversalStartTime
C_Player!0x2a7c m_traversalHandAppearTime
C_Player!0x2a80 m_traversalReleaseTime
C_Player!0x2a84 m_traversalBlendOutStartTime
C_Player!0x2a88 m_traversalBlendOutStartOffset
C_Player!0x2a94 m_traversalYawDelta
C_Player!0x2aa0 m_wallDangleJumpOffTime
C_Player!0x2aa4 m_wallDangleMayHangHere
C_Player!0x2aa5 m_wallDangleForceFallOff
C_Player!0x2aa6 m_wallDangleLastPushedForward
C_Player!0x2aa8 m_wallDangleDisableWeapon
C_Player!0x2aac m_wallDangleClimbProgressFloor
C_Player!0x2ab0 m_wallClimbSetUp
C_Player!0x2ab1 m_wallHanging
C_Player!0x2ab8 m_grapple
C_Player!0x2ab8 m_grapple
C_Player!0x2b48 m_grappleActive
C_Player!0x2b48 m_grappleActive
C_Player!0x2b49 m_grappleNeedWindowCheck
C_Player!0x2b4c m_grappleNextWindowHint
C_Player!0x2b5c m_slowMoEnabled
C_Player!0x2b5d m_sliding
C_Player!0x2b5e m_slideLongJumpAllowed
C_Player!0x2b60 m_lastSlideTime
C_Player!0x2b64 m_lastSlideBoost
C_Player!0x2b68 m_gravityGrenadeStatusEffect
C_Player!0x2b6c m_bIsStickySprinting
C_Player!0x2b70 m_prevMoveYaw
C_Player!0x2b74 m_sprintTiltVel
C_Player!0x2b90 m_turret
C_Player!0x2b94 m_hViewModels
C_Player!0x2ba8 m_viewOffsetEntity
C_Player!0x2ce8 m_activeZipline
C_Player!0x2cec m_lastZipline
C_Player!0x2cf0 m_lastZiplineDetachTime
C_Player!0x2cf4 m_ziplineValid3pWeaponLayerAnim
C_Player!0x2cf8 m_ziplineState
C_Player!0x2d00 m_zipline
C_Player!0x2d70 m_ziplineViewOffsetPosition
C_Player!0x2d7c m_ziplineViewOffsetVelocity
C_Player!0x2d88 m_ziplineGrenadeEntity
C_Player!0x2d8c m_ziplineGrenadeBeginStationEntity
C_Player!0x2d90 m_ziplineGrenadeBeginStationAttachmentIndex
C_Player!0x2d98 m_sameZiplineCooldownTime
C_Player!0x2da0 m_playAnimationType
C_Player!0x2da4 m_detachGrappleOnPlayAnimationEnd
C_Player!0x2da8 m_playAnimationNext
C_Player!0x2dbc m_boosting
C_Player!0x2dbd m_activateBoost
C_Player!0x2dbe m_repeatedBoost
C_Player!0x2dc0 m_boostMeter
C_Player!0x2dc4 m_jetpack
C_Player!0x2dc5 m_activateJetpack
C_Player!0x2dc6 m_jetpackAfterburner
C_Player!0x2dc7 m_gliding
C_Player!0x2dc8 m_glideMeter
C_Player!0x2dcc m_glideRechargeDelayAccumulator
C_Player!0x2dd0 m_hovering
C_Player!0x2dd1 m_isPerformingBoostAction
C_Player!0x2dd4 m_lastJumpHeight
C_Player!0x2dd8 m_touchingUpdraftTriggers
C_Player!0x2e18 m_touchingUpdraftTriggersCount
C_Player!0x2e1c m_touchingSlipTriggers
C_Player!0x2e5c m_touchingSlipTriggersCount
C_Player!0x2e60 m_slipAirRestrictDirection
C_Player!0x2e6c m_slipAirRestrictTime
C_Player!0x3000 m_melee
C_Player!0x3038 m_useCredit
C_Player!0x33b4 m_wallRunStartTime
C_Player!0x33b8 m_wallRunClearTime
C_Player!0x33bc m_onSlopeTime
C_Player!0x33c0 m_lastWallNormal
C_Player!0x33cc m_dodging
C_Player!0x33d0 m_lastDodgeTime
C_Player!0x33d4 m_vecPreviouslyPredictedOrigin
C_Player!0x33ec m_flTimeLastTouchedWall
C_Player!0x33f0 m_timeJetpackHeightActivateCheckPassed
C_Player!0x33f4 m_flTimeLastTouchedGround
C_Player!0x33f8 m_flTimeLastJumped
C_Player!0x33fc m_flTimeLastLanded
C_Player!0x3400 m_flLastLandFromHeight
C_Player!0x3404 m_usePressedTime
C_Player!0x3408 m_lastUseTime
C_Player!0x3418 m_lastFakeFloorPos
C_Player!0x3424 m_bHasJumpedSinceTouchedGround
C_Player!0x3425 m_bDoMultiJumpPenalty
C_Player!0x3426 m_dodgingInAir
C_Player!0x3610 m_activeViewmodelModifiers
C_Player!0x3890 m_lastMoveInputTime
C_Player!0x3894 m_ignoreEntityForMovementUntilNotTouching
C_Player!0x3d60 m_gameMovementUtil.m_surfaceFriction
C_Player!0x3ddc m_lungeTargetEntity
C_Player!0x3de0 m_isLungingToPosition
C_Player!0x3de4 m_lungeTargetPosition
C_Player!0x3df0 m_lungeStartPositionOffset
C_Player!0x3dfc m_lungeEndPositionOffset
C_Player!0x3e08 m_lungeStartTime
C_Player!0x3e0c m_lungeEndTime
C_Player!0x3e10 m_lungeCanFly
C_Player!0x3e11 m_lungeLockPitch
C_Player!0x3e14 m_lungeStartPitch
C_Player!0x3e18 m_lungeSmoothTime
C_Player!0x3e1c m_lungeMaxTime
C_Player!0x3e20 m_lungeMaxEndSpeed
C_Player!0x40e8 m_vPrevGroundNormal
C_Player!0x42ac m_pushAwayFromTopAcceleration
C_Player!0x42d8 m_controllerModeActive
C_Player!0x42f0 m_skydiveForwardPoseValueVelocity
C_Player!0x42f4 m_skydiveForwardPoseValueTarget
C_Player!0x42f8 m_skydiveForwardPoseValueCurrent
C_Player!0x42fc m_skydiveSidePoseValueVelocity
C_Player!0x4300 m_skydiveSidePoseValueTarget
C_Player!0x4304 m_skydiveSidePoseValueCurrent
C_Player!0x4308 m_skydiveYawVelocity
C_Player!0x430c m_skydiveIsNearLeviathan
C_Player!0x4328 m_skydiveState
C_Player!0x432c m_skydiveStartTime
C_Player!0x4330 m_skydiveEndTime
C_Player!0x4334 m_skydiveAnticipateStartTime
C_Player!0x4338 m_skydiveAnticipateEndTime
C_Player!0x433c m_skydiveDistanceToLand
C_Player!0x4340 m_skydiveDiveAngle
C_Player!0x4344 m_skydiveIsDiving
C_Player!0x4348 m_skydiveSpeed
C_Player!0x434c m_skydiveStrafeAngle
C_Player!0x4350 m_skydiveFreelookEnabled
C_Player!0x4354 m_skydiveFreelookLockedAngle
C_Player!0x4360 m_skydivePlayerPitch
C_Player!0x4364 m_skydivePlayerYaw
C_Player!0x4368 m_skydiveFollowing
C_Player!0x436c m_skydiveUnfollowVelocity
C_Player!0x437c m_skydiveLeviathanHitPosition
C_Player!0x4388 m_skydiveLeviathanHitNormal
C_Player!0x4394 m_skydiveSlipVelocity
C_Player!0x43b8 m_playerKnockBacks
C_Player!0x4438 m_updraftCount
C_Player!0x443c m_updraftStage
C_Player!0x4440 m_updraftEnterTime
C_Player!0x4444 m_updraftLeaveTime
C_Player!0x4448 m_updraftMinShakeActivationHeight
C_Player!0x444c m_updraftMaxShakeActivationHeight
C_Player!0x4450 m_updraftLiftActivationHeight
C_Player!0x4454 m_updraftLiftSpeed
C_Player!0x4458 m_updraftLiftAcceleration
C_Player!0x445c m_updraftLiftExitDuration
C_Player!0x4460 m_updraftSlowTime
```
</details>
<details>
<summary><code>class C_PlayerLocalData</code></summary>

```
{
	m_nStepside: Int,
	m_nOldButtons: Int,
	m_nOldVehicleButtons: Int,
	m_iHideHUD: Int,
	m_superJumpsUsed: Int,
	m_jumpedOffRodeo: Bool,
	m_jumpPressTime: Time,
	m_jetpackActivateTime: Time,
	m_jetpackDeactivateTime: Time,
	m_flSuitPower: Float,
	m_flSuitJumpPower: Float,
	m_flSuitGrapplePower: Float,
	m_flFallVelocity: Float,
	m_flStepSize: Float,
	m_airSlowMoFrac: Float,
	predictableFlags: Int,
	m_bitsActiveDevices: Int,
	m_forceStance: Int,
	m_duckToggleOn: Bool,
	m_bDrawViewmodel: Bool,
	m_bAllowAutoMovement: Bool,
	m_airMoveBlockPlanes: Vector,
	m_airMoveBlockPlaneTime: Time,
	m_airMoveBlockPlaneCount: Int,
	m_queuedMeleePressTime: Time,
	m_queuedGrappleMeleeTime: Time,
	m_disableMeleeUntilRelease: Bool,
	m_meleePressTime: Time,
	m_meleeDisabledCounter: Int,
	m_meleeInputIndex: Int,
	m_oneHandedWeaponUsage: Bool,
	m_prevOneHandedWeaponUsage: Bool,
	m_titanEmbarkEnabled: Bool,
	m_titanDisembarkEnabled: Bool,
	m_playerAnimStationaryGoalFeetYaw: Float,
	m_playerAnimJumping: Bool,
	m_playerAnimJumpStartTime: Time,
	m_playerAnimFirstJumpFrame: Bool,
	m_playerAnimDodging: Bool,
	m_playerAnimJumpActivity: Int,
	m_playerAnimLanding: Bool,
	m_playerAnimShouldLand: Bool,
	m_playerAnimLandStartTime: Time,
	m_playerAnimInAirWalk: Bool,
	m_playerAnimPrevFrameSequenceMotionYaw: Float,
	m_playerAnimMeleeParity: Int,
	m_playerAnimMeleeStartTime: Time,
	m_playerLocalGravityToWorldTransform: Quaternion,
	m_playerLocalGravityBlendStartRotation: Quaternion,
	m_playerLocalGravityBlendEndRotation: Quaternion,
	m_playerLocalGravityBlendEndDirection: Vector,
	m_playerLocalGravityBlendStartTime: Time,
	m_playerLocalGravityBlendEndTime: Time,
	m_playerLocalGravityBlendStrength: Float,
	m_playerLocalGravityStrength: Float,
	m_playerLocalGravityType: Int,
	m_playerLocalGravityPoint: Vector,
	m_playerLocalGravityLineStart: Vector,
	m_playerLocalGravityLineEnd: Vector,
	m_playerLocalGravityEntity: EHANDLE,
	m_playerLocalGravityLineStartEntity: EHANDLE,
	m_playerLocalGravityLineEndEntity: EHANDLE,
	m_playerFloatLookStartTime: Time,
	m_playerFloatLookEndTime: Time,
	m_wallrunLatestFloorHeight: Float,
	m_wallrunFromJetpack: Bool,
	m_groundNormal: Vector,
	m_continuousUseBlocked: Bool,
	m_useEnt: EHANDLE,
}
```

### Offsets

```
C_PlayerLocalData!0x0008 m_nStepside
C_PlayerLocalData!0x000c m_nOldButtons
C_PlayerLocalData!0x0010 m_nOldVehicleButtons
C_PlayerLocalData!0x0014 m_iHideHUD
C_PlayerLocalData!0x0018 m_superJumpsUsed
C_PlayerLocalData!0x001c m_jumpedOffRodeo
C_PlayerLocalData!0x0020 m_jumpPressTime
C_PlayerLocalData!0x0024 m_jetpackActivateTime
C_PlayerLocalData!0x0028 m_jetpackDeactivateTime
C_PlayerLocalData!0x002c m_flSuitPower
C_PlayerLocalData!0x0030 m_flSuitJumpPower
C_PlayerLocalData!0x0034 m_flSuitGrapplePower
C_PlayerLocalData!0x0038 m_flFallVelocity
C_PlayerLocalData!0x003c m_flStepSize
C_PlayerLocalData!0x0040 m_airSlowMoFrac
C_PlayerLocalData!0x0044 predictableFlags
C_PlayerLocalData!0x0048 m_bitsActiveDevices
C_PlayerLocalData!0x004c m_forceStance
C_PlayerLocalData!0x0050 m_duckToggleOn
C_PlayerLocalData!0x0051 m_bDrawViewmodel
C_PlayerLocalData!0x0052 m_bAllowAutoMovement
C_PlayerLocalData!0x0180 m_airMoveBlockPlanes
C_PlayerLocalData!0x0198 m_airMoveBlockPlaneTime
C_PlayerLocalData!0x019c m_airMoveBlockPlaneCount
C_PlayerLocalData!0x01a0 m_queuedMeleePressTime
C_PlayerLocalData!0x01a4 m_queuedGrappleMeleeTime
C_PlayerLocalData!0x01a9 m_disableMeleeUntilRelease
C_PlayerLocalData!0x01ac m_meleePressTime
C_PlayerLocalData!0x01b0 m_meleeDisabledCounter
C_PlayerLocalData!0x01b4 m_meleeInputIndex
C_PlayerLocalData!0x01bc m_oneHandedWeaponUsage
C_PlayerLocalData!0x01bd m_prevOneHandedWeaponUsage
C_PlayerLocalData!0x01f0 m_titanEmbarkEnabled
C_PlayerLocalData!0x01f1 m_titanDisembarkEnabled
C_PlayerLocalData!0x01f8 m_playerAnimStationaryGoalFeetYaw
C_PlayerLocalData!0x01fc m_playerAnimJumping
C_PlayerLocalData!0x0200 m_playerAnimJumpStartTime
C_PlayerLocalData!0x0204 m_playerAnimFirstJumpFrame
C_PlayerLocalData!0x0205 m_playerAnimDodging
C_PlayerLocalData!0x0208 m_playerAnimJumpActivity
C_PlayerLocalData!0x020c m_playerAnimLanding
C_PlayerLocalData!0x020d m_playerAnimShouldLand
C_PlayerLocalData!0x0210 m_playerAnimLandStartTime
C_PlayerLocalData!0x0214 m_playerAnimInAirWalk
C_PlayerLocalData!0x0218 m_playerAnimPrevFrameSequenceMotionYaw
C_PlayerLocalData!0x021c m_playerAnimMeleeParity
C_PlayerLocalData!0x0220 m_playerAnimMeleeStartTime
C_PlayerLocalData!0x0224 m_playerLocalGravityToWorldTransform
C_PlayerLocalData!0x0254 m_playerLocalGravityBlendStartRotation
C_PlayerLocalData!0x0264 m_playerLocalGravityBlendEndRotation
C_PlayerLocalData!0x0274 m_playerLocalGravityBlendEndDirection
C_PlayerLocalData!0x0280 m_playerLocalGravityBlendStartTime
C_PlayerLocalData!0x0284 m_playerLocalGravityBlendEndTime
C_PlayerLocalData!0x0288 m_playerLocalGravityBlendStrength
C_PlayerLocalData!0x028c m_playerLocalGravityStrength
C_PlayerLocalData!0x0290 m_playerLocalGravityType
C_PlayerLocalData!0x0294 m_playerLocalGravityPoint
C_PlayerLocalData!0x02a0 m_playerLocalGravityLineStart
C_PlayerLocalData!0x02ac m_playerLocalGravityLineEnd
C_PlayerLocalData!0x02b8 m_playerLocalGravityEntity
C_PlayerLocalData!0x02bc m_playerLocalGravityLineStartEntity
C_PlayerLocalData!0x02c0 m_playerLocalGravityLineEndEntity
C_PlayerLocalData!0x02c4 m_playerFloatLookStartTime
C_PlayerLocalData!0x02c8 m_playerFloatLookEndTime
C_PlayerLocalData!0x02cc m_wallrunLatestFloorHeight
C_PlayerLocalData!0x02d0 m_wallrunFromJetpack
C_PlayerLocalData!0x02d4 m_groundNormal
C_PlayerLocalData!0x02e0 m_continuousUseBlocked
C_PlayerLocalData!0x02e4 m_useEnt
```
</details>
<details>
<summary><code>class C_PlayerResource extends C_BaseEntity</code></summary>

```
{
	m_szName: String,
	m_boolStats: Int,
	m_killStats: Int,
	m_scoreStats: Int,
	m_iPing: Int,
	m_bConnected: Bool,
}
```

### Offsets

```
C_PlayerResource!0x0a40 m_szName
C_PlayerResource!0x1450 m_boolStats
C_PlayerResource!0x1654 m_killStats
C_PlayerResource!0x226c m_scoreStats
C_PlayerResource!0x2c80 m_iPing
C_PlayerResource!0x2e84 m_bConnected
```
</details>
<details>
<summary><code>class C_PlayerVehicle extends C_BaseAnimatingOverlay</code></summary>

```
{
	m_localOrigin: Vector,
	m_SequenceTransitioner: C_SequenceTransitioner,
	m_vehicleDriver: EHANDLE,
	m_vehicleActivated: Bool,
	m_blockDuckInput: Bool,
	m_vehicleLaunchTime: Float,
	m_vehicleVelocity: Vector,
	m_vehicleGroundEntity: EHANDLE,
	m_vehicleGroundNormal: Vector,
	m_vehicleGroundDist: Float,
	m_hoverVehicleHoverOffsetPrev: Float,
	m_hoverVehicleGroundAngles: Vector,
	m_hoverVehicleHoverSimulationIsAwake: Bool,
	m_hoverVehicleSmoothTilt: Vector,
	m_hoverVehicleSmoothTiltVelocity: Vector,
	m_hoverVehicleSmoothYaw: Float,
	m_hoverVehicleSmoothYawVelocity: Float,
	m_hoverVehicleLookAheadAcceleration: Vector,
	m_hoverVehicleLookAheadTilt: Vector,
	m_hoverVehicleLastBoostTime: Time,
	m_hoverVehicleBoostVelocity: Vector,
	m_hoverVehicleStunTimeEnd: Float,
	m_hoverVehicleThrottle: Float,
	m_hoverVehicleBanking: Float,
	m_hoverVehicleFrictionLastTime: Float,
	m_hoverVehicleFrictionSurfPropOther: Int,
	m_hoverVehicleFrictionNormal: Vector,
	m_hoverVehicleFrictionPos: Vector,
}
```

### Offsets

```
C_PlayerVehicle!0x0158 m_localOrigin
C_PlayerVehicle!0x0c00 m_SequenceTransitioner
C_PlayerVehicle!0x1984 m_vehicleDriver
C_PlayerVehicle!0x19a8 m_vehicleActivated
C_PlayerVehicle!0x19a9 m_blockDuckInput
C_PlayerVehicle!0x19b8 m_vehicleLaunchTime
C_PlayerVehicle!0x19c0 m_vehicleVelocity
C_PlayerVehicle!0x19cc m_vehicleGroundEntity
C_PlayerVehicle!0x19d0 m_vehicleGroundNormal
C_PlayerVehicle!0x19dc m_vehicleGroundDist
C_PlayerVehicle!0x19e0 m_hoverVehicleHoverOffsetPrev
C_PlayerVehicle!0x19f0 m_hoverVehicleGroundAngles
C_PlayerVehicle!0x19fc m_hoverVehicleHoverSimulationIsAwake
C_PlayerVehicle!0x1a50 m_hoverVehicleSmoothTilt
C_PlayerVehicle!0x1a5c m_hoverVehicleSmoothTiltVelocity
C_PlayerVehicle!0x1a68 m_hoverVehicleSmoothYaw
C_PlayerVehicle!0x1a6c m_hoverVehicleSmoothYawVelocity
C_PlayerVehicle!0x1a70 m_hoverVehicleLookAheadAcceleration
C_PlayerVehicle!0x1a7c m_hoverVehicleLookAheadTilt
C_PlayerVehicle!0x1a88 m_hoverVehicleLastBoostTime
C_PlayerVehicle!0x1a90 m_hoverVehicleBoostVelocity
C_PlayerVehicle!0x1aa0 m_hoverVehicleStunTimeEnd
C_PlayerVehicle!0x1aa4 m_hoverVehicleThrottle
C_PlayerVehicle!0x1aa8 m_hoverVehicleBanking
C_PlayerVehicle!0x1aac m_hoverVehicleFrictionLastTime
C_PlayerVehicle!0x1ab0 m_hoverVehicleFrictionSurfPropOther
C_PlayerVehicle!0x1ab4 m_hoverVehicleFrictionNormal
C_PlayerVehicle!0x1ac0 m_hoverVehicleFrictionPos
```
</details>
<details>
<summary><code>class C_Projectile extends C_BaseEntity</code></summary>

```
{
	m_weaponDataIsSet: Bool,
	m_forceAdjustToGunBarrelDisabled: Bool,
	m_weaponClassIndex: Int,
	m_destructionDistance: Float,
	m_passThroughDepthTotal: Int,
	m_modBitfield: Int,
	m_overrideMods: Int,
	m_projectileTrailIndex: Int,
	m_impactEffectTable: Int,
	m_reducedEffects: Bool,
	m_projectileCreationTimeServer: Float,
	m_weaponSource: EHANDLE,
	m_wpnData: Outer,
	m_hWeaponFileInfo: Short,
	m_weaponChargeLevel: Int,
	m_weaponChargeFraction: Float,
	m_modVars: Void,
	m_modVarsAreValid: Bool,
	m_launchOrigin: Vector,
	m_launchVel: Vector,
	m_scriptCB: Void,
	m_hasPlayedTrailEffect: Bool,
	m_projectileLifeTimeEndTick: Tick,
	m_projectileCreationTime: Float,
	m_isVortexRefired: Bool,
	m_damageAliveOnly: Bool,
	m_usesPositionFunction: Bool,
	m_lastCollisionNormal: Vector,
	m_bounceIndex: Int,
	m_randomInt: Int,
	m_thrownByAI: Bool,
	m_perPolyRadius: Float,
	m_posBeforePhysicsSimulate: Vector,
	m_hasIgnited: Bool,
	m_inLagCompensation: Bool,
	m_passEntities: EHANDLE,
	m_projectileSpeed: Float,
	m_wantStartTrailEffect: Bool,
	m_hasCalledPostDataUpdate: Bool,
}
```

### Offsets

```
C_Projectile!0x1600 m_weaponDataIsSet
C_Projectile!0x1601 m_forceAdjustToGunBarrelDisabled
C_Projectile!0x1604 m_weaponClassIndex
C_Projectile!0x1608 m_destructionDistance
C_Projectile!0x160c m_passThroughDepthTotal
C_Projectile!0x1610 m_modBitfield
C_Projectile!0x1614 m_overrideMods
C_Projectile!0x1618 m_projectileTrailIndex
C_Projectile!0x161c m_impactEffectTable
C_Projectile!0x1620 m_reducedEffects
C_Projectile!0x1624 m_projectileCreationTimeServer
C_Projectile!0x1628 m_weaponSource
C_Projectile!0x1630 m_wpnData
C_Projectile!0x1638 m_hWeaponFileInfo
C_Projectile!0x163c m_weaponChargeLevel
C_Projectile!0x1640 m_weaponChargeFraction
C_Projectile!0x1648 m_modVars
C_Projectile!0x28b0 m_modVarsAreValid
C_Projectile!0x28b4 m_launchOrigin
C_Projectile!0x28c0 m_launchVel
C_Projectile!0x28d0 m_scriptCB
C_Projectile!0x28f8 m_hasPlayedTrailEffect
C_Projectile!0x28fc m_projectileLifeTimeEndTick
C_Projectile!0x2900 m_projectileCreationTime
C_Projectile!0x2904 m_isVortexRefired
C_Projectile!0x2905 m_damageAliveOnly
C_Projectile!0x2906 m_usesPositionFunction
C_Projectile!0x2908 m_lastCollisionNormal
C_Projectile!0x2914 m_bounceIndex
C_Projectile!0x2918 m_randomInt
C_Projectile!0x291c m_thrownByAI
C_Projectile!0x2920 m_perPolyRadius
C_Projectile!0x2928 m_posBeforePhysicsSimulate
C_Projectile!0x2934 m_hasIgnited
C_Projectile!0x2935 m_inLagCompensation
C_Projectile!0x2938 m_passEntities
C_Projectile!0x29a0 m_projectileSpeed
C_Projectile!0x29c0 m_wantStartTrailEffect
C_Projectile!0x29c2 m_hasCalledPostDataUpdate
```
</details>
<details>
<summary><code>class C_PropDoor</code></summary>

```
{
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_nNextThinkTick: Int,
	m_angle: Float,
	m_startAngle: Float,
	m_startAngleVel: Float,
	m_startMoveTime: Time,
	m_nextHitSoundTime: Float,
	m_lastThinkTime: Float,
	m_interactingPlayer: EHANDLE,
	m_interactingPlayerWantsOpen: Bool,
	m_useDebounceEndTime: Time,
	m_prevAngle: Float,
}
```

### Offsets

```
C_PropDoor!0x0158 m_localOrigin
C_PropDoor!0x0164 m_localAngles
C_PropDoor!0x054c m_nNextThinkTick
C_PropDoor!0x1674 m_angle
C_PropDoor!0x1678 m_startAngle
C_PropDoor!0x167c m_startAngleVel
C_PropDoor!0x1680 m_startMoveTime
C_PropDoor!0x168c m_nextHitSoundTime
C_PropDoor!0x1690 m_lastThinkTime
C_PropDoor!0x16d8 m_interactingPlayer
C_PropDoor!0x16dc m_interactingPlayerWantsOpen
C_PropDoor!0x16e0 m_useDebounceEndTime
C_PropDoor!0x16e8 m_prevAngle
```
</details>
<details>
<summary><code>class C_SequenceTransitioner</code></summary>

```
{
	m_sequenceTransitionerLayers: C_SequenceTransitionerLayer,
	m_sequenceTransitionerLayerCount: Int,
}
```

### Offsets

```
C_SequenceTransitioner!0x0050 m_sequenceTransitionerLayers
C_SequenceTransitioner!0x01a0 m_sequenceTransitionerLayerCount
```
</details>
<details>
<summary><code>class C_SequenceTransitionerLayer</code></summary>

```
{
	m_sequenceTransitionerLayerActive: Bool,
	m_sequenceTransitionerLayerStartCycle: Float,
	m_sequenceTransitionerLayerSequence: Int,
	m_weight: Float,
	m_sequenceTransitionerLayerPlaybackRate: Float,
	m_sequenceTransitionerLayerStartTime: Time,
	m_sequenceTransitionerLayerFadeOutDuration: Float,
}
```

### Offsets

```
C_SequenceTransitionerLayer!0x0018 m_sequenceTransitionerLayerActive
C_SequenceTransitionerLayer!0x001c m_sequenceTransitionerLayerStartCycle
C_SequenceTransitionerLayer!0x0020 m_sequenceTransitionerLayerSequence
C_SequenceTransitionerLayer!0x0024 m_weight
C_SequenceTransitionerLayer!0x0028 m_sequenceTransitionerLayerPlaybackRate
C_SequenceTransitionerLayer!0x002c m_sequenceTransitionerLayerStartTime
C_SequenceTransitionerLayer!0x0030 m_sequenceTransitionerLayerFadeOutDuration
```
</details>
<details>
<summary><code>class C_Team extends C_BaseEntity</code></summary>

```
{
	m_score: Int,
	m_score2: Int,
	m_kills: Int,
	m_deaths: Int,
	m_iRoundsWon: Int,
	m_iTeamTeamNum: Int,
	m_szTeamname: Char,
}
```

### Offsets

```
C_Team!0x0a40 m_score
C_Team!0x0a44 m_score2
C_Team!0x0a48 m_kills
C_Team!0x0a4c m_deaths
C_Team!0x0a50 m_iRoundsWon
C_Team!0x0a54 m_iTeamTeamNum
C_Team!0x0a78 m_szTeamname
```
</details>
<details>
<summary><code>class C_TriggerCylinderHeavy</code></summary>

```
{
	m_teslaTrapObstructedEndTime: Time,
}
```

### Offsets

```
C_TriggerCylinderHeavy!0x0b08 m_teslaTrapObstructedEndTime
```
</details>
<details>
<summary><code>class C_VortexSphere extends C_BaseEntity</code></summary>

```
{
	m_enabled: Bool,
	m_radius: Float,
	m_height: Float,
	m_bulletFov: Float,
	m_bulletAbsorbedCount: Int,
	m_projectileAbsorbedCount: Int,
	m_ownerWeapon: EHANDLE,
	m_vortexEffect: EHANDLE,
	m_vortexLocalAngles: Vector,
	m_gunAttachment: String,
	m_listPrev: Outer,
	m_listNext: Outer,
}
```

### Offsets

```
C_VortexSphere!0x0a40 m_enabled
C_VortexSphere!0x0a44 m_radius
C_VortexSphere!0x0a48 m_height
C_VortexSphere!0x0a4c m_bulletFov
C_VortexSphere!0x0a50 m_bulletAbsorbedCount
C_VortexSphere!0x0a54 m_projectileAbsorbedCount
C_VortexSphere!0x0a58 m_ownerWeapon
C_VortexSphere!0x0a5c m_vortexEffect
C_VortexSphere!0x0a60 m_vortexLocalAngles
C_VortexSphere!0x0a70 m_gunAttachment
C_VortexSphere!0x0a78 m_listPrev
C_VortexSphere!0x0a80 m_listNext
```
</details>
<details>
<summary><code>class C_WallrunCurve extends C_GameplayHint</code></summary>

```
{
	width: Int,
	height: Int,
}
```

### Offsets

```
C_WallrunCurve!0x0a80 width
C_WallrunCurve!0x0a84 height
```
</details>
<details>
<summary><code>class C_WindowHint extends C_GameplayHint</code></summary>

```
{
	normal: Vector,
	right: Vector,
	halfSize: Float,
	halfSize[0]: Float,
	halfSize[1]: Float,
}
```

### Offsets

```
C_WindowHint!0x0a80 normal
C_WindowHint!0x0a8c right
C_WindowHint!0x0a98 halfSize
C_WindowHint!0x0a98 halfSize[0]
C_WindowHint!0x0a9c halfSize[1]
```
</details>
<details>
<summary><code>class C_Zipline extends C_BaseEntity</code></summary>

```
{
	m_ziplinePhysics: C_ZiplinePhysics,
	m_detachEndOnUse: Bool,
	m_currentFrameZipline.numZiplinePoints: Int,
	m_currentFrameZipline.ziplinePositions: Vector,
	m_currentFrameZipline.ziplinePreviousPositions: Vector,
	m_currentFrameZipline.ziplineDistances: Float,
}
```

### Offsets

```
C_Zipline!0x0a40 m_ziplinePhysics
C_Zipline!0x0d94 m_detachEndOnUse
C_Zipline!0x0e80 m_currentFrameZipline.numZiplinePoints
C_Zipline!0x0e84 m_currentFrameZipline.ziplinePositions
C_Zipline!0x0f44 m_currentFrameZipline.ziplinePreviousPositions
C_Zipline!0x1004 m_currentFrameZipline.ziplineDistances
```
</details>
<details>
<summary><code>class C_ZiplinePhysics</code></summary>

```
{
	m_ziplineType: Int,
	m_ziplineStart: Vector,
	m_ziplineEnd: Vector,
	m_nodes: C_ZiplinePhysicsNode,
	m_numNodes: Int,
	m_springDistance: Int,
	m_remainingUnsimulatedTime: Float,
	m_attachedEntities: C_ZiplinePhysicsAttachedEntity,
	m_numAttachedEntities: Int,
	m_ziplineOwner: EHANDLE,
}
```

### Offsets

```
C_ZiplinePhysics!0x000c m_ziplineType
C_ZiplinePhysics!0x0010 m_ziplineStart
C_ZiplinePhysics!0x001c m_ziplineEnd
C_ZiplinePhysics!0x0028 m_nodes
C_ZiplinePhysics!0x0228 m_numNodes
C_ZiplinePhysics!0x022c m_springDistance
C_ZiplinePhysics!0x0234 m_remainingUnsimulatedTime
C_ZiplinePhysics!0x0240 m_attachedEntities
C_ZiplinePhysics!0x0340 m_numAttachedEntities
C_ZiplinePhysics!0x0344 m_ziplineOwner
```
</details>
<details>
<summary><code>class C_ZiplinePhysicsAttachedEntity</code></summary>

```
{
	entity: EHANDLE,
	attachAcceleration: Vector,
	attachTime: Float,
}
```

### Offsets

```
C_ZiplinePhysicsAttachedEntity!0x0008 entity
C_ZiplinePhysicsAttachedEntity!0x000c attachAcceleration
C_ZiplinePhysicsAttachedEntity!0x0018 attachTime
```
</details>
<details>
<summary><code>class C_ZiplinePhysicsNode</code></summary>

```
{
	position: Vector,
	prevPosition: Vector,
}
```

### Offsets

```
C_ZiplinePhysicsNode!0x0008 position
C_ZiplinePhysicsNode!0x0014 prevPosition
```
</details>
<details>
<summary><code>class GrappleData</code></summary>

```
{
	m_grappleVel: Vector,
	m_grapplePoints: Vector,
	m_grapplePointCount: Int,
	m_grappleAttached: Bool,
	m_grapplePulling: Bool,
	m_grappleSwinging: Bool,
	m_grappleRetracting: Bool,
	m_grappleForcedRetracting: Bool,
	m_grappleGracePeriodFinished: Bool,
	m_grappleUsedPower: Float,
	m_grappleActivateTime: Time,
	m_grapplePullTime: Time,
	m_grappleAttachTime: Time,
	m_grappleDetachTime: Time,
	m_grappleMeleeTarget: EHANDLE,
	m_grappleAutoAimTarget: EHANDLE,
	m_grappleSwingDetachLowSpeed: Float,
	m_grappleSwingHoldTime: Time,
}
```

### Offsets

```
GrappleData!0x0008 m_grappleVel
GrappleData!0x0014 m_grapplePoints
GrappleData!0x0044 m_grapplePointCount
GrappleData!0x0048 m_grappleAttached
GrappleData!0x0049 m_grapplePulling
GrappleData!0x004a m_grappleSwinging
GrappleData!0x004b m_grappleRetracting
GrappleData!0x004c m_grappleForcedRetracting
GrappleData!0x004d m_grappleGracePeriodFinished
GrappleData!0x0050 m_grappleUsedPower
GrappleData!0x0054 m_grappleActivateTime
GrappleData!0x0058 m_grapplePullTime
GrappleData!0x005c m_grappleAttachTime
GrappleData!0x0060 m_grappleDetachTime
GrappleData!0x0064 m_grappleMeleeTarget
GrappleData!0x0068 m_grappleAutoAimTarget
GrappleData!0x0074 m_grappleSwingDetachLowSpeed
GrappleData!0x0078 m_grappleSwingHoldTime
```
</details>
<details>
<summary><code>class MissilePathExpandContractSettings_Client</code></summary>

```
{
	launchOutVec: Vector,
	launchInVec: Vector,
	launchOutTime: Time,
	launchInLerpTime: Time,
	launchInTime: Time,
	launchStraightLerpTime: Time,
	endPos: Vector,
	applyRandSpread: Bool,
}
```

### Offsets

```
MissilePathExpandContractSettings_Client!0x0004 launchOutVec
MissilePathExpandContractSettings_Client!0x0010 launchInVec
MissilePathExpandContractSettings_Client!0x0028 launchOutTime
MissilePathExpandContractSettings_Client!0x002c launchInLerpTime
MissilePathExpandContractSettings_Client!0x0030 launchInTime
MissilePathExpandContractSettings_Client!0x0034 launchStraightLerpTime
MissilePathExpandContractSettings_Client!0x003c endPos
MissilePathExpandContractSettings_Client!0x0054 applyRandSpread
```
</details>
<details>
<summary><code>class PlayerMelee_PlayerData</code></summary>

```
{
	meleeAttackParity: Int,
	attackActive: Bool,
	attackRecoveryShouldBeQuick: Bool,
	isSprintAttack: Bool,
	attackStartTime: Time,
	attackHitEntity: EHANDLE,
	attackHitEntityTime: Time,
	attackLastHitNonWorldEntity: Time,
	scriptedState: Int,
	pendingMeleePress: Bool,
	lungeBoost: Vector,
}
```

### Offsets

```
PlayerMelee_PlayerData!0x0008 meleeAttackParity
PlayerMelee_PlayerData!0x000c attackActive
PlayerMelee_PlayerData!0x000d attackRecoveryShouldBeQuick
PlayerMelee_PlayerData!0x000e isSprintAttack
PlayerMelee_PlayerData!0x0010 attackStartTime
PlayerMelee_PlayerData!0x0014 attackHitEntity
PlayerMelee_PlayerData!0x0018 attackHitEntityTime
PlayerMelee_PlayerData!0x001c attackLastHitNonWorldEntity
PlayerMelee_PlayerData!0x0020 scriptedState
PlayerMelee_PlayerData!0x0024 pendingMeleePress
PlayerMelee_PlayerData!0x0028 lungeBoost
```
</details>
<details>
<summary><code>class PlayerZiplineData_Client</code></summary>

```
{
	m_ziplineReenableWeapons: Bool,
	m_mountingZiplineDuration: Float,
	m_mountingZiplineAlpha: Float,
	m_ziplineStartTime: Time,
	m_ziplineEndTime: Time,
	m_mountingZiplineSourcePosition: Vector,
	m_mountingZiplineSourceVelocity: Vector,
	m_mountingZiplineTargetPosition: Vector,
	m_ziplineUsePosition: Vector,
	m_slidingZiplineAlpha: Float,
	m_lastMoveDir2D: Vector,
	m_ziplineReverse: Bool,
}
```

### Offsets

```
PlayerZiplineData_Client!0x0008 m_ziplineReenableWeapons
PlayerZiplineData_Client!0x000c m_mountingZiplineDuration
PlayerZiplineData_Client!0x0010 m_mountingZiplineAlpha
PlayerZiplineData_Client!0x0014 m_ziplineStartTime
PlayerZiplineData_Client!0x0018 m_ziplineEndTime
PlayerZiplineData_Client!0x001c m_mountingZiplineSourcePosition
PlayerZiplineData_Client!0x0028 m_mountingZiplineSourceVelocity
PlayerZiplineData_Client!0x0034 m_mountingZiplineTargetPosition
PlayerZiplineData_Client!0x004c m_ziplineUsePosition
PlayerZiplineData_Client!0x0058 m_slidingZiplineAlpha
PlayerZiplineData_Client!0x005c m_lastMoveDir2D
PlayerZiplineData_Client!0x0068 m_ziplineReverse
```
</details>
<details>
<summary><code>class Player_ViewOffsetEntityData</code></summary>

```
{
	viewOffsetEntityHandle: EHANDLE,
	lerpInDuration: Float,
	lerpOutDuration: Float,
	stabilizePlayerEyeAngles: Bool,
}
```

### Offsets

```
Player_ViewOffsetEntityData!0x0008 viewOffsetEntityHandle
Player_ViewOffsetEntityData!0x000c lerpInDuration
Player_ViewOffsetEntityData!0x0010 lerpOutDuration
Player_ViewOffsetEntityData!0x0014 stabilizePlayerEyeAngles
```
</details>
<details>
<summary><code>class PredictedAnimEventData</code></summary>

```
{
	m_predictedAnimEventTimes: Time,
	m_predictedAnimEventIndices: Int,
	m_predictedAnimEventCount: Int,
	m_predictedAnimEventTarget: EHANDLE,
	m_predictedAnimEventSequence: Int,
	m_predictedAnimEventModel: Int,
	m_predictedAnimEventsReadyToFireTime: Time,
}
```

### Offsets

```
PredictedAnimEventData!0x0008 m_predictedAnimEventTimes
PredictedAnimEventData!0x0028 m_predictedAnimEventIndices
PredictedAnimEventData!0x0048 m_predictedAnimEventCount
PredictedAnimEventData!0x004c m_predictedAnimEventTarget
PredictedAnimEventData!0x0050 m_predictedAnimEventSequence
PredictedAnimEventData!0x0054 m_predictedAnimEventModel
PredictedAnimEventData!0x0058 m_predictedAnimEventsReadyToFireTime
```
</details>
<details>
<summary><code>class StatusEffectEndlessData</code></summary>

```
{
	seComboVars: Int,
}
```

### Offsets

```
StatusEffectEndlessData!0x0008 seComboVars
```
</details>
<details>
<summary><code>class StatusEffectTimedData</code></summary>

```
{
	seComboVars: Int,
	seTimeEnd: Float,
	seEaseOut: Float,
}
```

### Offsets

```
StatusEffectTimedData!0x0008 seComboVars
StatusEffectTimedData!0x000c seTimeEnd
StatusEffectTimedData!0x0010 seEaseOut
```
</details>
<details>
<summary><code>class WeaponInventory_Client</code></summary>

```
{
	weapons: EHANDLE,
	activeWeapons: EHANDLE,
}
```

### Offsets

```
WeaponInventory_Client!0x0008 weapons
WeaponInventory_Client!0x004c activeWeapons
```
</details>
<details>
<summary><code>class WeaponPlayerData</code></summary>

```
{
	m_moveSpread: Float,
	m_spreadStartTime: Time,
	m_spreadStartFracHip: Float,
	m_spreadStartFracADS: Float,
	m_kickSpreadHipfire: Float,
	m_kickSpreadADS: Float,
	m_kickTime: Time,
	m_kickScaleBasePitch: Float,
	m_kickScaleBaseYaw: Float,
	m_kickPatternScaleBase: Float,
	m_kickSpringHeatBaseTime: Time,
	m_kickSpringHeatBaseValue: Float,
	m_semiAutoTriggerHoldTime: Time,
	m_semiAutoTriggerDown: Bool,
	m_pendingTriggerPull: Bool,
	m_pendingTriggerUpForCharge: Bool,
	m_semiAutoNeedsRechamber: Bool,
	m_pendingReloadAttempt: Bool,
	m_offhandHybridNormalMode: Bool,
	m_pendingoffhandHybridToss: Bool,
	m_fastHolster: Bool,
	m_didFirstDeploy: Bool,
	m_shouldCatch: Bool,
	m_clipModelIsHidden: Bool,
	m_segmentedReloadEndSeqRequired: Bool,
	m_reloadStartedEmpty: Bool,
	m_segmentedAnimStartedOneHanded: Bool,
	m_segmentedReloadCanRestartLoop: Bool,
	m_segmentedReloadLoopFireLocked: Bool,
	m_realtimeModCmds: Char,
	m_realtimeModCmdHead: Char,
	m_realtimeModCmdCount: Char,
	m_customActivityAttachedModelIndex: Int,
	m_customActivityAttachedModelAttachmentIndex: Int,
	m_fireRateLerp_startTime: Time,
	m_fireRateLerp_startFraction: Float,
	m_fireRateLerp_stopTime: Time,
	m_fireRateLerp_stopFraction: Float,
	m_chargeAnimIndex: Int,
	m_chargeAnimIndexOld: Int,
	m_reloadMilestone: Int,
	m_rechamberMilestone: Int,
	m_cooldownMilestone: Int,
	m_prevSeqWeight: Int,
	m_fullReloadStartTime: Time,
	m_scriptTime0: Time,
	m_scriptFlags0: Int,
	m_scriptInt0: Int,
	m_curZoomFOV: Float,
	m_targetZoomFOV: Float,
	m_zoomFOVLerpTime: Float,
	m_zoomFOVLerpEndTime: Time,
	m_latestDryfireTime: Time,
	m_requestedAttackEndTime: Time,
	m_currentAltFireAnimIndex: Int,
	m_legendaryModelIndex: Int,
	m_charmModelIndex: Int,
	m_charmAttachment: Int,
	m_charmScriptIndex: Int,
}
```

### Offsets

```
WeaponPlayerData!0x0008 m_moveSpread
WeaponPlayerData!0x000c m_spreadStartTime
WeaponPlayerData!0x0010 m_spreadStartFracHip
WeaponPlayerData!0x0014 m_spreadStartFracADS
WeaponPlayerData!0x0018 m_kickSpreadHipfire
WeaponPlayerData!0x001c m_kickSpreadADS
WeaponPlayerData!0x0020 m_kickTime
WeaponPlayerData!0x0024 m_kickScaleBasePitch
WeaponPlayerData!0x0028 m_kickScaleBaseYaw
WeaponPlayerData!0x002c m_kickPatternScaleBase
WeaponPlayerData!0x0030 m_kickSpringHeatBaseTime
WeaponPlayerData!0x0034 m_kickSpringHeatBaseValue
WeaponPlayerData!0x0038 m_semiAutoTriggerHoldTime
WeaponPlayerData!0x003c m_semiAutoTriggerDown
WeaponPlayerData!0x003d m_pendingTriggerPull
WeaponPlayerData!0x003e m_pendingTriggerUpForCharge
WeaponPlayerData!0x003f m_semiAutoNeedsRechamber
WeaponPlayerData!0x0040 m_pendingReloadAttempt
WeaponPlayerData!0x0041 m_offhandHybridNormalMode
WeaponPlayerData!0x0042 m_pendingoffhandHybridToss
WeaponPlayerData!0x0043 m_fastHolster
WeaponPlayerData!0x0044 m_didFirstDeploy
WeaponPlayerData!0x0045 m_shouldCatch
WeaponPlayerData!0x0046 m_clipModelIsHidden
WeaponPlayerData!0x0047 m_segmentedReloadEndSeqRequired
WeaponPlayerData!0x0048 m_reloadStartedEmpty
WeaponPlayerData!0x0049 m_segmentedAnimStartedOneHanded
WeaponPlayerData!0x004a m_segmentedReloadCanRestartLoop
WeaponPlayerData!0x004b m_segmentedReloadLoopFireLocked
WeaponPlayerData!0x004c m_realtimeModCmds
WeaponPlayerData!0x0054 m_realtimeModCmdHead
WeaponPlayerData!0x0055 m_realtimeModCmdCount
WeaponPlayerData!0x0058 m_customActivityAttachedModelIndex
WeaponPlayerData!0x005c m_customActivityAttachedModelAttachmentIndex
WeaponPlayerData!0x0060 m_fireRateLerp_startTime
WeaponPlayerData!0x0064 m_fireRateLerp_startFraction
WeaponPlayerData!0x0068 m_fireRateLerp_stopTime
WeaponPlayerData!0x006c m_fireRateLerp_stopFraction
WeaponPlayerData!0x0070 m_chargeAnimIndex
WeaponPlayerData!0x0074 m_chargeAnimIndexOld
WeaponPlayerData!0x0094 m_reloadMilestone
WeaponPlayerData!0x0098 m_rechamberMilestone
WeaponPlayerData!0x009c m_cooldownMilestone
WeaponPlayerData!0x00a0 m_prevSeqWeight
WeaponPlayerData!0x00a4 m_fullReloadStartTime
WeaponPlayerData!0x00a8 m_scriptTime0
WeaponPlayerData!0x00ac m_scriptFlags0
WeaponPlayerData!0x00b0 m_scriptInt0
WeaponPlayerData!0x00b4 m_curZoomFOV
WeaponPlayerData!0x00b8 m_targetZoomFOV
WeaponPlayerData!0x00bc m_zoomFOVLerpTime
WeaponPlayerData!0x00c0 m_zoomFOVLerpEndTime
WeaponPlayerData!0x00c4 m_latestDryfireTime
WeaponPlayerData!0x00c8 m_requestedAttackEndTime
WeaponPlayerData!0x00cc m_currentAltFireAnimIndex
WeaponPlayerData!0x00d0 m_legendaryModelIndex
WeaponPlayerData!0x00d4 m_charmModelIndex
WeaponPlayerData!0x00d8 m_charmAttachment
WeaponPlayerData!0x00dc m_charmScriptIndex
```
</details>
<details>
<summary><code>class fogplayerparamsstate_t</code></summary>

```
{
	enable: Bool,
	botAlt: Float,
	topAlt: Float,
	halfDistBot: Float,
	halfDistTop: Float,
	distOffset: Float,
	densityScale: Float,
	halfAngleDeg: Float,
	distColorStr: Float,
	dirColorStr: Float,
	HDRColorScale: Float,
	minFadeTime: Float,
	forceOntoSky: Bool,
	distColor: Color32,
	dirColor: Color32,
	vlParams.color: Vector,
	vlParams.distFalloff: Float,
	vlParams.intensity: Float,
	vlParams.scatter: Float,
	vlParams.inShadowScatter: Float,
	direction: Vector,
	id: Int,
}
```

### Offsets

```
fogplayerparamsstate_t!0x0000 enable
fogplayerparamsstate_t!0x0004 botAlt
fogplayerparamsstate_t!0x0008 topAlt
fogplayerparamsstate_t!0x000c halfDistBot
fogplayerparamsstate_t!0x0010 halfDistTop
fogplayerparamsstate_t!0x0014 distOffset
fogplayerparamsstate_t!0x0018 densityScale
fogplayerparamsstate_t!0x001c halfAngleDeg
fogplayerparamsstate_t!0x0020 distColorStr
fogplayerparamsstate_t!0x0024 dirColorStr
fogplayerparamsstate_t!0x0028 HDRColorScale
fogplayerparamsstate_t!0x002c minFadeTime
fogplayerparamsstate_t!0x0030 forceOntoSky
fogplayerparamsstate_t!0x0031 distColor
fogplayerparamsstate_t!0x0035 dirColor
fogplayerparamsstate_t!0x003c vlParams.color
fogplayerparamsstate_t!0x0048 vlParams.distFalloff
fogplayerparamsstate_t!0x004c vlParams.intensity
fogplayerparamsstate_t!0x0050 vlParams.scatter
fogplayerparamsstate_t!0x0054 vlParams.inShadowScatter
fogplayerparamsstate_t!0x0058 direction
fogplayerparamsstate_t!0x0064 id
```
</details>

## ConVars

<details>
<summary><code></code></summary>

default: `"1.0"`  
flags: `0x0`  
</details>
<details>
<summary><code>Allow_auto_Party</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>BlendBonesMode</code></summary>



default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>CrossPlay_user_optin</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>DigiCertGlobalRoot_usable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>DoorSoundPrefixDouble</code></summary>

Sound prefix for door sounds for double doors

default: `"Door_Single_"`  
flags: `0x2`  
</details>
<details>
<summary><code>DoorSoundPrefixSingle</code></summary>

Sound prefix for door sounds for single doors

default: `"Door_Single_"`  
flags: `0x2`  
</details>
<details>
<summary><code>EADP_RTM_Productid</code></summary>



default: `"r5"`  
flags: `0x2`  
</details>
<details>
<summary><code>EAPD_search_accountname_count</code></summary>

The Number of responce from a name search.

default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>ScriptDisallowedToUsePersistenceOnSP</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ScriptSaveAllowed</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>StreamMicDisabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>TLS_trust_cert</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>TalkIsStream</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>TextDataFromCommunityOnlyInLobby</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>VoiceDataFromCommunityOnlyInLobby</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>VoiceNeedsReset</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>add_to_parent_realms_default</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_titan_grapple_max_len</code></summary>



default: `"3000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_enabled</code></summary>

Enables air slowmo

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_enter_time</code></summary>

Duration it takes to reach full slowmo

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_ground_immediate_end</code></summary>

Controls whether air slowmo fades out after landing or immediately stops

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_leave_time</code></summary>

Duration it takes to leave full slowmo

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_scripted_speed</code></summary>



default: `"0.8"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_when_hovering</code></summary>

Replaces hovering with air slowmo

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>animEvent_debug</code></summary>

1 = sparse, 2 = verbose

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>animEvent_debugEnt</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>animEvent_debug_cl</code></summary>

1 = sparse, 2 = verbose

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>anim_estimateVelocity</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_playerMovementAngleMargin</code></summary>



default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_player_ragdoll_fix</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_print_transition_overflow</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_runGestureAnimEventsToCompletionOnReset_client</code></summary>



default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>anim_showstate</code></summary>

Show the (client) animation state for the specified entity (-1 for none).

default: `"-1"`  
flags: `0x6002`  
</details>
<details>
<summary><code>anim_showstatelog</code></summary>

1 to output anim_showstate to Msg(). 2 to store in AnimState.log. 3 for both.

default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>anim_transitionsequences</code></summary>

Enables blended transitions between sequences.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_view_entity_third_person_camera_use_move_parent</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>announcement</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>announcementImage</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>announcementVersion</code></summary>



default: `"0"`  
flags: `0x12`  
</details>
<details>
<summary><code>assetdownloads_desiredState</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>assetdownloads_enabled</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>assetdownloads_hostname</code></summary>



default: `"r5-assets.stryder.respawn.com"`  
flags: `0x2`  
</details>
<details>
<summary><code>async_serialize</code></summary>

Force async reads to serialize for profiling

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>automantle_backoff_anim_maxfrac</code></summary>

Fraction of mantle after which pulling back simply aborts the mantle

default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_cooldown</code></summary>

Minimum time between mantles

default: `".25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_dangle_required_space</code></summary>

Required space under the ledge to dangle

default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_debug</code></summary>

Debugs player auto-mantle behavior

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_disable_hang</code></summary>

disables the hang mantle behavior

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_enable</code></summary>

Enables player auto-mantle behavior

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_forwarddist</code></summary>

Distance forward to do the ground check from when auto-mantling

default: `"26.f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_gun_enable_height</code></summary>

Eye height above ledge at which gun is reenabled

default: `"33"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_height_above</code></summary>

Mantle height above ledge below which the "above" animation is used and above which the "high" animation is used

default: `"30"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_height_below</code></summary>

Mantle height above ledge below which the "below" animation is used

default: `"-10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_height_level</code></summary>

Mantle height above ledge below which the "level" animation is used

default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_jumpoff_anim_maxfrac</code></summary>

Maximum fraction of mantle at which jump off animation is played

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_jumpoff_duration</code></summary>

Duration of jump off animation when jumping off

default: `"0.1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_max_frac</code></summary>

Fractional amount (0-1) player can move forward without hitting jump.

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_maxangle_push</code></summary>

Max angle the player can be pushing from the wall normal to auto-mantle

default: `"50"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_maxangle_view</code></summary>

Max angle the player can be facing from the wall to auto-mantle

default: `"50"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_min_frac</code></summary>

Fractional amount (0-1) player can move backward without hitting jump.

default: `"0.1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_mindist</code></summary>

Minimum forward distance when auto-mantling

default: `"18.f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_rest_frac</code></summary>

Fractional amount (0-1) player will tend toward when no input is given.

default: `"0.4"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_rest_frac_below</code></summary>

Replaces rest_frac when using the "below" animation

default: `"0.3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_searchdist</code></summary>

Forward distance within which to look for a ledge to auto-mantle

default: `"5.f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_view_correction_speed</code></summary>

Speed at which view direction is clamped when mantling

default: `"180"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_view_high_yaw_max</code></summary>

Max view yaw when mantling with the "high" mantle animation

default: `"90"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_view_pitch_max</code></summary>

Max view pitch when mantling

default: `"35"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_view_pitch_min</code></summary>

Min view pitch when mantling

default: `"-80"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_view_yaw_max</code></summary>

Max view yaw when mantling

default: `"60"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_wallrun_maxangle_view</code></summary>

Max angle the player can be facing from the wall to auto-mantle while wall running

default: `"45"`  
flags: `0x2002`  
</details>
<details>
<summary><code>baseanimatingoverlay_playbackRateThreshold</code></summary>



default: `"0.05"`  
flags: `0x2`  
</details>
<details>
<summary><code>baselines_print</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>bhit_enable</code></summary>

Enables bhit commands from the client

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bhit_reliable</code></summary>

Makes bhit commands reliable messages

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bink_materials_enabled</code></summary>

Allows materials with 'Emissive Uses Video' checked to play video on the material 

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>bink_preload_videopanel_movies</code></summary>

Preload Bink movies used by VideoPanel.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>boost_jetwash_prediction_factor</code></summary>

Factor used to scale player's velocity when finding jetwash trace point.

default: `"20.0f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bot_lagOut</code></summary>

Cause bots to lag out

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_animatingEntities</code></summary>



default: `"5000"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_animationOverlayEntities</code></summary>



default: `"260"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_combatCharEntities</code></summary>



default: `"200"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_weaponEntities</code></summary>



default: `"1200"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_ziplineEntities</code></summary>



default: `"250"`  
flags: `0x2`  
</details>
<details>
<summary><code>bug_reproNum</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>buildcubemaps_async</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>buildcubemaps_index</code></summary>



default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>buildcubemaps_pvs_start_early</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>buildcubemaps_single_step</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>building_cubemaps</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>bulletPredictionDebug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bullet_trace_test_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bullet_trace_test_enable</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>c_dropship_ground_fx_dist_interval</code></summary>



default: `"256"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_ground_fx_time_interval</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_rope_debug</code></summary>

 Used to visualize the drop ship rope interaction.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_rope_events</code></summary>

Turn on client side drop ship rope interaction detection.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_rope_magnitude</code></summary>

Used to scale the interaction of a drop ship and a rope.

default: `"128"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_rope_range</code></summary>

Max distance away from a drop ship that a Rope is effected.

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_maxdistance</code></summary>



default: `"400"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_maxpitch</code></summary>



default: `"90"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_maxyaw</code></summary>



default: `"135"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_mindistance</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_minpitch</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_minyaw</code></summary>



default: `"-135"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_orthoheight</code></summary>



default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_orthowidth</code></summary>



default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdistADS_110</code></summary>



default: `"35.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdistADS_70</code></summary>



default: `"50.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdistADS_90</code></summary>



default: `"40.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdist_110</code></summary>



default: `"60.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdist_70</code></summary>



default: `"100.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdist_90</code></summary>



default: `"75.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderdist</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshouldergetsviewpunch</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderheight</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderoffset</code></summary>



default: `"17.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_threadedAnimPostData</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_collision</code></summary>

When in thirdperson and cam_collision is set to 1, an attempt is made to keep the camera from passing though walls.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_idealdelta</code></summary>

Controls the speed when matching offset to ideal angles in thirdperson view

default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_idealdist</code></summary>



default: `"150"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_ideallag</code></summary>

Amount of lag used when matching offset to ideal angles in thirdperson view

default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_idealpitch</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_idealyaw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchLock_feetRelative</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_on</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_period</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_phase</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_pitchBase</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_pitchRange</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_pitchWiggleRoom</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_player_viewheight_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_showangles</code></summary>

When in thirdperson, print viewangles/idealangles/cameraoffsets to the console.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_captiontrace</code></summary>

Show missing closecaptions (0 = no, 1 = devconsole, 2 = show in hud)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_global_norepeat</code></summary>

How often a caption can repeat, unless overriden by norepeat. (or 0)

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_linger_time</code></summary>

Close caption linger time in seconds.

default: `"1.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cc_max_duration</code></summary>

The max duration in seconds for a closed caption if event doesn't stop playing.

default: `"30.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_minvisibleitems</code></summary>

Minimum number of caption items to show.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_predisplay_time</code></summary>

Close caption delay in seconds before showing caption.

default: `"0.25"`  
flags: `0x80`  
</details>
<details>
<summary><code>cc_rui</code></summary>

Use RUI to draw closecaption text.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_text_size</code></summary>

Changes the size of subtitles and closed captions text. 0 = normal, 1 = large, 2 = huge.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>cc_timeshift_norepeat</code></summary>

How often a caption can repeat, unless overriden by norepeat. (timeshift only) (or 0)

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>chasecam_distanceMax_override</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chat_rules_log_details</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_console_ptt</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_doRealNameLookups</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_min_status_send_interval</code></summary>



default: `"16"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_nameLength</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_namePaddingX</code></summary>



default: `"12"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_nameWidth</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_onlyWhenActive</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_sendServerMutes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_useSlopSpace</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_voiceMode</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_voiceMode</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cheap_captions_fadetime</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cheap_captions_test</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chroma_enable</code></summary>



default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_NotifyAllLevelAssetsLoaded_endframe</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_RunClientConnectScripts_Before_ProcessOnDataChangedEvents</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_SetupAllBones</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ShowBoneSetupEnts</code></summary>

Show which entities are having their bones setup each frame.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_adjustTimeEntsPerJob</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_aggregate_particles</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_allowABSCalculationDuringSnapshotScriptCalls</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_allowABSDuringSnapshotScriptCalls</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_allowAnimsToInterpolateBackward</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_always_draw_3p_player</code></summary>

Always draw the 3p player model, even when in first-person view

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_always_ragdoll_radius</code></summary>

Always create client ragdoll if within this distance to viewer

default: `"500"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anglespeedkey</code></summary>



default: `"0.67"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_blend_transition_dist</code></summary>



default: `"2500"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_detail_dist</code></summary>



default: `"1500"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_face_dist</code></summary>



default: `"250"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_sequence_transition_full_weight_optimization</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_sounds_seek</code></summary>



default: `"1"`  
flags: `0xa`  
</details>
<details>
<summary><code>cl_approx_footstep_origin</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_approx_tracer_origin</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_async_bone_setup</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_base_entity_effect_lock</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_bones_incremental_blend</code></summary>

Don't reblend bones which we don't need to in SetupBones.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_bones_incremental_transform</code></summary>

Don't retransform bones which we don't need to in SetupBones.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_bones_oldhack</code></summary>

Redo all previously transformed bones in SetupBones--old 'hack'.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_bounds_show_errors</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_burninggibs</code></summary>

A burning player that gibs has burning gibs.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_clock_correction</code></summary>

Enable/disable clock correction on the client.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_ahead_correct_interval</code></summary>

Minimum interval over which the clock will try to correct to ideal when it's ahead

default: `"20"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_behind_correct_interval</code></summary>

Interval over which the clock will try to correct to ideal when it's behind

default: `"200"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_force_server_tick</code></summary>

Force clock correction to match the server tick + this offset (-999 disables it).

default: `"999"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_cmdbackup</code></summary>

Number of redundant usercmds to send, to cover client->server packet loss

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_cmdrate</code></summary>

Max number of command packets sent to server per second

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_configversion</code></summary>

Configuration layout version.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_configversion_dummy</code></summary>

Configuration layout version dummy.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_cull_weapon_fx</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_dataBlockFragmentPL</code></summary>



default: `"0.0"`  
flags: `0x2`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>cl_deathhints_enabled</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>cl_debugClientEntities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_debug_deferred_trace</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_debug_deferred_trace_overlay</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_debug_model_fx_sounds</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_decal_alwayswhite</code></summary>

Force FX decals to white (1), or white full alpha (2).

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_decal_backoff</code></summary>

Amount to back off FX decal trace by.

default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_deferred_effects</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_deferred_trace_normal_priority</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_demoviewoverride</code></summary>

Override view during demo playback

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_disable_ragdolls</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_disable_splitscreen_cpu_level_cfgs_in_pip</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_disconnectOnTooManySnapshotFrames</code></summary>

Disconnect when the client gets too many snapshot messages from the server without the server getting any messages from the client.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_doNetworkAsserts</code></summary>

Turn off to disable some client asserts that fail rarely, presumably due to network bugs.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_doRecreateEnts</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_draw_player_model</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_drawhud</code></summary>

Enable the rendering of the hud

default: `"1"`  
flags: `0x40004000`  
</details>
<details>
<summary><code>cl_drawmonitors</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ejectbrass</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_enable_remote_splitscreen</code></summary>

Allows viewing of nonlocal players in a split screen fashion

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_entCreateDeleteDebug</code></summary>

If true, print out when we create or delete an entity on the client

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_events_ignore_invalidate</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_failremoteconnections</code></summary>

Force connection attempts to time out

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_fasttempentcollision</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_flip_vis_bits</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_flushentitypacket</code></summary>

For debugging. Force the engine to flush an entity packet.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_footstep_event_max_dist</code></summary>



default: `"2500"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_footstep_event_max_dist_titan</code></summary>



default: `"4000"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_forceAdjustTime</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_fovScale</code></summary>



default: `"1.27216005"`  
flags: `0x41000200`  
min value: `1`  
max value: `1.7`  
</details>
<details>
<summary><code>cl_gib_allow</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>cl_gib_attack_dir_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_gib_lifetime</code></summary>



default: `"3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_idealpitchscale</code></summary>

0 to turn off. 0.8 is a good starting value

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ignorepackets</code></summary>

Force client to ignore packets (for debugging).

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_interp_all</code></summary>

Disable interpolation list optimizations.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_interpolate</code></summary>

Interpolate entities on the client.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_interpolate</code></summary>

Interpolate entities on the client.

default: `"1.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_interpolateSoAllAnimsLoop</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_interpolation_before_prediction</code></summary>

Interpolate entities before doing prediction

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_isUnderAge</code></summary>



default: `"0"`  
flags: `0x80000200`  
</details>
<details>
<summary><code>cl_is_softened_locale</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_jiggle_bone_debug</code></summary>

Display physics-based 'jiggle bone' debugging information

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_jiggle_bone_debug_pitch_constraints</code></summary>

Display physics-based 'jiggle bone' debugging information

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_jiggle_bone_debug_yaw_constraints</code></summary>

Display physics-based 'jiggle bone' debugging information

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_jiggle_bone_invert</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_jiggle_bone_sanity</code></summary>

Prevent jiggle bones from pointing directly away from their target in case of numerical instability.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_keepPersistentDataOnDisconnect</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_lagcompensation</code></summary>

Perform server side lag compensation of weapon firing events.

default: `"1"`  
flags: `0x200`  
</details>
<details>
<summary><code>cl_language</code></summary>

Language

default: `"english"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_leafsystemvis</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_lerpIfChildrenLerp</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_loadBspFromServerInfo</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_loadPostProcessShadersEarly</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_loadStaticPropsInJob</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_matchmaking_timeout</code></summary>

Total time allowed for the client to resend the 'connect' attempt when matchmaking

default: `"1"`  
flags: `0x80000`  
min value: `0.5`  
max value: `20000`  
</details>
<details>
<summary><code>cl_model_fx_gib_cull_front_dist</code></summary>



default: `"3000"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_model_fx_gib_cull_radius</code></summary>



default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_mouseenable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_move_use_dt</code></summary>

Use the actual delta time for motion instead some super complicated system based on the server frame rate.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_noTimeoutLocalHost</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_overrideEventTimes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_parallelParticlePreDrawWork</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_parallel_clientside_animations</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_batch_mode</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_fallback_base</code></summary>

Base for falling back to cheaper effects under load.

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>cl_particle_fallback_multiplier</code></summary>

Multiplier for falling back to cheaper effects under load.

default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>cl_particle_limiter_display_killed</code></summary>

Display a red box around killed fx.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_limiter_hide_killable</code></summary>

Hide fx than could be killed if over limit.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>cl_particle_limiter_max_particle_count</code></summary>

Limit the total number of active particles. 0 to not limit.

default: `"10000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_limiter_max_system_count</code></summary>

Limit the total number of active particle systems. 0 to not limit.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_limiter_min_kill_distance</code></summary>

Only kill fx that are further than this distance from the player.

default: `"4000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_limiter_overlay</code></summary>

Display particle limiter infos.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_max_count</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_sim_fallback_base_multiplier</code></summary>

How aggressive the switch to fallbacks will be depending on how far over the cl_particle_sim_fallback_threshold_ms the sim time is.  Higher numbers are more aggressive.

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_sim_fallback_threshold_ms</code></summary>

Amount of simulation time that can elapse before new systems start falling back to cheaper versions

default: `"6.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_snoozetime</code></summary>

Particle snooze time in seconds (0 is off)

default: `"0.166667"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particles_show_bbox</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particles_show_bbox_name</code></summary>

show the bounding box of only particles with this name

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particles_show_controlpoints</code></summary>

1 to show parent effects, 2 shows all children effects too

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_pclass</code></summary>

Dump entity by prediction classname.

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_pdump</code></summary>

Dump info about this entity to screen.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_phys_maxticks</code></summary>

Sets the max number of physics ticks allowed for client-side physics (ragdolls)

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_phys_show_active</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_phys_timescale</code></summary>

Sets the scale of time for client-side physics (ragdolls)

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_physics_invalidate_ents</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_physics_maxvelocity</code></summary>

Max velocity of a vphysics object on the client

default: `"4000.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_physicsshadowupdate_render</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_pitchspeed</code></summary>



default: `"225"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_playback_screenshots</code></summary>

Allows the client to playback screenshot and jpeg commands in demos.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_player_fullupdate_predicted_origin_fix</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_player_resource_always_connected</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_player_touch_triggers</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_postSnapshotTransitionBlockCount</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_preSnapshotTransitionBlockCount</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_pred_error_verbose</code></summary>

Show more field info when spewing prediction errors.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_pred_optimize</code></summary>

Optimize for not rerunning prediction if there was no difference between what we predicted and the incoming networked state

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_predict</code></summary>

Perform client side prediction.

default: `"1"`  
flags: `0x200`  
</details>
<details>
<summary><code>cl_predict_basetoggles</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_predict_cmdlimit</code></summary>

Artificially limits the number of remembered commands that can be used for prediction

default: `"750"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_error_icon_duration</code></summary>

Duration for prediction error icon to stay visible

default: `"0.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_error_icon_show</code></summary>

Whether to show the prediction error icon

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_error_icon_threshold_angle</code></summary>

Angle error required to show prediction error icon

default: `"0.01"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_error_icon_threshold_dist</code></summary>

Distance error required to show prediction error icon

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_motioncontrol</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_predict_viewangles</code></summary>

Predict view angles even if cl_predict is 0.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_prediction_error_timestamps</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_predictionlist</code></summary>

Show which entities are predicting


default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predictweapons</code></summary>

Perform client side prediction of weapon effects.

default: `"1"`  
flags: `0x200`  
</details>
<details>
<summary><code>cl_prevent_weapon_text_hints</code></summary>

stops weapon text hints from appearing

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_force_fade_time</code></summary>

Fade out ragdoll even if in players view after this many seconds

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_force_fade_time_local_view_player</code></summary>

If the ragdoll is of the local view player then use the max of this and cl_ragdoll_force_fade_time for the fade time

default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_force_fade_time_on_moving_geo</code></summary>

Fade out ragdoll even if in players view after this many seconds when touching moving geo.

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_force_fade_time_titan</code></summary>

Fade out titan ragdoll even if in players view after this many seconds

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_maxcount</code></summary>



default: `"8"`  
flags: `0x40000000`  
min value: `0`  
max value: `8`  
</details>
<details>
<summary><code>cl_ragdoll_self_collision</code></summary>



default: `"1"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>cl_replayDelayTolerance</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_requireAnimForAnimEventsHdr</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_resend</code></summary>

Delay in seconds before the client will resend the 'connect' attempt

default: `"0.5"`  
flags: `0x80000`  
min value: `0.5`  
max value: `20`  
</details>
<details>
<summary><code>cl_resend_timeout</code></summary>

Total time allowed for the client to resend the 'connect' attempt

default: `"10"`  
flags: `0x80000`  
min value: `0.5`  
max value: `20000`  
</details>
<details>
<summary><code>cl_retire_low_priority_lights</code></summary>

Low priority dlights are replaced by high priority ones

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_runWeaponCloneThinkWhenHidden</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_safearea</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>cl_screenshotname</code></summary>

Custom Screenshot name

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_scriptCompileAsync</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_script_perf_dump_on_shutdown</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_shadowupdatespacing</code></summary>



default: `"10.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showClanTags</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_show_splashes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showerror</code></summary>

Show prediction errors, 2 for above plus detailed field deltas.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showerror_watchfield</code></summary>

When showing prediction errors, only show fields that match this name

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showfiredbullets</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showfps</code></summary>

Draw fps meter (1 = fps, 2 = smooth, 3 = server, 4 = Show+LogToFile, +10 = detailed )

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_showfps_altframetime</code></summary>

Use the showfps_enabled time instead of the old cl_showfps time.

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_showpausedimage</code></summary>

Show the 'Paused' image when game is paused.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showpos</code></summary>

Draw current position at top of screen

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_showsounds</code></summary>

Print server to client networked sounds to the console

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_showtime</code></summary>

Draw current demo time if recording a demo

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_simulateAllModelsRegardless</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_simulationtimefix</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_skipAnimEventsOnProps</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_skipfastpath</code></summary>

Set to 1 to stop all models that go through the model fast path from rendering

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_smooth</code></summary>

Smooth view/eye origin after prediction errors

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_smooth_debug</code></summary>

Show prediction errors that are being smoothed

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_smoothtime</code></summary>

Smooth client's view after prediction error over this many seconds

default: `"0.25"`  
flags: `0x2`  
min value: `0.01`  
max value: `2`  
</details>
<details>
<summary><code>cl_threaded_bone_setup</code></summary>

Enable parallel processing of C_BaseAnimating::SetupBones()

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_update_visibility_for_non_predicted_local_player_always</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_updatedirty_async</code></summary>

Call UpdateDirtySpatialPartitionEntities on a worker thread.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_updatedirty_early</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_updaterate_mp</code></summary>

Number of packets per second of updates you are requesting from the server in mp

default: `"20"`  
flags: `0x10202`  
</details>
<details>
<summary><code>cl_upspeed</code></summary>



default: `"320"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_useFutureSnapForEvents</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_useLobbyTypeForChatroom</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_view_cone</code></summary>

Enable clamping view to animated/scripted viewcone

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_view_cone_debug</code></summary>

Show view cone debugging window

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_viewmodel_pre_animate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_warnAboutSoundsOnInvalidEntities</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_yawspeed</code></summary>



default: `"210"`  
flags: `0x2`  
</details>
<details>
<summary><code>clampHostFrameTimeToOneTick_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>clearOnAnimChange</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>client_deferredSnapshotScriptCalls</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>clientport</code></summary>

Host game client port

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cloak_enabled</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cloak_pilotNoiseFactor</code></summary>

Intensity of noise in pilot cloak aberration

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>cloak_pilotTint1</code></summary>

Brightness factor for center-left sample

default: `"0.35"`  
flags: `0x2002`  
</details>
<details>
<summary><code>cloak_pilotTint2</code></summary>

Brightness factor for upper-right sample

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>cloak_pilotTint3</code></summary>

Brightness factor for lower-right sample

default: `"0.65"`  
flags: `0x2002`  
</details>
<details>
<summary><code>clock_bias_mp</code></summary>



default: `"-18.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>clock_bias_sp</code></summary>



default: `"-2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>clock_showcorrections</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>clock_showdebuginfo</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>closecaption</code></summary>

Enable close captioning. 1 = dialogue only, 2 = dialogue and sound effects.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>cockpitDrift_scalePitch</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitDrift_scaleYaw</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitDrift_speedPitch</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitDrift_speedYaw</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitShake_sourceRollRange</code></summary>

The range of weapon kick roll that will be sampled for cockpit shake.

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitShake_translateRange</code></summary>

Max amount of cockpit shake.

default: `"0.6"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_damage_chroma_scale</code></summary>



default: `"0.4"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_hit_chroma_max_time</code></summary>

Time to get rid of the most recent hit_chroma adjustment when at near 0 health.

default: `"0.6"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_hit_chroma_scale</code></summary>



default: `"0.2"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_pitch_down_frac</code></summary>

fractional amount that cockpit pitches as you look down

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_pitch_up_frac</code></summary>

fractional amount that cockpit pitches as you look up

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_chroma_scale</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_bottom</code></summary>



default: `"1.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_left</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_mid</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_right</code></summary>



default: `"0.75"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_top</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>coll_spatial_entry_limit_client</code></summary>

How many entries are used in the spatial acceleration structure for dynamic entities on the client.

default: `"140"`  
flags: `0x2`  
</details>
<details>
<summary><code>coll_spatial_optimize_prefetch</code></summary>

Prefetch memory into the cache before optimizing spatial acceleration trees. This does more work, but tends to be faster overall.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>coll_use_bolt_size</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>colorblind_mode</code></summary>



default: `"0"`  
flags: `0x41000000`  
min value: `0`  
max value: `3`  
</details>
<details>
<summary><code>communities_doRealNameLookupsForCommunityCreators</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>communities_enabled</code></summary>

Enable communities

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>communities_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>community</code></summary>

Our current community

default: `""`  
flags: `0x200`  
</details>
<details>
<summary><code>community_abortCommunitySettingsTime</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_abortUserInfoTime</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_browse_excludeMine</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_clantags</code></summary>

put community name in the clan tag

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_doRealNameLookupsForInbox</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_frame_run</code></summary>

Communities should run it's frame update.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_queryServerWhenOrphaned</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_replaceInboxTokens</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_replaceInboxTokens</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_resolveNames</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_resolveNames</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_send_server_voice</code></summary>

Communities will route voice data to the chat server!

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_spam</code></summary>

Whether communities should spam to the console log

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_staleCommunitySettingsTime</code></summary>



default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_staleUserInfoTime</code></summary>



default: `"120"`  
flags: `0x2`  
</details>
<details>
<summary><code>con_logfile</code></summary>

Console output gets written to this file

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>con_timestamp</code></summary>

Prefix console.log entries with timestamps

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cpu_level</code></summary>

CPU Level - Default: High

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>cpu_level</code></summary>

CPU Level - Default: High

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>createentitydecals</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>crossPlay_Enabled</code></summary>

Allow crossPlay code to work!

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm0_on_worker</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_cascade_res</code></summary>

Set the cascading shadow maps rendertarget resolution

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_cascade_res</code></summary>

Set the cascading shadow maps rendertarget resolution

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_coverage</code></summary>

Set the cascading shadow maps coverage

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_culling_use_base_planes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_culling_use_exclusion_planes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_culling_use_inclusion_planes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_culling_use_planes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_debug_2d</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_debug_culling</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_debug_vis_hi_range</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_debug_vis_lo_range</code></summary>



default: `".35"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_depth_bias</code></summary>



default: `"-0.000005f"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_dropsequence_adjusted_coverage</code></summary>

Coverage for csm_dropsequence_adjustment

default: `"6400"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_dropsequence_adjustment</code></summary>

Adjust CSM 2 coverage during drop sequence for STATICSHADOWMODE_GENERATE_ONCE in order to prevent drop ship shadow from being clamped.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_enabled</code></summary>

Set whether to render cascading shadow maps

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_fadeModels</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_force_no_csm_in_reflections</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_frustum_draw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_frustum_draw_lock</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_ignore_cascade12</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_ignore_edge_planes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_ignore_face_planes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_max_z_offset</code></summary>

Note csm_z_cover_world expands Z range as well

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_min_z_offset</code></summary>

Note csm_z_cover_world expands Z range as well

default: `"-1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_renderable_shadows</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_rope_shadows</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_rot_override</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_rot_x</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_rot_y</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"43"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_shadow_split_lerp_factor_range</code></summary>



default: `".1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_texel_size_cascade_0</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_texel_size_cascade_1</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_texel_size_cascade_2</code></summary>



default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_texel_size_cascade_onecascade</code></summary>



default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_use_env_light_direction</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_world_shadow_meshes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_world_shadows</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_z_cover_world</code></summary>

Expands CSM Depth coverage. 1 - Sea Height to Jump Height by Script, 2 - Static shadow's depth range

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>curl_allowHTTPS</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>curl_preloadDlls</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>curl_spamAllQueryStates</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cursorWide</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>damageIndicatorReplayTimeOffset</code></summary>

Artificial delay of damage indicator in replay

default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>damage_indicator_style_pilot</code></summary>



default: `"2"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>damageinfo_defendInvalidValues</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>debugFootstepEffects</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_debug_overlay</code></summary>

Enable debug of the debug overlays

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>debug_force_textRestriction</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_force_ugcRestriction</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_force_voiceRestriction</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_map_crc</code></summary>

Prints CRC for each map lump loaded

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>decal_clip_debug_draw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>decal_clip_debug_groups</code></summary>

this kicks off this many work groups when a decal is spawned instead of one for each triangle on the model. 0 is disabled

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>defer_weapon_effects</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>delayPostSnapshotNotificationsToAfterInterpolation</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>demo_autoRecord</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>demo_autoRecordName</code></summary>



default: `"demo"`  
flags: `0x2`  
</details>
<details>
<summary><code>demo_connect_string</code></summary>

Connect string for demo UI

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>demo_ui_enable</code></summary>

Suffix for the demo UI

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>devStats</code></summary>

True if game should report dev stats.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>developer</code></summary>

Set developer message level

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>disable_player_use_prompts</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>discord_largeImage</code></summary>



default: `"default"`  
flags: `0x2`  
</details>
<details>
<summary><code>discord_smallImage</code></summary>



default: `"default_small"`  
flags: `0x2`  
</details>
<details>
<summary><code>discord_updatePresence</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dlight_default_falloff</code></summary>

default half-distance fraction for legacy dlights.

default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>dlight_early_clear</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dlight_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dlight_overlay</code></summary>

Draw debug overlay of dlight array

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>dodge_cockpitHack</code></summary>

Hack to avoid eye moving too far back in cockpit

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_cockpitOffsetMax</code></summary>

Cockpit translation while dodging

default: `"3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_cockpitTiltMax</code></summary>

Additional view tilt applied to the cockpit while dodging

default: `"4"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_vertical_enable</code></summary>

Enables vertical dodge

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_vertical_horzspeedscale</code></summary>

Horizontal speed retained when dodging vertically

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_vertical_in_air</code></summary>

Allow dodge to still apply vertical acceleration when player is in the air

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_vertical_threshold</code></summary>

Stick deflection before dodge becomes vertical

default: `"0.1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_viewTiltDecreaseSpeed</code></summary>

Speed at which view tilt decreases while dodging (degrees/sec)

default: `"2.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_viewTiltFalloffTime</code></summary>

Time during which view tilt decays to zero while dodging

default: `".7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_viewTiltIncreaseSpeed</code></summary>

Speed at which view tilt increases while dodging (degrees/sec)

default: `"5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_viewTiltMax</code></summary>

Amount of view tilt while dodging in degrees

default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dof_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_farDepthEnd</code></summary>



default: `"3000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_farDepthStart</code></summary>



default: `"2000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_monitorFarDepthEnd</code></summary>



default: `"3000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_monitorFarDepthStart</code></summary>



default: `"2000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_monitorNearDepthEnd</code></summary>



default: `"7.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_monitorNearDepthStart</code></summary>



default: `"7.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_nearDepthEnd</code></summary>



default: `"7.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_nearDepthStart</code></summary>



default: `"7.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_overrideParams</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_variable_blur</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dormant_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>draw_target_info_offscreen</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchclass</code></summary>

Watch all fields encoded with this table.

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchdecode</code></summary>

When watching show decode.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchencode</code></summary>

When watching show encode.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchent</code></summary>

Watch this entities data table encoding.

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchvar</code></summary>

Watch the named variable.

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>dump_varsights_calculations</code></summary>

Dumps one frame of variable sights calculations and turns itself off.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>durango_voice_chat_team_only</code></summary>

Only turn on voice chat for players on the same team

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dvs_enable</code></summary>

Enable dynamic viewport scaling.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dvs_enable_in_lobby</code></summary>

Paired with dvs_enable. Both have to be true if we want to enable DVS in Lobby.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dvs_gpuframetime_max</code></summary>

GPU frametime threshold above which DVS will start decreasing the scale. Specified in microseconds.

default: `"16500"`  
flags: `0x2`  
</details>
<details>
<summary><code>dvs_gpuframetime_min</code></summary>

GPU frametime threshold below which DVS will start increasing the scale. Specified in microseconds.

default: `"15000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dvs_scale_min</code></summary>

Smallest scale the viewport dimensions can be scaled by.

default: `"0.5f"`  
flags: `0x2`  
min value: `0.01`  
max value: `1`  
</details>
<details>
<summary><code>eadpAuth_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>eadpFriends_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>eadpGroups_Enabled</code></summary>

Allow EADP Groups to run

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>eadpGroups_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>eadpRtm_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>eadpSearch_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>eadp_rtm_spam_log</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>effect_update_array_spam</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>enable_KVFileOverrides</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>enable_debug_overlays</code></summary>

Enable rendering of debug overlays

default: `"1"`  
flags: `0x4004`  
</details>
<details>
<summary><code>enable_height_based_land_anims</code></summary>

Enables different land animations based on the height of the fall. These may just be duplicates of each other.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>enable_height_based_land_anims_titans</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>enable_skeleton_draw</code></summary>

Render skeletons in wireframe

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>encrypt_multiKey</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_lightweightEnts</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_repack_almostFull</code></summary>



default: `"3000"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_repack_threshhold</code></summary>



default: `"0.0001"`  
flags: `0x2`  
</details>
<details>
<summary><code>entity_error_on_hitbox_count_mismatch</code></summary>

If set to true, SetModel will trigger a script error if any hitbox attachments will become invalid.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>entity_skipRedundantAddEffects</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>entity_useNetworkFieldBuffer</code></summary>



default: `"1"`  
flags: `0x400002`  
</details>
<details>
<summary><code>error_if_non_standard_ent_create</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>eula_version</code></summary>

What the current version of the EULA is

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>eula_version_accepted</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>eventseq_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>everything_unlocked</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fast_intro</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fatal_script_error_prompt</code></summary>



default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>fatal_script_errors</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fatal_script_errors_client</code></summary>

Enable fatal errors for client script. -1 will revert to using "fatal_script_errors"

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fatal_script_errors_server</code></summary>

Enable fatal errors for server script. -1 will revert to using "fatal_script_errors"

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>filesystem_buffer_size</code></summary>

Size of per file buffers. 0 for none

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_max_stdio_read</code></summary>



default: `"16"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_native</code></summary>

Use native FS or STDIO

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_report_buffered_io</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_unbuffered_io</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_use_overlapped_io</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>fire_animevents_overlay_not_active</code></summary>

fires anim events even if the overlay isn't active

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>first_person_bullet_delay</code></summary>

Set the amount of additional delay for first person bullets fired with net_optimize_weapons in seconds. Required so bullets match animations with cl_predict 0 and in kill replay

default: `"0.1f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>first_person_proxy_blend_distance</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>first_person_proxy_debug</code></summary>



default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>firsttime_mp_message</code></summary>

first time joining multiplayer

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>fog_enable</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"1"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>fog_enable_water_fog</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_enableskybox</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>force3PLaserAttachment</code></summary>



default: `"HEADFOCUS"`  
flags: `0x2002`  
</details>
<details>
<summary><code>force_EAAccess</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>fps_max</code></summary>

Frame rate limiter. -1 indicates use the desktop refresh. 0 is unlocked.

default: `"-1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>fps_max_use_refresh</code></summary>

Use refresh rate for fps_max.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fps_max_vsync</code></summary>

Frame rate limiter with vsync is enabled.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>freecam_swallowButtonInput</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>freefall_sound_autoplay_time</code></summary>

If the player falls for longer than this amount of time freefall sounds will automatically start playing.

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>freefall_sound_height</code></summary>

Height player must be falling from to trigger freefall sound effects.

default: `"200.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>friends_onlineUpdateInterval</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_intralevel_reads</code></summary>

Internal var to tell the file system that we are in an intraread state...

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_monitor_read_from_pack</code></summary>

0:Off, 1:Any, 2:Sync only

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_intra_level_readopens</code></summary>

0:Off, 1:NotAudio, 2:All

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_long_reads</code></summary>

0:Off, 1:All (for tracking accumulated duplicate read times), >1:Microsecond threshold

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_sync_opens</code></summary>

0:Off, 1:Always, 2:Not during map load

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_sync_opens_callstack</code></summary>

0 to not display the call-stack when we hit a fs_report_sync_opens warning. Set to 1 to display the call-stack.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_sync_opens_fatal</code></summary>



default: `"0"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>fs_showAllReads</code></summary>

0:Off, 1:On

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_vpk_file_open</code></summary>

0: No reporting, 1: Patch:VPKFilePath, 2: Patch:VPKFilePath:PartialPath

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_warning_mode</code></summary>

0:Off, 1:Warn main thread, 2:Warn other threads

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>func_break_max_pieces</code></summary>



default: `"15"`  
flags: `0x2080`  
</details>
<details>
<summary><code>fx_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fx_deferWorldTraceConstraint</code></summary>

'Collision via traces' ops using collision mode 0 use deferred traces.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_glass_velocity_cap</code></summary>

Maximum downwards speed of shattered glass particles

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_impact_ally</code></summary>



default: `"0.49 0.76 1.0 1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_impact_enemy</code></summary>



default: `"1.0 0.47 0.13 1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_impact_neutral</code></summary>



default: `"0.86 0.86 0.86 1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_screenspacepass</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>g_debug_ragdoll_removal</code></summary>



default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>g_ragdoll_fadespeed</code></summary>



default: `"600"`  
flags: `0x2`  
</details>
<details>
<summary><code>g_ragdoll_important_maxcount</code></summary>



default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>g_ragdoll_lvfadespeed</code></summary>



default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>gameCursor_ModeActive</code></summary>

Globally activates/deactivates game cursor mode

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gameCursor_Velocity</code></summary>

Game cursor velocity under joystick control

default: `"1300.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_0</code></summary>

Gamepad ads sensitivity for 1x scopes / ironsights.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_1</code></summary>

Gamepad ads sensitivity for 2x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_2</code></summary>

Gamepad ads sensitivity for 3x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_3</code></summary>

Gamepad ads sensitivity for 4x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_4</code></summary>

Gamepad ads sensitivity for 6x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_5</code></summary>

Gamepad ads sensitivity for 8x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_6</code></summary>

Gamepad ads sensitivity for 10x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_7</code></summary>

Gamepad ads sensitivity for an unused scope.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_aim_assist_ads_high_power_scopes</code></summary>

Gamepad uses aim assist in ADS with high powered scopes

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_ads_low_power_scopes</code></summary>

Gamepad uses aim assist in ADS with low powered scopes

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_hip_high_power_scopes</code></summary>

Gamepad uses aim assist in Hip with high powered scopes

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_hip_low_power_scopes</code></summary>

Gamepad uses aim assist in Hip with low powered scopes

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_melee</code></summary>

Gamepad uses aim assist with melee weapons

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_speed</code></summary>



default: `"2"`  
flags: `0x1000000`  
min value: `0`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_0</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_1</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_2</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_3</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_4</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_5</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_6</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_7</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_button_layout</code></summary>

Gamepad button layout (used by menus)

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_buttons_are_southpaw</code></summary>

Gamepad button layouts should use southpaw variants (used by menus)

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_pitch</code></summary>



default: `"75.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_turn_delay</code></summary>



default: `"0.25"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_turn_pitch</code></summary>



default: `"30.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_turn_time</code></summary>



default: `"1.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_turn_yaw</code></summary>



default: `"30.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_yaw</code></summary>



default: `"110.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_assist_on</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_curve</code></summary>



default: `"10.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_deadzone_in</code></summary>



default: `"0.15"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_deadzone_out</code></summary>



default: `"0.02"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_pitch</code></summary>



default: `"120.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_turn_delay</code></summary>



default: `"0.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_turn_pitch</code></summary>



default: `"0.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_turn_time</code></summary>



default: `"0.33"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_turn_yaw</code></summary>



default: `"220.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_yaw</code></summary>



default: `"160.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_pilot</code></summary>



default: `"0,1,2,3,4,5,6,7,8,9,10,11,12,13"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_titan</code></summary>



default: `"0,1,2,3,4,5,6,7,8,9,10,11,12,13"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_deadzone_index_look</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_deadzone_index_move</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_enabled</code></summary>

True if the gamepad is enabled, false otherwise.

default: `"1"`  
flags: `0x2`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>gamepad_look_curve</code></summary>



default: `"0"`  
flags: `0x1000000`  
min value: `0`  
max value: `4`  
</details>
<details>
<summary><code>gamepad_stick_layout</code></summary>

Gamepad stick layout (used by menus)

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_toggle_ads</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_togglecrouch_hold</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_trigger_threshold</code></summary>



default: `"30"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_use_per_scope_ads_settings</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_use_per_scope_sensitivity_scalars</code></summary>

Gamepad uses the per scope scalars

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_use_type</code></summary>

Gamepad use scheme (used by menus), 0: hold use, tap reload, 1: tap use, hold reload, 2: tap use/reload

default: `"2"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gameui_xbox</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gamma_adjusted</code></summary>

Whether player has done gamma adjustment

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gatherprops_no_wait</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gfx_desaturate_force</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gl_clear_color_buffer</code></summary>

Enable or disable the clearing of the main color buffer.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>gl_clear_fogcolor</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>gl_clear_randomcolor</code></summary>

Clear the back buffer to random colors every frame. Helps spot open seams in geometry.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>glass_break_required_speed</code></summary>



default: `"150"`  
flags: `0x6000`  
</details>
<details>
<summary><code>glass_shatter_direction_force_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>glass_shatter_force_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>glass_shatter_size_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>glass_shatter_use_real_direction</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>glitch_aberrationScale</code></summary>

How far apart the glitch cloak samples should be.

default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>global_lighting_partial_update</code></summary>

Allow partial uploads of GPU lights (optimization.)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_count</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_driven_tex_stream</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_driven_tex_stream_single_thread</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_level</code></summary>

GPU Level - Default: High

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_level</code></summary>

GPU Level - Default: High

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_mem_level</code></summary>

Memory Level - Default: Normal

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_mem_level</code></summary>

Memory Level - Default: Normal

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_vram_size_mb</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>grapple_accel_human</code></summary>

Speed added per second from grapple, up to the grapple_speedRamp* speed

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_accel_titan</code></summary>

Speed added per second from grapple, up to the grapple_speedRamp* speed

default: `"1800"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_around_obstacle_accel</code></summary>

Acceleration around obstacles while grappling

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMantle</code></summary>

After detaching from grapple, how long to keep trying to automantle

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleeConvergeTime</code></summary>

Simplify relative velocities when the enemy is this many seconds away from hitting us (increases chances of a hit)

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleeOnDetach</code></summary>

Starts a melee sequence when the grapple detaches.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleePredict</code></summary>

Whether to run grapple melee logic on the client (tends to mispredict anyway)

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleePredictTime</code></summary>

Melee begins when the enemy is this many seconds away from hitting us

default: `"0.13"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleeViewRotateSpeedFar</code></summary>

Speed at which view rotates toward grapple melee target

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleeViewRotateSpeedNear</code></summary>

Speed at which view rotates toward grapple melee target

default: `"3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_debug</code></summary>

Show grapple debug info

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_decelMeleeStrength</code></summary>

Strength of extra deceleration that forces melee targets to come to you

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_decel_human</code></summary>

Deceleration of player's speed that doesn't go toward the grapple point

default: `"425"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_decel_titan</code></summary>

Deceleration of player's speed that doesn't go toward the grapple point

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_detachExtraAllowedLength</code></summary>

Extra allowed grapple length before detaching once it's attached

default: `"256"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_disableMeleeWhenActive</code></summary>

Disallows melee when the grapple is out.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_dontFightGravity</code></summary>

Ignores downward speed when applying deceleration, so that gravity continues to pull you down

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_fallSpeed</code></summary>

Fall speed of the grapple hook while it's returning

default: `"300"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_forcedRetractVel</code></summary>

Return speed of grapple hook when grapple is finished or cancelled

default: `"3000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_gracePeriod</code></summary>

Length of time player can grapple without using a charge, in case they mess up

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_gravityPushUnderContribution</code></summary>

Pushing forward while looking "under" the grapple point increases gravity this much

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulseOffGround_human</code></summary>

Initial launch speed off the ground when grapple connects

default: `"50"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulseOffGround_human_npc</code></summary>

Initial launch speed off the ground when grapple connects

default: `"150"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulseOffGround_titan</code></summary>

Initial launch speed off the ground when grapple connects

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulse_human</code></summary>

Initial launch speed when grapple connects

default: `"350"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulse_titan</code></summary>

Initial launch speed when grapple connects

default: `"350"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSlowFracVert_human</code></summary>

Fraction of vertical speed that is retained when grapple connects

default: `"0.4"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSlowFracVert_titan</code></summary>

Fraction of vertical speed that is retained when grapple connects

default: `"0.1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSlowFrac_human</code></summary>

Fraction of XY speed that is retained when grapple connects

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSlowFrac_titan</code></summary>

Fraction of XY speed that is retained when grapple connects

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSpeedMin_human</code></summary>

When grapple connects, player speed is immediately set to at least this value (negative = away, positive = towards)

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSpeedMin_titan</code></summary>

When grapple connects, player speed is immediately set to at least this value (negative = away, positive = towards)

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_jumpFrac</code></summary>

Jump velocity multiplier when grappled

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_letGravityHelpCosAngle</code></summary>

Don't ignore gravity when grappling downward this much (0 is horizontal, 1 is straight down)

default: `"0.8"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_lift</code></summary>

Distance above grapple hook that player is pulled to

default: `"25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_pullDelay_human</code></summary>

Grapple delay between attachment and acceleration

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_pullDelay_titan</code></summary>

Grapple delay between attachment and acceleration

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_retractVel</code></summary>

Return speed of grapple hook when it hasn't hit anything yet

default: `"6000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_rodeoVerticalImpulse</code></summary>

Vertical impulse applied to the player when grappling off of a rodeo.

default: `"750"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_shootVel</code></summary>

Outward speed of grapple hook

default: `"2000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampMax_human</code></summary>

Player will accelerate to this speed after grapple_speedRampTime has passed

default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampMax_titan</code></summary>

Player will accelerate to this speed after grapple_speedRampTime has passed

default: `"750"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampMin_human</code></summary>

Player will accelerate to this speed while grappling; lerps to grapple_speedRampMax over grapple_speedRampTime

default: `"50"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampMin_titan</code></summary>

Player will accelerate to this speed while grappling; lerps to grapple_speedRampMax over grapple_speedRampTime

default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampTime_human</code></summary>

Time from grapple_speedRampMin to grapple_speedRampMax

default: `"1.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampTime_titan</code></summary>

Time from grapple_speedRampMin to grapple_speedRampMax

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_swingAngle</code></summary>

Maximum angle from vertical that swinging will generate acceleration (it will tend to zero acceleration at this angle)

default: `"45"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_swingPullAngle</code></summary>

If the player is pushing forward within this angle of the pull direction, then switch out of swinging mode.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_swingPullSpeedLength</code></summary>

When swinging, the grapple pull speed scale begins to scale back to 1.0 at lengths below this

default: `"300.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_swingPullSpeedScale</code></summary>

When swinging, the grapple pull speed is scaled by this much

default: `"0.025"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_titanEmbarkDist</code></summary>

Distance at which to begin embark when grappling to your own titan.

default: `"250"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_windowCheckDist</code></summary>

Check for window hints at this distance from grapple point

default: `"150"`  
flags: `0x2002`  
</details>
<details>
<summary><code>gravity_grenade_decel</code></summary>

Deceleration applied by gravity grenade to nearby objects

default: `"20000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>gravity_grenade_projectile_min_speed</code></summary>

Gravity grenade never slows projectiles below this speed

default: `"600"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ground_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ground_trace_hull_radius</code></summary>

How wide of a sphere is the trace for getting a character's ground surface

default: `"12.0f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grx_hasUnknownItems</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>gtao_angle_bias</code></summary>

angle in degree [0-90)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gtao_intensity</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>gtao_intensity_in_lobby</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>gtao_thickness_heuristic</code></summary>

in range of [0,1)

default: `"0.2"`  
flags: `0x2`  
</details>
<details>
<summary><code>hasAnyAssetsWithDiscardedStreamableData</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>hasMic</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>hasPartialInstall</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>hbao_angle_bias</code></summary>

angle in degree [0-90)

default: `"6"`  
flags: `0x2`  
</details>
<details>
<summary><code>hbao_intensity</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>hbao_stepsize_random</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>hbaobasic_tangent_bias</code></summary>

angle in degree [0-90)

default: `"25"`  
flags: `0x2`  
</details>
<details>
<summary><code>hidehud</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>highlight_deferred_update</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>highlight_draw</code></summary>

highlight_draw 0|1

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>highlight_lazy_clear_buffers</code></summary>

highlight_lazy_clear_buffers 0|1

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>highlight_object_max_count</code></summary>

highlight_object_max_count OBJECT_MAX_COUNT

default: `"255"`  
flags: `0x2`  
</details>
<details>
<summary><code>hitbox_bodygroup_check</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>hitch_alert_active</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>hitch_alert_color</code></summary>

The hitch/choke allerts will use this color.

default: `"255 255 0 255"`  
flags: `0x2`  
</details>
<details>
<summary><code>hitch_alert_show_large_snapshots</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_RunFrameServerAlways</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_ShowIPCCallCount</code></summary>

Print # of IPC calls this number of times per second. If set to -1, the # of IPC calls is shown every frame.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_flush_threshold</code></summary>

Memory threshold below which the host should flush caches between server instances

default: `"12"`  
flags: `0x80000`  
</details>
<details>
<summary><code>host_forceTakeHomeBuild</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_framerate</code></summary>

Set to lock per-frame time elapse.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_hasIrreversibleShutdown</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_limitlocal</code></summary>

Apply cl_cmdrate and cl_updaterate to loopback connection

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_map</code></summary>

Current map name.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>host_print_frame_times</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_profile</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_runframe_input_parcelremainder</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_server_thread_min_ticks</code></summary>

Only run the server thread when it needs this many ticks.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_sleep</code></summary>

Force the host to sleep a certain number of milliseconds each frame.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>host_speeds</code></summary>

Show general system running times.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_syncfps</code></summary>

Synchronize real render time to host_framerate if possible.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_thread_join_fast</code></summary>

If true we force the server thread join before existing '_Host_RunFrame'

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_thread_mode</code></summary>

Run the host in threaded mode, (0 == off, 1 == if multicore, 2 == force)

default: `"1"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>host_threaded_sound</code></summary>

Run the sound on a thread (independent of mix)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_timescale</code></summary>

Prescale the clock by this amount.

default: `"1.0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>hostname</code></summary>

Hostname for server.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>hover_vehicle_passenger_left_attachment_name</code></summary>



default: `"passenger1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>http_StryderKey</code></summary>



default: `"LABj38NWSTxHUhdYaP62ZU6HtutCas3L"`  
flags: `0x12`  
</details>
<details>
<summary><code>http_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_debug_forceFailRate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_debug_forceFailStatus</code></summary>



default: `"429"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_failuresAsErrors</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_maxAllocateAttempts</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_recv_fail_realloc</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_sandbox</code></summary>



default: `"EARW.50"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_showQueries</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>hud_autoreloadscript</code></summary>

Automatically reloads the animation script each time one is ran

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>hud_setting_accessibleChat</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_adsDof</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_compactOverHeadNames</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_damageIndicatorStyle</code></summary>



default: `"2"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_damageTextStyle</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_enableModWheel</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_healthUseOnHold</code></summary>

use health by holding button

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_healthWheelToggle</code></summary>

toggle health wheel on press

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_healthWheelUseOnRelease</code></summary>

use health after selecting it

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_lootPromptStyle</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_minimapRotate</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_ordnanceUseOnHold</code></summary>

use ordnance by holding button

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_ordnanceWheelToggle</code></summary>

toggle ordnance wheel on press

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_ordnanceWheelUseOnRelease</code></summary>

use ordnance after selecting it

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_pingAlpha</code></summary>



default: `"1.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_pingDoubleTapEnemy</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_pingWheelToggle</code></summary>

toggle ping wheel on press

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showButtonHints</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showCallsigns</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showHopUpPopUp</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showLevelUp</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showMedals</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showMeter</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showObituary</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showTips</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showWeaponFlyouts</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_streamerMode</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hudchat_new_message_fade_duration</code></summary>

How long messages added to the text chat will take to fade from opaque to not visible

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>hudchat_new_message_shown_duration</code></summary>

How long messages added to the text chat stick around with the panel not focused

default: `"12"`  
flags: `0x2`  
</details>
<details>
<summary><code>hudchat_play_text_to_speech</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hudchat_transition_message_mode_fade_duration</code></summary>

When switching message mode of the text chat panel how long it takes to transition visibility

default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>hudchat_visibility</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hudwarp_chopsize</code></summary>

Number of pixels to a primitive before chopping for warping.

default: `"60.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_override</code></summary>

Use convar settings for hud warp (instead of script-provided settings)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_viewDist</code></summary>

Distance back from sphere center to use when 2d projecting.

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_xScale</code></summary>

Final scale for X (after projecting sphere surface to 2d.)

default: `"1.2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_xWarp</code></summary>

Degrees of arc of sphere to use (0-90, low distortion to high.)

default: `"45.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_yScale</code></summary>

Final scale for Y (after projecting sphere surface to 2d.)

default: `"1.1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_yWarp</code></summary>

Degrees of arc for Y warp (0-90, low distortion to high.)

default: `"30.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>idcolor_ally</code></summary>



default: `"0.34 0.59 0.86 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_ally_cb1</code></summary>



default: `"0.24 0.50 0.96 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_ally_cb2</code></summary>



default: `"0.0 0.58 0.77 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_ally_cb3</code></summary>



default: `"0.28 0.52 0.97 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_enemy</code></summary>



default: `"0.8 0.25 0.15 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_enemy_cb1</code></summary>



default: `"0.89 0.78 0.0 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_enemy_cb2</code></summary>



default: `"1.0 0.627 0.68 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_enemy_cb3</code></summary>



default: `"0.82 0.74 0.06 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_neutral</code></summary>



default: `"1.0 1.0 1.0 0.6"`  
flags: `0x2`  
</details>
<details>
<summary><code>ignore_fatal_errors</code></summary>

Don't exit on fatal errors.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ignore_script_errors</code></summary>

Ignore script errors.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ik_debug</code></summary>

Enables debug lines for IK

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_debug_chain</code></summary>

Allows specifying a single IK chain name for IK debugging

default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_debug_ent</code></summary>

Allows specifying a single entity for IK debugging

default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_debug_text</code></summary>

Enables IK debug text; requires ik_debug

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_enable</code></summary>

Enables IK

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_enable_client</code></summary>

Enables IK on the client

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust</code></summary>

Enable ik height adjustment

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust_debug</code></summary>

Debugging for ik height adjustment

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust_move_speed</code></summary>

IK height adjustment speed per unit of horizontal velocity in units per second

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust_sine</code></summary>

Test ik height adjustment with a sine wave

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust_speed</code></summary>

IK height adjustment speed as a fraction of step size per second

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_latch</code></summary>

Enables IK latching to ground during footsteps

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_normal_lerp_rate</code></summary>

Rate at which feet adjust to a new ground orientation in angles per second

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_unlatch_max_rate</code></summary>

Maximum rate an IK'd bone can unlatch; prevents pop on animation transition

default: `"5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ime_enabled</code></summary>

Enabled the IME

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>imgui_buildmode</code></summary>

Show the imgui implementation of the Build Mode dialog

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>imgui_buildmode</code></summary>

Show the imgui implementation of the Build Mode dialog

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>impact_allow</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>impact_debug_info</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>impact_victim_offset_dist</code></summary>

Distance to offset impact sounds from the victim, when requested

default: `"256"`  
flags: `0x2`  
</details>
<details>
<summary><code>impulse_low_decel_duration_scalar</code></summary>

Impulse magnitude is multiplied by this to give a length of time that the player can't decelerate

default: `"0.003"`  
flags: `0x2002`  
</details>
<details>
<summary><code>inPartyChat</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>in_forceuser</code></summary>

Force user input to this split screen player.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>in_syncRT</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>in_usekeyboardsampletime</code></summary>

Use keyboard sample time smoothing.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>inbox_enabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>infoblock_requestInterval</code></summary>

Time between info block requests

default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>input_did_turn_threshold</code></summary>

Degrees per second.

default: `"1.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>intro_viewed</code></summary>

Whether the introduction video has been viewed by this player

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>ip</code></summary>

Overrides IP for multihomed hosts

default: `"localhost"`  
flags: `0x80000`  
</details>
<details>
<summary><code>joy_advaxisr</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisu</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisv</code></summary>



default: `"6"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisx</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisy</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisz</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_inverty</code></summary>

Whether to invert the Y axis of the joystick for looking.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>joy_legacy</code></summary>

Turn on/off 'Legacy' mapping for control sticks.

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>joy_movement_stick</code></summary>

Which stick controls movement (0 is left stick)

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>joy_requireFocus</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_rumble</code></summary>

Controller rumble.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>joy_xcontroller_cfg_loaded</code></summary>

If 0, the 360controller.cfg file will be executed on startup & option changes.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>jpeg_quality</code></summary>

jpeg screenshot quality.

default: `"90"`  
flags: `0x2`  
</details>
<details>
<summary><code>jt_help_with_anything_ignore_preference</code></summary>

This let's JT_HelpWithAnything() work on tasks that are not preferred.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>jump_graceperiod</code></summary>

Extra time during which a player can jump after falling off a ledge

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>jump_keyboardgrace_max</code></summary>

Amount of velocity change allowed during jump_keyboardgraceperiod, as a fraction of sprinting speed

default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>jump_keyboardgrace_strength</code></summary>

Fraction of change toward the new direction when pressing a direction during jump_keyboardgraceperiod

default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>jump_keyboardgraceperiodmax</code></summary>

Extra time during which a player can change their direction with keyboard input after jumping (fades to 0 strength at this time)

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>jump_keyboardgraceperiodmin</code></summary>

Extra time during which a player can change their direction with keyboard input after jumping (at full strength)

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>killReplay_lagCompensate</code></summary>

Adjust player timing to try to match what the client saw rather than what the server saw.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>killReplay_playNonReplayRemoteCallsOnLocalClientPlayer</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>leaf_threadedRecompute</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>leaf_threadedRecompute_batchSize</code></summary>



default: `"12"`  
flags: `0x2`  
</details>
<details>
<summary><code>leech_npc_angle_cos</code></summary>

Cos(angle) allowed for leeching npcs

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>lerp_careAboutAttachmentBonePosition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>lerp_debugEnt</code></summary>



default: `"-2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>lerp_opt</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>lerp_threaded</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>lerp_threaded_numEntsPerTask</code></summary>



default: `"6"`  
flags: `0x2`  
</details>
<details>
<summary><code>light_maxcone</code></summary>

Max light cone limit.  Cone limit is half angle in degrees.

default: `"85"`  
flags: `0x2`  
</details>
<details>
<summary><code>lightmap_realtimelight</code></summary>

If true use the real-time light lightmap for selecting real-time lights.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>lightmap_realtimeshadows</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>load_during_video</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>loaderrorsCount</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>loaderrorsNeedShown</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>localClientPlayerCachedLevel</code></summary>



default: `"1"`  
flags: `0x1000010`  
</details>
<details>
<summary><code>locationInfo</code></summary>

What OS(on PC and Durango) or PSN account(on PS4) reports as the user's location

default: `""`  
flags: `0x210`  
</details>
<details>
<summary><code>locationInfo_nucleus</code></summary>

What origin(on PC) or nucleus(on console) reports as the user's location

default: `""`  
flags: `0x210`  
</details>
<details>
<summary><code>locator_background_border_color</code></summary>

The default color for the border.

default: `"255 255 255 15"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_border_thickness</code></summary>

How many pixels the background borders the left and right.

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_color</code></summary>

The default color for the background.

default: `"255 255 255 5"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_shift_x</code></summary>

How many pixels the background is shifted right.

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_shift_y</code></summary>

How many pixels the background is shifted down.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_style</code></summary>

Setting this to 1 will show rectangle backgrounds behind the items word-bubble pointers.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_thickness_x</code></summary>

How many pixels the background borders the left and right.

default: `"8"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_thickness_y</code></summary>

How many pixels the background borders the top and bottom.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_fade_time</code></summary>

Number of seconds it takes for a lesson to fully fade in/out.

default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_icon_max_size_non_ss</code></summary>

Minimum scale of the icon on the screen

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_icon_min_size_non_ss</code></summary>

Minimum scale of the icon on the screen

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_lerp_rest</code></summary>

Number of seconds before moving from the center.

default: `"2.25f"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_lerp_speed</code></summary>

Speed that static lessons move along the Y axis.

default: `"5.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_lerp_time</code></summary>

Number of seconds to lerp before reaching final destination

default: `"1.75f"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_pulse_time</code></summary>

Number of seconds to pulse after changing icon or position

default: `"1.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_split_len</code></summary>



default: `"0.5f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>locator_split_maxwide_percent</code></summary>



default: `"0.80f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>locator_start_at_crosshair</code></summary>

Start position at the crosshair instead of the top middle of the screen.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_target_offset_x</code></summary>

How many pixels to offset the locator from the target position.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_target_offset_y</code></summary>

How many pixels to offset the locator from the target position.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_topdown_style</code></summary>

Topdown games set this to handle distance and offscreen location differently.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>lookspring</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>lookstrafe</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>m_acceleration</code></summary>

Mouse acceleration.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>m_forward</code></summary>

Mouse forward factor.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>m_invert_pitch</code></summary>

Whether to invert the pitch axis of the mouse.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>m_side</code></summary>

Mouse side factor.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mainmenu_background_movie</code></summary>



default: `"media/frontend.bik"`  
flags: `0x2`  
</details>
<details>
<summary><code>map_settings_override</code></summary>

If this is enabled then the following ConVars will be functional and override the maps current value: fog_enable, mat_bloomscale

default: `"0"`  
flags: `0x40004002`  
</details>
<details>
<summary><code>mat_autoexposure_compensation</code></summary>

This works like exposure compensation on a camera, in EV units. 0EV is no compensation, -1EV gives half the light, +2EV gives 4x the light, etc. The exposure range is still subject to the min/max, so you might want to use mat_autoexposure_uncap 1.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_force_value</code></summary>



default: `"0.0"`  
flags: `0x40004000`  
</details>
<details>
<summary><code>mat_autoexposure_max</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_max_multiplier</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_min</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_min_multiplier</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_speed</code></summary>

Changes the speed at which exposure adapts to changes in scene luminance.

default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_uncap</code></summary>

mat_autoexposure_min and mat_autoexposure_max are ignored when this is set.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_cutoff</code></summary>



default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_max_lighting_value</code></summary>



default: `"5.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_bloom_scalefactor_scalar</code></summary>



default: `"0.1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mat_bloom_streak_amount</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_streak_cutoff</code></summary>



default: `"5.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_streak_cutoff_exposure_adapt</code></summary>

Whether streak cutoff value should scale with exposure values. R2 behavior is 0.0, R5 behavior is 1.0

default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_streak_exponent_post</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_streak_exponent_pre</code></summary>



default: `"1.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_wide_amount</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_wide_exponent_pre</code></summary>



default: `"1.5f"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloomamount_rate</code></summary>



default: `"0.05f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_bloomscale</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_checkStalls</code></summary>

If true, flushes then syncs the render thread to the GPU at various spots of code to find hidden GPU stalls.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_cloudmask</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_disableentities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_disableentities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_disableentities</code></summary>

Disable map color-correction entities

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_editor</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_editor</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_forceentitiesclientside</code></summary>

Forces color correction entities to be updated on the client

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_colorcorrection</code></summary>



default: `"1"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_debug_postprocess_allowed</code></summary>

Allow postprocessing when debug views are enabled.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_postprocessing_effects</code></summary>

0 = off, 1 = show post-processing in top left corner of screen

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_disable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_mid1</code></summary>



default: `"10.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_mid2</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_shoulder</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_toe</code></summary>



default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debugalttab</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_depthbias_decal</code></summary>

use integer value

default: `"-16"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_normal</code></summary>

use integer value

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_shadowmap</code></summary>

use integer value

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_tightshadowmap</code></summary>

use integer value. effective on View model selfshadow

default: `"10000"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_ui</code></summary>

use integer value

default: `"-50"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_zfill</code></summary>

use integer value

default: `"16"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_decal</code></summary>



default: `"-0.001"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_normal</code></summary>



default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_shadowmap</code></summary>



default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_ui</code></summary>



default: `"-0.001"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_zfill</code></summary>



default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthtest_force_disabled</code></summary>

only works on PC and XB1 for now

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_detail_tex</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_diffuse</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_disable_bloom</code></summary>



default: `"0"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>mat_disable_lightmap_ambient</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_disable_lightmaps</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_disable_model_ambient</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_drawMenuGrid</code></summary>

Enable menu grid guide overlay. Only accurate for 16:9 aspect ratio.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_drawTitleSafe</code></summary>

Enable title safe overlay

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_drawflat</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_dxlevel</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_dynamic_tonemapping</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_dynamic_tonemapping</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_enable_ssr</code></summary>

NOTE - UNABLE TO ENABLE - Toggle Screen Space Reflections.
If you want to use SSR again, uncomment the line with (1u << MTLENVOPT_SSR) in shader.cpp of bakery and then rebuild shaders.

default: `"0"`  
flags: `0x2`  
max value: `1`  
</details>
<details>
<summary><code>mat_envmap_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_envmaptgasize</code></summary>

Final envmap size for "envmap" console command; should be <= 128.

default: `"CUBEMAP_SCREENSHOT_RES"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_fastnobump</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_fastspecular</code></summary>

Enable/Disable specularity for visual testing.  Will not reload materials and will not affect perf.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_filterlightmaps</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_filtertextures</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_force_bloom</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_forceaniso</code></summary>



default: `"2"`  
flags: `0x40000000`  
min value: `0`  
max value: `16`  
</details>
<details>
<summary><code>mat_frame_color_bias</code></summary>

Add a constant value to the average frame color.

default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_frame_color_enabled</code></summary>

Update the average frame color each frame.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_frame_color_scale</code></summary>

Scale the average frame color.

default: `"5.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_frame_color_spot_metering_screen_ratio</code></summary>

Use a percentage of the screen around the center to compute the average frame color.

default: `"0.8"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_fullbright</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_fxaa_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_global_lighting</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_global_lighting</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_global_lighting</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_hdr_level</code></summary>

Set to 0 for no HDR, 1 for LDR+bloom on HDR maps, and 2 for full HDR on HDR maps.

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_hdrcolcorrection_editor</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_hdrcolorcorrection</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_hide_sun_in_last_cascade</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_instancing</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_letterbox_aspect_goal</code></summary>

Letterbox when the window aspect ratio is below this threshold

default: `"1.6"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_letterbox_aspect_threshold</code></summary>

Letterbox when the window aspect ratio is below this threshold

default: `"1.59"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_lightcull_subview</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_lightcull_subviews</code></summary>

Re-cull lighting for subviews (monitors etc.)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_local_contrast_edge_scale_override</code></summary>



default: `"-1000.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_local_contrast_midtone_mask_override</code></summary>



default: `"-1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_local_contrast_scale_override</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_local_contrast_vignette_end_override</code></summary>



default: `"-1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_local_contrast_vignette_start_override</code></summary>



default: `"-1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_1</code></summary>



default: `"0 0 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_2</code></summary>



default: `"1 0 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_3</code></summary>



default: `"1 1 2 2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_1</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_2</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_3</code></summary>



default: `"1 0 0 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_1</code></summary>



default: `"1 0 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_2</code></summary>



default: `"1 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_3</code></summary>



default: `"1 1 2 2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_1</code></summary>



default: `"1 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_2</code></summary>



default: `"1 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_3</code></summary>



default: `"1 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_1</code></summary>



default: `"0 0 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_2</code></summary>



default: `"0 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_3</code></summary>



default: `"1 1 2 2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_maxframelatency</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_mip_linear</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>mat_mipmaptextures</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_norendering</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_norendering</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_phong</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_picmip</code></summary>



default: `"0"`  
flags: `0x40000000`  
min value: `0`  
max value: `4`  
</details>
<details>
<summary><code>mat_postprocess_enable</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_postprocess_enable</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_proxy</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_reducefillrate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_report_queue_status</code></summary>



default: `"0"`  
flags: `0x800002`  
</details>
<details>
<summary><code>mat_reversedepth</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_screen_blur_enabled</code></summary>

Enables screen blur render step

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_screen_blur_override</code></summary>



default: `"-1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_shadowstate</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_sharpen_amount</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_sharpen_threshold</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_sharpen_width</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_show_texture_memory_usage</code></summary>

Display the texture memory usage on the HUD.

default: `"0"`  
flags: `0x5000`  
</details>
<details>
<summary><code>mat_showenvmapmask</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_showlowresimage</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_showmiplevels</code></summary>

color-code miplevels 2: normalmaps, 1: everything else

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_skipid</code></summary>

Don't draw a particular mesh id. Helps track down which mesh you care about.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_sky_color</code></summary>

forces the color of sky ambient; the alpha value of 0 means no override.

default: `"0.0 0.0 0.0 0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_sky_scale</code></summary>

scales all sky ambient light by a constant factor

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_decal</code></summary>



default: `"-4"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_normal</code></summary>



default: `"0.0f"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_shadowmap</code></summary>



default: `"2"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_ui</code></summary>



default: `"-1.7"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_zfill</code></summary>



default: `"2"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_sun_color</code></summary>

forces the color of the sun directional light; the alpha value of 0 means no override.

default: `"0.0 0.0 0.0 0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_sun_scale</code></summary>

scales all sun direct light by a constant factor

default: `"1.0"`  
flags: `0x40004000`  
</details>
<details>
<summary><code>mat_surfacefilter</code></summary>

If set, limits surfaces shown by mat_surfaceid and mat_surfacemat to those containing the substring.

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_surfaceid</code></summary>

Draws the index of world surfaces. Can be filtered with mat_surfacefilter.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_surfacemat</code></summary>

Draws the material name of world surfaces. Can be filtered with mat_surfacefilter.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_syncGPU</code></summary>

If true, syncs the render thread to the GPU at the end of each frame, instead of letting the render thread get one frame ahead.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_syncInterval</code></summary>

Number of frames to skip per sync. 0 = novsync, 1 = 60 fps, 2 = 30, 3 == 20, 4 = 15, etc.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_sync_rt</code></summary>

Sync the render thread after each queued call. This is really slow, but makes debugging much easier.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_sync_rt_flushes_gpu</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_texture_list</code></summary>

For debugging, show a list of used textures per frame

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_texture_list_view</code></summary>

If this is nonzero, then the texture list panel will render thumbnails of currently-loaded textures.

default: `"1"`  
flags: `0x1002`  
</details>
<details>
<summary><code>mat_translucency_errors</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_vignette_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_warn_texture_convert</code></summary>

Print warnings for textures that had to be converted at load time, slowing down loads. 0 = off, 1 = old size not smaller, 2 = any change

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_backingOutMaxTimeToWait</code></summary>



default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_backoutslow</code></summary>

Forces empty server queries (for backing out of a lobby) to take this long

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_connect</code></summary>

If set to 0, we won't actually connect to any matchmaking results we get back

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_defaultMap_party</code></summary>

Default map to load if the dedicated server is empty

default: `"mp_lobby"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_dir</code></summary>

What dir to look in for the matchmaking scripts

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>match_dumpSearchResults</code></summary>

Dumps search result text to the console

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_emptyUpdateRate</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_enabled</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_fakePort</code></summary>

Lie about our port number (so players can't connect)

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_fakeS2SPort</code></summary>

Lie about our s2s port number (so servers can't connect)

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_forceVerboseSearches</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_goodReputation</code></summary>



default: `"1"`  
flags: `0x80000202`  
</details>
<details>
<summary><code>match_maxPingsSent</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_myBestDatacenter</code></summary>

Which datacenter we have the lowest ping to

default: `""`  
flags: `0x80080200`  
</details>
<details>
<summary><code>match_myDatacenter</code></summary>

Which datacenter we prefer (same as match_myBestDatacenter unless user changes it)

default: `""`  
flags: `0x80080200`  
</details>
<details>
<summary><code>match_myRankedDatacenter</code></summary>

Which datacenter we prefer for Ranked play (same as match_myBestDatacenter unless user changes it)

default: `""`  
flags: `0x80080200`  
</details>
<details>
<summary><code>match_myTeam</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>match_partyChangeNum</code></summary>

The int that represents the change num of our party struct (did it change?)

default: `""`  
flags: `0x200`  
</details>
<details>
<summary><code>match_partySize</code></summary>

The size of our party

default: `""`  
flags: `0x200`  
</details>
<details>
<summary><code>match_partySub</code></summary>

The name of our party subscription

default: `""`  
flags: `0x200`  
</details>
<details>
<summary><code>match_pingWaveInterval</code></summary>



default: `"0.2"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_playlist</code></summary>

The playlist we are looking for

default: `""`  
flags: `0x80200`  
</details>
<details>
<summary><code>match_precachemap</code></summary>

Whether to precache the map for the selected playlist

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>match_privateMatchListWithStryder</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_rankedMaxPing</code></summary>



default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>match_rankedSwitchETA</code></summary>



default: `"300"`  
flags: `0x2002`  
</details>
<details>
<summary><code>match_resetPlaylistBetweenMatches</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_roleToken</code></summary>

The role token used when matchmaking (e.g. for private match).

default: `""`  
flags: `0x80200`  
</details>
<details>
<summary><code>match_searchInterval</code></summary>

How often to repeat searches

default: `"2"`  
flags: `0x80000`  
</details>
<details>
<summary><code>match_searching</code></summary>

Whether or not we want the system to be actively searching right now

default: `"0"`  
flags: `0x80200`  
</details>
<details>
<summary><code>match_teamNoFill</code></summary>

If set, matchmaking won't fill the player's team with non-party members

default: `"0"`  
flags: `0x80200`  
</details>
<details>
<summary><code>match_updateNotableRate</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_updateRate</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_useMatchmaking</code></summary>

This dedi is a matchmaking dedi

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_verbosePrintsInterval</code></summary>



default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_visiblePlaylists</code></summary>



default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>matchmaking_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>max_explosive_damage_mass</code></summary>

Anything heavier than this will be clamped. (units kg)

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>max_explosive_damage_velocity</code></summary>

inches/sec

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>max_tweak_shadow_updates</code></summary>



default: `"8"`  
flags: `0x2`  
</details>
<details>
<summary><code>melee_aim_assist_can_lock_pitch</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_aim_assist_use_target_velocity</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_attack_trace_can_use_lunge_distance</code></summary>



default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_cone_trace_box_check</code></summary>



default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_abort_distance</code></summary>

Abort the lunge if the distance moved in one frame is less than this much of the expected lunge distance.

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_abort_if_blocked</code></summary>

Lunging can abort if the player hits something that blocks their lunge movement.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_adjust_trace_distance</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_align_eye_position</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_dot_check</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_force_enable_flying</code></summary>

Lunging will always ignore gravity.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_lag_compensate_target</code></summary>

Lunging will apply lag compensation the target's position.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_scale_by_speed</code></summary>

Increase lunge range (by up to the given scale) if the player is going fast enough.

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_slide</code></summary>

When lunging, try slide along surfaces

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_use_closest_distance_between_cylinders</code></summary>

When calculating distance to the lunge target, treat them as cylinders rather than points.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_use_command_time</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_queue_attack_anim_event</code></summary>

Run melee attacks after the player has moved this frame

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mem_dumpstats</code></summary>

Dump current and max heap usage info to console at end of frame ( set to 2 for continuous output )


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_force_flush</code></summary>

Force cache flush of unlocked resources on every alloc

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_force_flush_section</code></summary>

Cache section to restrict mem_force_flush

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_incremental_compact_rate</code></summary>

Rate at which to attempt internal heap compaction

default: `".5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mem_level</code></summary>

Memory Level - Default: High

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_level</code></summary>

Memory Level - Default: High

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_max_heapsize</code></summary>

Maximum amount of memory to dedicate to engine hunk and datacache (in mb)

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_max_heapsize_dedicated</code></summary>

Maximum amount of memory to dedicate to engine hunk and datacache, for dedicated server (in mb)

default: `"64"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_min_heapsize</code></summary>

Minimum amount of memory to dedicate to engine hunk and datacache (in mb)

default: `"48"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_runheapchecks</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_test_each_frame</code></summary>

Run heap check at end of every frame


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_test_every_n_seconds</code></summary>

Run heap check at a specified interval


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_test_quiet</code></summary>

Don't print stats when memtesting

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>menu_faq_community_version</code></summary>



default: `"-1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>menu_faq_patchnotes_version</code></summary>



default: `"-1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>menu_faq_viewed</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>menu_was_multiplayer_played_last</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>migrate_attempt_interval</code></summary>



default: `"2.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>migrate_attempt_max_retries</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_actor_occlusion_radius</code></summary>

Distance which must be penetrated for one of the entity check points to be considered occluded.

default: `"8.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_channels</code></summary>

Number of audio channels, commonly 2(stereo), 6(5.1), 8(7.1). (0 is default)

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>miles_flip_active_window_logic</code></summary>

Only hear audio when NOT the active window.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>miles_force_emitter_environment</code></summary>

Force Environment on played sounds and entities (per-event controllers and suffixes.)

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_force_listener_environment</code></summary>

Force environment on listener (i.e., global controller changes only)

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_freeze</code></summary>

When 1, sound is paused and incoming play events are ignored.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_initial_occlusion_delay</code></summary>

Time (in msec) to delay new sounds when we defer their traces.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_language</code></summary>

Language to use for audio (requires a miles restart to change.)

default: `""`  
flags: `0x1000000`  
</details>
<details>
<summary><code>miles_listener_freeze</code></summary>

When 1, stop updating listener position.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_nonactor_occlusion</code></summary>

Do traces to determine when non-entity sounds are occluded.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_nonactor_occlusion_radius</code></summary>

Distance which must be penetrated for a non-entity sound to be considered occluded.

default: `"8.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_nopandist</code></summary>

Distance at which panning is forced to center-front.

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_occlusion</code></summary>

When nonzero, perform occlusion checks

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>miles_occlusion_force</code></summary>

0 to 100: Force all sounds to have occlusion values of 0 (unoccluded) to 100 (completely occluded). -1 for normal.

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_occlusion_partial</code></summary>

When zero, occlusion state is binary. When nonzero, allow partial occlusion of audio.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_occlusion_use_reset_after_deferred_initial</code></summary>

For A/B testing feature. Enable permanently eventually.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_samplerate</code></summary>

Sample rate, commonly 48000, 44100, 22050, or 11025 (0 is default)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_server_sounds_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_server_sounds_print</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_solo_ents</code></summary>

Only play sounds from this entity index (or space-separated list of indices.)

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_soundscape_imgui</code></summary>

Show imgui-based soundscape debugging window

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_spatialize_front_degrees</code></summary>

Front panning field angle

default: `"45.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_spatialize_offplane_strength</code></summary>

Offplane omni-fication strength

default: `"0.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_spatialize_on</code></summary>

Enable hard spatialization test

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_spatialize_rear_degrees</code></summary>

Rear panning field angle

default: `"120.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_suffixes</code></summary>

Use emitter suffixed versions of sounds.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>min_explosive_damage_mass</code></summary>

Anything lighter than this will be clamped. (units kg)

default: `"20"`  
flags: `0x2002`  
</details>
<details>
<summary><code>missile_default_speed</code></summary>



default: `"2500"`  
flags: `0x2002`  
</details>
<details>
<summary><code>missile_homing_speed</code></summary>



default: `"150"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mod_trace_load</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>model_defaultFadeDistMin</code></summary>

Default minimum fade distance.

default: `"400"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_defaultFadeDistMin</code></summary>

Default minimum fade distance.

default: `"400"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_defaultFadeDistScale</code></summary>

Factor that is multiplied by the model's radius to get the default fade distance.

default: `"40"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_defaultFadeDistScale</code></summary>

Factor that is multiplied by the model's radius to get the default fade distance.

default: `"40"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_fadeRangeFraction</code></summary>

Fraction of the fade distance to fade over.

default: `"0.1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_fadeRangeFractionNear</code></summary>

Fraction of the near fade distance at which impostors are invisible.

default: `"0.9"`  
flags: `0x4000`  
</details>
<details>
<summary><code>monitor_cc</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_mat_sharpen_amount</code></summary>



default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_postfx</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_rui_world_enabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_snapshot_frame_delay</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_zfar_default</code></summary>



default: `"642"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_zfar_override</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_zfar_override_enabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>motd</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>mouse_sensitivity</code></summary>

Mouse sensitivity.

default: `"5"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_use_per_scope_sensitivity_scalars</code></summary>

Uses the per scope scalars

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_0</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_1</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_2</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_3</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_4</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_5</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_6</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_7</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>move_one_cmd_per_client_frame</code></summary>

Force clients to generate exactly one user command per client frame. There will not be a one-to-one relationship between cmds and ticks.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>movement_anim_downed_playback_maxrate</code></summary>



default: `"2.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>movement_anim_playback_maxrate</code></summary>



default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>movement_anim_playback_minrate</code></summary>



default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>movement_anim_sprint_playback_maxrate</code></summary>



default: `"1.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_accountLink_requestInterval</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_allowed</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_bodyyawrate</code></summary>



default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_countRRNobodyAsLobby</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_enablematchending</code></summary>

When set to 0, match will not end

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_enabletimelimit</code></summary>

enable mp_timelimit timer in games

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_gamemode</code></summary>

Current game mode name

default: `""`  
flags: `0x12002`  
</details>
<details>
<summary><code>mp_linkingAccountTime</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_linkingAccountWindow</code></summary>



default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_maxbodyyaw</code></summary>



default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_permission_requestInterval</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_permission_rerequestInterval</code></summary>



default: `"21600"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_player_level</code></summary>

To read mp player level in SP

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>mp_scaleAnimationSpeeds</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_showgestureslots</code></summary>

Show multiplayer client/server gesture slot information for the specified player index (-1 for no one).

default: `"-1"`  
flags: `0x6002`  
</details>
<details>
<summary><code>mtx_svEdition</code></summary>



default: `"75"`  
flags: `0x2002`  
</details>
<details>
<summary><code>muteWeaponSounds</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>name</code></summary>

Current user name

default: `"unnamed"`  
flags: `0x480`  
</details>
<details>
<summary><code>net_RunInvalidatePhysics</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_async_sendto</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_autoUnthrottle</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_bandwidthPrintThreshold</code></summary>

Percentage where it's worth printing spam about this message in the bandwidth tracker prints

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_bindToSpecificAddress</code></summary>

Only bind to a certain interface

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_blockmsg</code></summary>

Discards incoming message: <0|1|name>

default: `"none"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_chatThroughChatserver</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_chokeloop</code></summary>

Apply bandwidth choke to loopback packets (only in MP)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_clearReliableDataOnReset</code></summary>

Whether we should erase unsent reliable data when we call netchan->Reset()

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_client_side_weapon_animations</code></summary>

Enable/disable client side weapon animations. Only apply to already optimized weapons, eg. rapid fire instant hit weapons like xo16, r101 etc.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_compressDataBlock</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_compressLZValue</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_compresspackets</code></summary>

Use lz compression on game packets.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_compresspackets_minsize</code></summary>

Don't bother compressing packets below this size.

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_connectPacketWarningThreshhold</code></summary>



default: `"0.9"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_connectingDataRate</code></summary>



default: `"128000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_createUndoDeltas</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_data_block_enabled</code></summary>

Enable/disable net data block optimization for load times. When disabled large chunks are sent down via existing netchan reliability system instead of net data blocks.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_datablockPrintSummaries</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_fastRate</code></summary>



default: `"128000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_longSendTime</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_minResendInterval</code></summary>



default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_networkLossForSlowSpeed</code></summary>



default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_resendRateForSlowSpeed</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_slowRate</code></summary>



default: `"64000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_debugDataBlockReceiver</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_debugDataBlockSender</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_debugLerping</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_deltaFieldEntityBlockSize</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_disconnectIfDeltaBufferIsFull</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_drawslider</code></summary>

Draw completion slider during signon

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_droppackets</code></summary>

Drops next n packets on client

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_dumpChangesPrecise</code></summary>

Prints floats at full precision

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_encrypt_copyCtx</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_encryptionDebug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_forceDeltaBufferToOverflow</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_forceUnnecessaryUndoDeltas</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_forcetimeout</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_fullyConnectedDataRate</code></summary>



default: `"256000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_highPacketLatencyThreshold</code></summary>



default: `"0.200"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_highPacketLossThreshold</code></summary>



default: `"0.05"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_ignoreAllSnapshots</code></summary>

Drop all snapshot messages

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_largeSnapshotThreshold</code></summary>

The size of a snapshot that qualifies as a large snapshot

default: `"15000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_lerpFields</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_lowBandwidthConnect</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_maxAccumulatedClearTimeBalance</code></summary>

Max time (in seconds) to count not sending data to this player towards their 'remaining bandwidth' balance [if we haven't sent a packet in 2 minutes, that doesn't mean they have 2 minutes of bandwidth remaining to use]

default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_maxcleartime</code></summary>

Max # of seconds we can wait for next packets to be sent based on rate setting (0 == no limit).

default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_maxfilesize</code></summary>

Maximum allowed file size for uploading in MB

default: `"16"`  
flags: `0x2`  
min value: `0`  
max value: `64`  
</details>
<details>
<summary><code>net_maxfragments</code></summary>

Max fragment bytes per packet

default: `"1200"`  
flags: `0x2`  
min value: `256`  
max value: `1200`  
</details>
<details>
<summary><code>net_maxroutable</code></summary>

Requested max packet size before packets are 'split'.

default: `"1200"`  
flags: `0x202`  
min value: `576`  
max value: `1200`  
</details>
<details>
<summary><code>net_minConnectionTimeForSpam</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_minQueuedPacketsForPrint</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_minResetIdleTimerInterval</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_minimumPacketLossDC</code></summary>

The lowest packet loss we have to any datacenter

default: `"100"`  
flags: `0x200`  
</details>
<details>
<summary><code>net_minroutable</code></summary>

Forces larger payloads.

default: `"16"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_netGraph2</code></summary>



default: `"0"`  
flags: `0x40080000`  
</details>
<details>
<summary><code>net_noPostDataForDeletedEnts</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_old_seed_generation</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_optimize_persistent_data</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_optimize_playlists</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_optimize_weapons</code></summary>

Enable/disable bandwidth optimizations made to weapons. Additional experimental optimizations can be enabled values 2 (weapon player data) and 3 (client side weapon animation)

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_predictParentEntities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_predictedEntsUseFirstAvailableSnapshot</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_predictionDebug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_pretendSnapshotArrayFull</code></summary>

Pretend the client snapshot array is full even when it isn't

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_printCompression</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_printOutOfSnapshots</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_printUnnecessaryDeltas</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_propSkipPrintThreshold</code></summary>

Show prop skips more than this many apart

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_queue_trace</code></summary>



default: `"0"`  
flags: `0x2000002`  
</details>
<details>
<summary><code>net_queuedPackets_PrintOversleeps</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_queuedPackets_SkipSmallSleeps</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_queued_packet_sender_nopacket_sleep</code></summary>



default: `"10"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_queued_packet_thread</code></summary>

Use a high priority thread to send queued packets out instead of sending them each frame.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recentNetworkGapWindow</code></summary>



default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recentNetworkGapsNeeded</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recreateScriptInstanceOnReplayTransition</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_dumpChanges</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_dumpNetworkedChangesOnEntCreate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_watchEnt</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_watchField1</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_watchField2</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>net_resourcePrintMinimum</code></summary>

Minimum count for printing bandwidth info about a resource (sound, effect)

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_sendFloatDeltas</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_sendProfileTotals</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_sendtoInJob</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showFailedAuth</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showLargeSnapshot</code></summary>

Show console spam when we get large snapshots from the server

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showQueued</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showUndoDeltas</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showUserWarnings</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showchoke</code></summary>

Show console spam when we get choked snapshots from the server


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showchokeInterval</code></summary>

The minimum time interval between spam about going above our network budget

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showdrop</code></summary>

Show dropped packets in console

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showfragments</code></summary>

Show netchannel fragments

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showmsg</code></summary>

Show incoming message: <0|1|name>

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showpeaks</code></summary>

Show messages for large packets only: <size>

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showsendrecv</code></summary>

Show sendto and recvfrom calls

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showsplits</code></summary>

Show info about packet splits

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_showudp</code></summary>

Dump UDP packets summary to console

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_showudp_oob</code></summary>

Dump OOB UDP packets summary to console

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_showudp_remoteonly</code></summary>

Dump non-loopback udp only

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_showusercmd</code></summary>

Show user command encoding

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_skipUnnecessaryDeltas</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_splitrate</code></summary>

Number of fragments for a splitpacket that can be sent per frame

default: `"3"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_splitrateDefaultMP</code></summary>

Default MP number of fragments for a splitpacket that can be sent per frame

default: `"3"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_splitrateDefaultSP</code></summary>

Default SP number of fragments for a splitpacket that can be sent per frame

default: `"10000"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_tamperPackets</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_threadedEntityDeltas</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_threadedProcessPacket</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_timeoutUsesLastReadTime</code></summary>

Don't let us time out if we haven't been actually checking the socket for packets (inside a loop, for example)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_trackerWarningInterval</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_usesocketsforloopback</code></summary>

Use network sockets layer even for listen server local player's packets (multiplayer only).

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_verifyEncryption</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_voiceEchoFromChatServer</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_warnAboutSocketReadGaps</code></summary>

Warn if we are waiting longer than this to check a socket for new packets

default: `"0.200"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_warnGapTime</code></summary>



default: `"0.4"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_wifi</code></summary>

0 = ethernet, 1 = wifi, -1 = unknown

default: `"-1"`  
flags: `0x80200`  
</details>
<details>
<summary><code>net_worldHitchSlopTime</code></summary>



default: `"0.031"`  
flags: `0x2`  
</details>
<details>
<summary><code>next</code></summary>

Set to 1 to advance to next frame ( when singlestep == 1 )

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>noReloadAfterUse</code></summary>

Disables reloads for "+useAndReload" input if a use is triggered.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>noise_filter_scale</code></summary>



default: `"0.006"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>not_focus_sleep</code></summary>

MS to sleep while window doesn't have focus

default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>notification_displayTime</code></summary>

How long notifications should wait before auto-hiding

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>nucleus_id</code></summary>



default: `"0"`  
flags: `0x80000200`  
</details>
<details>
<summary><code>nucleus_pid</code></summary>



default: `"unknown"`  
flags: `0x80000200`  
</details>
<details>
<summary><code>number_shortenToMillionsAfter</code></summary>



default: `"2000000"`  
flags: `0x2`  
</details>
<details>
<summary><code>object_placement_debug</code></summary>

Enable/disable debug lines for object placement

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>offhandTossOverheadPitchThreshold</code></summary>



default: `"-1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>offhand_alignEndAnim1p3p</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>old_culling</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>old_gather_props</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>one_handed_change_rate</code></summary>

The rate at which the transition to and from one handed weapon usage takes place

default: `"1.25"`  
flags: `0xa`  
</details>
<details>
<summary><code>opaque_renderable_worker</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>openInvite_spam</code></summary>

Whether open invites should spam to the console log

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>openInvites_filterByLanguage</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>openInvites_filterByRegion</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>openinvite_duration_default</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>ordnanceSwapSelectCooldown</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_Errorlevel_OldBehaviour</code></summary>

Enables Setting errorlevel for as in the old code base did.


default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_Errorlevel_Telementry</code></summary>

Enables sending host Telemetry event for Origin errorLevel


default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_authCodeFailureMaxBackoffSeconds</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_autoRefreshTokenClient</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_autoRefreshTokenServer</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_debug</code></summary>

Enable Origin HTTP debug logging (all HTTP queries and responses, token data etc.)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_disconnectWhenOffline</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_ignoreInvitesOnLoadScreen</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_igo_mutes_sound_enabled</code></summary>

Enables feature for optionally muting game sound when Origin overlays are launched.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_igo_muting_sound</code></summary>

True if game sound was muted when launching an Origin overlay.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_presense_updateRate</code></summary>

Minimum time between origin updates in seconds.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_tokenFailureMaxBackoffSeconds</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>panel_showVisChanges</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>panel_test_title_safe</code></summary>

Test vgui panel positioning with title safe indentation

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>parenting_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particleEffect_checkShouldStillPlay</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_alwayswakeonstop</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_cpu_level</code></summary>



default: `"0"`  
flags: `0x40000000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>particle_delete_all_except</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_dlights_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_dlights_spew</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_gpu_level</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_lighting_clear_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_lighting_size</code></summary>

The size of each particle in the atlas

default: `"32"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_lighting_viewmodel_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay</code></summary>

Show particle overlay (2 for same as particle_overlay_list_tally)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_detail_attributes</code></summary>

Space separated list of attributes to show per particle - 'all id duration xyz prev_xyz radius color alpha length'

default: `"id"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_detail_filter</code></summary>

Filters which particles to see in detail - can be id or substring or *

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_detail_list_particles</code></summary>

List individual particles in detail view

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_detail_scroll</code></summary>

Skip this many rows in particle overlay detail

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_hide_sleeping</code></summary>

Hide sleeping effects in particle overlay

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_list_filter</code></summary>

Filters which particles to see in list - can be id or substring or *

default: `"*"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_list_tally</code></summary>

Show tally of particle counts, rather than list (same as particle_overlay 2)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_list_tally_collapse_children</code></summary>

Collapse children in tally-- only show totals at top level.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_old</code></summary>

Draw particle overlay the old way (no imgui)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_scroll</code></summary>

Skip this many rows in particle overlay

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_remap_vol2cp_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_script_dump</code></summary>

particle_script_dump SCRIPT_HANDLE

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_script_list</code></summary>



default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_script_log</code></summary>

particle_script_log SCRIPT_HANDLE

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_scrub_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_debug_effect</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_is_using_time_scrub</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_scrub_max_dt</code></summary>



default: `"0.02"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_play_speed</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_quality</code></summary>



default: `"6"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_time</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_simulateoverflow</code></summary>

Used for stress-testing particle systems. Randomly denies creation of particles.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>particles_cull_dlights</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particles_max_passes</code></summary>



default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particles_spawncull</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particles_spawncull_report</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>parties_alwaysReadSubs</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_autoCreatePartyAlways</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_autoCreatePartyDelay</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_color_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>party_doRealNameLookups</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_doRealNameLookupsForOwner</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>party_httpHandleTimeout</code></summary>



default: `"10.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_keepAliveTime</code></summary>

How often party clients should send a keepalive packet

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_keepAliveTime</code></summary>

How often party clients should send a keepalive packet

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_leaderAlwaysDetectsChanges</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_leaveMatchOnJoin</code></summary>

Whether a player should quit the match they're in when they join a party

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>party_lookupRealNamesForOpenInvites</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_lookupRealNamesForOpenInvitesForOwner</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_minSize</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_privacy</code></summary>

our privacy setting for parties

default: `"open"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_readyToSearch</code></summary>

our ready-up status

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_relyOnPartyForMemberUserInfo</code></summary>

If true, we won't re-request userinfo speculatively, only when their version changes in our party block

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_requireConsensusForSearch</code></summary>

Whether everyone in the party has to ready up before finding a match

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>perTriangleCollisionForced</code></summary>

Forces all traces on static models to use high detail traces.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>perfcharts_killswitch</code></summary>

A value of 1 means perfcharts is deactivated.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>perfcharts_pin_enabled</code></summary>

Enable telemetry events (pin).

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>perfcharts_pin_print</code></summary>

Print telemetry events (pin).

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>persistenceDef_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>persistenceDef_queryMaxHttpRetries</code></summary>



default: `"4"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistenceDef_readMaxHttpRetries</code></summary>



default: `"2"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistenceDef_retryReadAfterErrorTime</code></summary>



default: `"15"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistenceDef_writeMaxHttpRetries</code></summary>



default: `"4"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistence_clForceNew</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>persistence_disableForBuildProcess</code></summary>



default: `"0"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistence_enforce_manifest</code></summary>

Enable validating against manifest.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>persistence_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>persistence_new_player_if_upgrade_fails</code></summary>

Create a new player if upgrade fails. (dev only)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>persistence_upload_def</code></summary>



default: `"1"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistence_upload_failure_is_error</code></summary>



default: `"1"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistent_warningRate</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pertrianglecollision</code></summary>

Enables per-triangle collision with TRACEDETAILLEVEL_HIGH (i.e., bullets) on static models.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_bounce</code></summary>



default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_cfm</code></summary>

Constraint Force Mixing value. Softens the force applied to resolve constraints. ode.org/ode-latest-userguide.html: "If CFM is set to zero, the constraint will be hard .... the constraint is allowed to be violated by an amount proportional to CFM times the restoring force that is needed to enforce the constraint"

default: `"0.0001"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_cfm_anglejointstop</code></summary>



default: `"0.0001"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_drawContacts</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_drawContactsDuration</code></summary>



default: `"0.016666"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_drawGeoms</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_drawTunnelChecks</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_enableObjectPairCollidePrototype</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_erp</code></summary>

Fraction of penetration that physics tries to resolve per time step. At 1.0, all contacts add a velocity that will end the penetration in a single frame, though this is unstable. At 0.0, contacts create no outward force (though they still provide friction).

default: `"0.05"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_erp_anglejointstop</code></summary>



default: `"0.05"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_frictionDefault</code></summary>



default: `"0.82"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_showObjectCount</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_threadGoWide</code></summary>

Go wide across threads with Physics.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>physics_async_cl</code></summary>

Run physics simulation asynchronously from the main thread.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>physics_autoSleepAngularThreshold</code></summary>

Angular speed below which a physic object goes to sleep. (in degrees / second)

default: `"120"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_autoSleepDebug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_autoSleepGroundHysteresis</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_autoSleepSpeedThreshold</code></summary>

Speed below which a physic object goes to sleep.

default: `"20"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_collideWithMovingGeo</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_defaultMaxAngularSpeed</code></summary>



default: `"10000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_defaultMaxSpeed</code></summary>



default: `"10000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_scaled_mem</code></summary>

Amout of extra memory taken by scaled collision meshes

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>physics_tunnelChecks</code></summary>

Do traces to prevent physics objects from falling through the world.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_tunnelChecksForceAlways</code></summary>

Require objects to do tunnel checks every frame.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>pin_opt_in</code></summary>

Enables sending PIN telemetry data to EA

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>pin_plat_id</code></summary>

Platform user id for PIN

default: `"0"`  
flags: `0x80000202`  
</details>
<details>
<summary><code>pin_sid</code></summary>

session id

default: `"unknown"`  
flags: `0x80000200`  
</details>
<details>
<summary><code>pin_telemetry_actually_send</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_debug_code</code></summary>

Shows unformatted json of all messages

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_debug_payload</code></summary>

Shows final payloads being sent to PIN server, including header

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_debug_script</code></summary>

Shows nicely formatted json of script messages

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_dont_send_events</code></summary>

List of PIN events to suppress

default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>pin_telemetry_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_inactivity_send_time</code></summary>

Interval at which client PIN messages are sent. (Client only)

default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_max_payload_size</code></summary>



default: `"30720"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_send_debug</code></summary>

Enables x-ea-lint-level 2 for useful error messages

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_debug</code></summary>

Debug latency calculation.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_max_green</code></summary>



default: `"70"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_max_red</code></summary>



default: `"250"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_max_yellow</code></summary>



default: `"140"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_minSentForChoice</code></summary>

Minimum number of pings sent to this target (not received) before we are willing to say the player can matchmake because we're confident that this data is useful

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_qos_units</code></summary>

Divisor to use for pings, so we don't think a 3 ping is wildly better than a 4 ping, but we do think a 33 ping is worse than a 31 ping (at 60fps, that's another frame of latency)

default: `"32"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_usePacketLoss</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pixvis_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pixvis_maxquads</code></summary>

Change the upper bound on how many 2x2 quads to sample for pixel visibility

default: `"256"`  
flags: `0x4000`  
</details>
<details>
<summary><code>pixvis_spew</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>plat_environment</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>plat_retryNameLookups</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>platform_user_id</code></summary>

Platform user id (origin user id on PC, xuid on xboxone)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerListPartyColorB</code></summary>



default: `"204"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerListPartyColorG</code></summary>



default: `"255"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerListPartyColorR</code></summary>



default: `"179"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerListUseFriendColor</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_ADS_buffer_time_seconds</code></summary>

How long (in seconds) will the game buffer a Toggle Zoom attempt if the player cannot ADS when they press the button.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_debugPredictedPosition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_deltaAnimsMakeMeUnpredicted</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_doJetwashEffects</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_extraairaccelleration</code></summary>

Extra air acceleration given to players, even if they're already at max speed. Helps to start wall running

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_highFrequencyThinkDistance</code></summary>



default: `"6000"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_movementBounds_predictionShare</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_movingDeathThreshold</code></summary>



default: `"50"`  
flags: `0x6000`  
</details>
<details>
<summary><code>player_respawnInputDebounceDuration</code></summary>

How long after respawning will certain player inputs be debounced for

default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_setting_autosprint</code></summary>

Automatically sprint when walking forward.

default: `"0"`  
flags: `0x41000000`  
</details>
<details>
<summary><code>player_setting_damage_closes_deathbox_menu</code></summary>

Controls whether death box automatically closes when taking damage (used for menus).

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>player_setting_stickysprintforward</code></summary>

Double-tapping sprint will keep the player sprinting forward.

default: `"0"`  
flags: `0x41000200`  
</details>
<details>
<summary><code>player_showEyePosition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_useMovementBounds</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_viewchange_debug_pitch</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_viewchange_debug_roll</code></summary>



default: `"9"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_viewchange_debug_yaw</code></summary>



default: `"160"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_changeGamemodeAutomatically</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_debug_getvar</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_debug_localization</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_dump</code></summary>

Dump contents of playlists file (and patches) when we parse it

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_privateMatchEnabled</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>playlist_rotationGroup</code></summary>

Current rotation group among playlists

default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>playlist_rotationInterval</code></summary>

How often to rotate playlist groups in minutes

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_rotationIntervalDefault</code></summary>

Default value of how often to rotate playlist groups in minutes

default: `"150"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_rotationIntervalOverride</code></summary>

Override value (if > 0) of how often to rotate playlist groups in minutes

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_rotationNextTime</code></summary>

Next rotation time for playlist group

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>playlist_variableErrorsChecks</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>portal_pointpush_debug</code></summary>

Debug the portal_pointpush.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>portal_pointpush_think_rate</code></summary>

The amount of time between thinks for the portal_pointpush.

default: `"0.05f"`  
flags: `0x6000`  
</details>
<details>
<summary><code>portal_use_player_avoidance</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>postdataupdate_threaded</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>postdataupdate_threaded_chunksize</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>printConnectTimings</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>print_timeprefix</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>process_pending_vm_effects</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>progressbar_allow_wrap</code></summary>

Allow loading bar to wrap.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>progressbar_high_precision</code></summary>

Use a higher precision bar.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>progressbar_single_bar</code></summary>

Use a single bar.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_fake_prediction_in_kill_replay</code></summary>

Calls weapon primary-attack callbacks on client during replay to create predicted projectiles

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_faketrails</code></summary>

Enables fake projectile trails when the projectile impacts on the server before lag compensation is complete

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_filltrails</code></summary>

Fill the gap between the gun barrel and the first seen projectile position for trail Fx (1: 1st person only, 2: 3rd person only, 3: 1st and 3rd persons)

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_lagCompensationDebug</code></summary>

Draws lag compensation on projectiles

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationDebugDrawNoReg</code></summary>

Debug draw when noreg is detected.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationDebugDrawTime</code></summary>

Amount of time debug drawing persists with projectile_lagCompensationDebug enabled.

default: `"3.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationDebugExtra</code></summary>

Draws the "real" arc the projectile would take, as well as an extra simple simulation to compare with the actual path

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationDebugForceNoReg</code></summary>

Force no reg on all shots.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationDebugServerOffset</code></summary>

Offset the server debug lines by this many units vertically

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationMissileTimeStepScalar</code></summary>

Scales the time step used for seeking missiles in lag compensation

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_muzzleOffsetFirstPersonDecayDist</code></summary>

Distance over which projectiles fake their origin to come out of the gun muzzle

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_muzzleOffsetFirstPersonDecayMaxTime</code></summary>

Max time over which projectiles fake their origin to come out of the gun muzzle

default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_muzzleOffsetThirdPersonDecayDist</code></summary>

Distance over which projectiles fake their origin to come out of the gun muzzle

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_muzzleOffsetThirdPersonDecayMaxTime</code></summary>

Max time over which projectiles fake their origin to come out of the gun muzzle

default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_prediction</code></summary>

Performs client-side prediction and lag compensation on projectiles

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_predictionErrorCorrectTime</code></summary>

Time over which prediction errors are corrected for projectiles

default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>prop_lightweightPropsSkipAnimData</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>prop_survivalSkipsAnimData</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>props_break_burst_rotation</code></summary>

Rate of rotation in degrees per second.

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>props_break_max_pieces</code></summary>

Maximum prop breakable piece count (-1 = model default)

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>props_break_max_pieces_perframe</code></summary>

Maximum prop breakable piece count per frame (-1 = model default)

default: `"20"`  
flags: `0x2002`  
</details>
<details>
<summary><code>publication_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>push_cl</code></summary>

1: Moving geo pushes client entities, 0: Client entities do not get pushed

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>push_cl_always_update_prev_matrix</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>push_debug</code></summary>

Debug all pushing entities

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>push_debug_ent</code></summary>

Debug pushing entity

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>push_ragdolls</code></summary>

Toggles whether to push ragdoll entities

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>pve_debug</code></summary>



default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>pvs_addWorkItemsAccum</code></summary>

Accumulate this many work items from the main PVS job before adding them to the worker thread array, which can be slow

default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_addWorkItemsThreshold_edges</code></summary>

load balancing threshold; if a node has more than this many leaves, it will spread the work across threads

default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_addWorkItemsThreshold_leaves</code></summary>

load balancing threshold; if a node has more than this many leaves, it will spread the work across threads

default: `"5000"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_cullBoxes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_drawPortals</code></summary>

Draw portal access paths. when N > 0, access paths with edge count >= N will show up in green color. when N < 0, access paths with edge count == -N will show up in green.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>pvs_frustumCullOnly</code></summary>

0 - Off, 1 - On by Script, 2 - forced On

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_start_early</code></summary>

0 not early, 1 after view setup, 2 after threaded bone setup

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_AirboatViewDampenDamp</code></summary>



default: `"1.0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>r_AirboatViewDampenFreq</code></summary>



default: `"7.0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>r_AirboatViewZHeight</code></summary>



default: `"0.0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>r_JeepViewDampenDamp</code></summary>



default: `"1.0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>r_JeepViewDampenFreq</code></summary>



default: `"7.0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>r_VehicleViewDampen</code></summary>



default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>r_WaterDrawReflection</code></summary>

Enable water reflection

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_WaterDrawRefraction</code></summary>

Enable water refraction

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_aspectratio</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_blurmenubg</code></summary>

Blurs background when menus are open

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_bone_matrix_bulk_update_threshold</code></summary>



default: `"64"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_brush_queue_mode</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_createmodeldecals</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>r_cullshadowworldmeshes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_debug_draw_box_depth_test</code></summary>

Toggle depth test for debug draw box functionality

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_cover_count</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_cull_stretch_limit</code></summary>

Reciprocal of per-tri limit on decal stretching (0 is most permissive, 1 is most restrictive.)

default: `"0.707"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_decal_draw_basis</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_drawclipped</code></summary>

A bit-field! 1:Draw decal debug triangle overlays of *all* potential hits, 2:Draw actual hits, 4:Draw clipped hits

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_decal_overlap_area</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_overlap_count</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_test_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decals</code></summary>



default: `"256"`  
flags: `0x40000000`  
min value: `0`  
max value: `256`  
</details>
<details>
<summary><code>r_delay_texture_destroy</code></summary>

immediate call on Destroy() may cause GPU hang as it can still be used by GPU. this will make it delayed by one frame.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_ditherFade</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_ditherFade</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_ditherFadeShadows</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawallrenderables</code></summary>

Draw all renderables, even ones inside solid leaves.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawalphasort</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawbrushmodels</code></summary>

Render brush models. 0=Off, 1=Normal, 2=Wireframe

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawbrushmodels</code></summary>

Render brush models. 0=Off, 1=Normal, 2=Wireframe

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawdecals</code></summary>

Render decals.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawdepth_of_blend2transparent</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_drawdlights</code></summary>

whether to debug draw dlights

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawentities</code></summary>

0: dont' draw; 1: draw normal; 2: draw bones; 3: draw hulls

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawlightdist</code></summary>

If r_drawstaticlight is -1, only include draw lights within this radius

default: `"4000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawlightinfo</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawmodelsinzfill</code></summary>

Draw models in the zfill pass where they will affect light tile culling

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawopaquerenderables</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawparticles</code></summary>

Enable/disable particle rendering

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawrenderboxes</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawscreenspaceparticles</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_drawsky</code></summary>

Enable the rendering of sky

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawskybox_deprecated</code></summary>

Enable the rendering of sky boxes

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawstaticlight</code></summary>

0 = none, -1 = all within r_drawlightdist, other draws that light index

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawstaticprops</code></summary>

Toggle drawing of static props

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawtracers</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawvgui</code></summary>

Enable the rendering of vgui panels

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawviewmodel</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawworld</code></summary>

Render the world (0 = none, 1 = opaque only, 2 = trans only, 3 = both).

default: `"3"`  
flags: `0x40004000`  
</details>
<details>
<summary><code>r_dynamic</code></summary>



default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>r_earlyRenderables</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_enableOriginSort</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_fadeincode</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_farz</code></summary>

Override the far clipping plane. -1 means to use the value in env_fog_controller.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_fastzreject</code></summary>

Activate/deactivates a fast z-setting algorithm to take advantage of hardware with fast z reject. Use -1 to default to hardware settings

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_forcecheapwater</code></summary>

Force all water to be cheap water, will show old renders if enabled after water has been seen

default: `"0"`  
flags: `0x4008`  
</details>
<details>
<summary><code>r_jiggle_bones</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_lightmap</code></summary>



default: `"-1"`  
flags: `0x804000`  
</details>
<details>
<summary><code>r_lightprobe_force_trans_dist</code></summary>

if an entity moves this distance or greater in one frame it is automatically transitioned to a new probe

default: `"2000"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_lightstyle</code></summary>



default: `"-1"`  
flags: `0x804000`  
</details>
<details>
<summary><code>r_lod</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_lod</code></summary>



default: `"-1"`  
flags: `0x2`  
min value: `-1`  
</details>
<details>
<summary><code>r_lod</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_lod_shift</code></summary>

Shifts the quality of LODs towards higher LOD levels

default: `"0"`  
flags: `0x2`  
min value: `0`  
</details>
<details>
<summary><code>r_lod_shift</code></summary>

Shifts the quality of LODs towards higher LOD levels

default: `"0"`  
flags: `0x2`  
min value: `0`  
</details>
<details>
<summary><code>r_lod_switch_scale</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>r_mapextents</code></summary>

Set the max dimension for the map.  This determines the far clipping plane

default: `"16384"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_modeldecal_maxtotal</code></summary>



default: `"75"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_nearz</code></summary>

Near clipping plane distance

default: `"7"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_no_stalls</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_no_stalls</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_no_stalls</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_norefresh</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_debug</code></summary>

Toggle Particle Lighting Debug Texture

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_enable</code></summary>

Toggle Particle Lighting

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_enable</code></summary>

Toggle Particle Lighting

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_force</code></summary>

Force all particles to be lit

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_force</code></summary>

Force all particles to be lit

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_debug</code></summary>

Toggle Low Res Paricle Debug Texture

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_draw_weight_tex</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_force</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_tiled_composite</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_sim_spike_increment_ms</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_sim_spike_threshold_ms</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_timescale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particles_early_distance_cull</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_pos_debug</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>r_render_pos_debug</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>r_rimlight</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_rootlod</code></summary>

Root LOD

default: `"0"`  
flags: `0x4800002`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>r_rootlod</code></summary>

Root LOD

default: `"0"`  
flags: `0x4800002`  
</details>
<details>
<summary><code>r_ropetranslucent</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_setupBoneWorkSize</code></summary>

work size for SetupBone_Worker

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_setupBoneWorkerThreadhold</code></summary>

minimum ModelListByType_t::m_nSetupBoneCount value to be threaded for SetupBoneForModelList() call

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_shadowrendertotexture</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_sky_ignoreAngles</code></summary>

Ignore the angle of the sky (for debugging)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_sort_trans_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_sort_trans_debug_dist</code></summary>



default: `"2000"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_threaded_particles</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_updaterefracttexture</code></summary>

When disabled, supresses any update of refract texture.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_updaterefracttexture_allowmultiple</code></summary>

Allows multiple updates of refract texture per frame.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_visambient</code></summary>

Draw leaf ambient lighting samples. Mask of VIS_AMBIENT = 1, VIS_SKY = 2, VIS_SUN = 4, VIS_CLOUDMASK = 8, VIS_LIGHTS_1ST = 16, etc.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_visambient_orig</code></summary>

Show original lighting probes instead of the improved ones the game actually uses

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_visambient_point</code></summary>

Draw leaf ambient lighting samples, for a point (like particles).

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_vismodellighting_lightpos</code></summary>

Draw a line from the point light to the model lighting origin for this many of the closest lights that have r_vismodellighting enabled.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_maxdist</code></summary>



default: `"1000.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_mindist</code></summary>



default: `"48.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_offset_x</code></summary>

Offset the model lighting spheres by this amount.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_offset_y</code></summary>

Offset the model lighting spheres by this amount.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_offset_z</code></summary>

Offset the model lighting spheres by this amount.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visualizeproplightcaching</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_visualizetraces</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visualizetraces_duration</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_blur_count</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_blur_type</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_enabled</code></summary>

Toggle Volumetric Light

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_numSteps</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_rotate_dither</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_waterforceexpensive</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_waterforcereflectentities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_zfill</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ragdoll_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ragdoll_sleepaftertime</code></summary>

After this many seconds of being basically stationary, the ragdoll will go to sleep.

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>rankedplay_display_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>rankedplay_voice_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>rate</code></summary>

Max bytes/sec the host can receive data

default: `"128000"`  
flags: `0x2`  
</details>
<details>
<summary><code>reactive_wakeOnStop</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>real_time_update_dt</code></summary>



default: `"0.001"`  
flags: `0x2`  
</details>
<details>
<summary><code>recalculateOrigin_threaded_chunksize</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>reconnect_available_timespan</code></summary>

Amount of seconds for which reconnect is available since last save.

default: `"2400"`  
flags: `0x2`  
</details>
<details>
<summary><code>reconnect_check_platform_id</code></summary>

Check platform id from local file. Useful when testing with multiple local win64 clients.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>reconnect_debug</code></summary>

Never disconnect a player having lost connection.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>reconnect_delay</code></summary>

Amount of seconds for which reconnect is waiting before connecting.

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>reconnect_enabled</code></summary>

Whether disconnecting keeps the player around to allow reconnecting

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>remoteCalls_requireConnectionScriptsForViewPlayer</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>remoteMatchInfo_print</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>replay_enable</code></summary>

Enable Replay recording on server

default: `"1"`  
flags: `0x402002`  
</details>
<details>
<summary><code>replay_prediction_smooth</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>report_cliententitysim</code></summary>

List all clientside simulations and time - will report and turn itself off.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>report_clientthinklist</code></summary>

List all clientside entities thinking and time - will report and turn itself off.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>roamingcam_cameraspacemotion</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_downSpeedMult</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_forwardSpeed</code></summary>



default: `"1500"`  
flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_forwardSpeed_fast</code></summary>



default: `"10000"`  
flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_forwardSpeed_slow</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_movesmoothing</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_rollSpeed</code></summary>



default: `"40"`  
flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_sideSpeedMult</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_smoothtime</code></summary>



default: `"0.15"`  
flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_viewsmoothing</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rodeo_camera_smooth_blend_out_time</code></summary>



default: `"0.4"`  
flags: `0x2`  
</details>
<details>
<summary><code>rodeo_camera_smooth_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rodeoed_anims_enabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_collide</code></summary>

Collide rope with the world

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_debug_shake</code></summary>

Helps visualize ropes effected by a shake.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_parallelMeshBuilder</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_regenMeshEachDraw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_shake</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_texels_per_world_unit</code></summary>



default: `"8"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_harmonic_falloff</code></summary>

Falloff for oscillation magnitude of wave of increasing frequency (ropes and grapple)

default: `"0.75"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_magnitude_loose</code></summary>

Fraction of rope (including grapple) distance used as max wiggle distance while the rope is loose (shooting)

default: `".04"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_magnitude_tight</code></summary>

Fraction of rope (including grapple) distance used as max wiggle distance while the rope is tight (pulling or retracting)

default: `".002"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_oscillate_speed</code></summary>

Speed at which rope (including grapple) wiggle oscillates

default: `"64"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_rotate_speed</code></summary>

Speed at which rope (including grapple) wiggle rotates

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_zipline_min_points</code></summary>

Increases point count for ziplines that are wiggling

default: `"80"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wind_dist</code></summary>

Don't use CPU applying small wind gusts to ropes when they're past this distance.

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>rotate_ents</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rspn_motd</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>rt_sync_message_pump</code></summary>

If 1 render thread with process message queue before starting main thread processing. If 2 then main thread will wait for the next frames message queue processing before gathering mouse input. If 3 then the main thread will wait for the current frames message queue processing before gathering mouse input...this may have a problem with the queue message pump sometimes starting before gathering mouse input and sometimes after causing hitches, but it has less latency and lower frame times when compared to the other methods. If 0 then gathering mouse input may happen before, during, or after the last frames mouse messages were process causing horrible hitch mouse response.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rt_worker</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ruiPanel_resArgName</code></summary>



default: `"actualRes"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_asyncTracks</code></summary>

Toggles async update of RUI tracks

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_defaultDebugFontFace</code></summary>

Default font face for rui text in debug messages

default: `"PTMonoFont"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_defaultFontFace</code></summary>

Default font face for rui text

default: `"DefaultRegularFont"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_defaultFontHeight</code></summary>

Default font height for rui text

default: `"28"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_overrideVguiTextRendering</code></summary>

Use rui for rendering all vgui text

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_padDist</code></summary>



default: `"0.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_safeAreaFrac</code></summary>

Fraction of safe area to use

default: `"0.0"`  
flags: `0x2`  
max value: `1`  
</details>
<details>
<summary><code>rui_standardTextHeight</code></summary>



default: `"64.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>save_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>scheme_manager_font_debug</code></summary>

0:Off, 1:On

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>scr_centertime</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_back_range</code></summary>

Number of degrees behind the player that is considered more behind than to the side

default: `"60.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_ellipse_height</code></summary>



default: `"0.75"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_ellipse_width</code></summary>



default: `"0.85"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_pitch_limit</code></summary>

The maximum pitch difference that will affect the indicator position

default: `"75.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_pitch_scale</code></summary>



default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>screenfade_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_compile_all_levels</code></summary>

Compiles all level scripts when loading a map.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_connect_client_on_mapspawn</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_connect_ui_auto</code></summary>

Only takes effect after uiscript_reset. Use command line argument -script_debugger_connect_ui for startup.

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_host</code></summary>



default: `"localhost"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_debugger_port_client</code></summary>



default: `"15101"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_port_server</code></summary>



default: `"15100"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_port_ui</code></summary>



default: `"15102"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_disallow_newslot_on_globals</code></summary>

Throws compile errors for global variables assigned with <-

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_dump_simple</code></summary>

If enabled then script dump format will skip null array/table entries and display each non-container value on a single line with the fully scoped key name.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_error_on_midgame_load</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_infinite_loop_ms</code></summary>

If script runs for more than this many milliseconds at one time then you will get a script error.


default: `"10000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_parallel_trace_LOS_multiple</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_precache_errors</code></summary>



default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>script_printDeferredCalls</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_retry_after_compile_errors</code></summary>

After a compile error, tries compiling again immediately.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_seasonNameQueryInterval</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_showErrorDialogs</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_slopTimeBeforeBudgetEnforcement</code></summary>

How long to wait before we start complaining about slow budgets

default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_window_client_precache</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>seasonquest_force_missionscleared_count</code></summary>



default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>seasonquest_force_treasurepacks_count</code></summary>



default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>send_script_errors</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sequence_transitioner_enable</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>serverFilter</code></summary>

Only connects to servers with the same value

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>serverReports_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>server_concommands_allways_network</code></summary>

When set to 1 , server commands with listen server pass down the network layer.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>server_query_interval</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sfm_record_hz</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_always_update</code></summary>

Set to 1 to make shadow maps regenerate every frame.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_bleedfudge</code></summary>

Fudge value to decrease shadow map light bleeding

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_capable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_clear_dist</code></summary>



default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_dbg_draw</code></summary>

Visualize shadow atlas texture (1 .. 4, larger numbers for smaller sizes)Tweak - Purple, Dirty - Red, Dynamic - Green, Old Dynamic - Blue

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_default_filter_size</code></summary>

Size of the blur filter applied to spot shadows that don't request a different size. Odd integer only.

default: `"3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_depth_dimen_min</code></summary>

Minimum resolution of a spot shadow map in width and height

default: `"256"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_depth_upres_factor_max</code></summary>

Maximum requested upres factor of spot shadows (dimen_min << this) == largest spot shadow dimen

default: `"2"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_drawfrustum</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_dynamic_blendfactor</code></summary>

Blend dynamic shadows over time. Low value: long history, 1: no history

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_enable</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_esm_enable</code></summary>

(EXPERIMENTAL) Use exponential spot shadow maps instead of variance maps

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_filter_maxstep</code></summary>

Max step threshold for shadow map blend

default: `"0.18"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_info</code></summary>

Information about currently active depth shadows

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_lobby_mode_allowed</code></summary>

allow special mode for lobby that does some tricks to improve spotlight shadow quality. 0 - disallowed, 1- allowed, 2 - forced

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_max_downsizing_spot_updates</code></summary>

Downsizing shadows are exceptions. We want to update more of them than shadow_max_spot_updates

default: `"24"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_max_dynamic_lobby</code></summary>

Maximum number of shadows that should update every frame in Lobby.

default: `"5"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_max_old_dynamic</code></summary>

Maximum number of old shadows that should update every frame. It's a part of shadow_maxdynamic

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_max_spot_updates</code></summary>

Maximum number of dynamic shadow maps to update on any given frame

default: `"4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_maxdynamic</code></summary>

Maximum number of shadows that should update every frame.

default: `"4"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_maxdynamic</code></summary>

Maximum number of shadows that should update every frame.

default: `"4"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_min_count_smallest</code></summary>

Represents the minimum number of min resolution spot shadows to allocate in the shadow atlas.This will be adjusted upward to a multiple of the max sized spot shadow to find legal sized atlas dimensions.

default: `"576"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_minvariance</code></summary>

Minimum variance for shadow maps (controls edge softness)

default: `"0.00001"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_multisampled</code></summary>

Enable multisampling for shadows.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_noLOD</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_show_spot_udpate_infos</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_tools_depth_dimen_min</code></summary>

Minimum tools mode (lightedit) resolution of a spot shadow map in width and height

default: `"256"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_tools_depth_upres_factor_max</code></summary>

Maximum requested tools mode upres factor of spot shadows (dimen_min << this) == largest spot shadow dimen

default: `"3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_tools_min_count_smallest</code></summary>

Represents the minimum number of min resolution spot shadows to allocate in the shadow atlas in tools mode.This will be adjusted upward to a multiple of the max sized spot shadow to find legal sized atlas dimensions.

default: `"4096"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_tools_mode</code></summary>

Turn on shadow tools mode rendering (higher atlas size limits, running out of shadows does not spam

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_update_culling</code></summary>

Don't update shadows that aren't in the view frustum.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shake_angleFactor_human</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_angleFactor_titan</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_basicPitchFactor</code></summary>



default: `"0.20"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_basicRandomRollFactor</code></summary>



default: `"0.15"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_offsetFactor_human</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_offsetFactor_titan</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_viewmodelFactor_ads_human</code></summary>



default: `"0.01"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_viewmodelFactor_ads_titan</code></summary>



default: `"0.10"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_viewmodelFactor_human</code></summary>



default: `"0.10"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_viewmodelFactor_titan</code></summary>



default: `"0.10"`  
flags: `0x2`  
</details>
<details>
<summary><code>showfps_enabled</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showfps_heightpercent</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>showfps_mouse_latency</code></summary>

If 1 showfps_enabled will show mouse input latency instead of the time from before the move command.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>showfps_smoothtime</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>showfps_spinner</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>showmem_enabled</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showmem_mode_bottom</code></summary>

Mode 0 is total free memory(excluding garlic), 1 is small block heap, 2 is Client Script, 3 is  UI Script

default: `"3"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showmem_mode_top</code></summary>

Mode 0 is total free memory(excluding garlic), 1 is small block heap, 2 is Client Script, 3 is  UI Script

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showmemnumstats</code></summary>

Display On-Screen Numerical Memory Information

default: `"0"`  
flags: `0x2`  
min value: `0`  
</details>
<details>
<summary><code>showmemnumstatsrefresh</code></summary>

Refresh rate of querying the os for memory information in milliseconds.

default: `"500"`  
flags: `0x2`  
min value: `0`  
</details>
<details>
<summary><code>shownet_enabled</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showsnapshot_enabled</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sidearmSwapSelectCooldown</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>sidearmSwapSelectDoubleTapTime</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>single_frame_shutdown_for_reload</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>singlestep</code></summary>

Run engine in single step mode ( set next to 1 to advance a frame )

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>skill_arena</code></summary>

The arena that skill should be read from / written to (eg. fnf, experimental, etc)

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>skill_dediOnly</code></summary>

Only do skill for dedicated servers

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>skill_enabled</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>skill_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>skip_jump_height_fraction</code></summary>

Jump height fraction when skipping

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_jump_height_speed</code></summary>

Jump height loss only applies above this speed

default: `"450"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_replenish_double_jump</code></summary>

Whether the player can double jump after skipping

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_sounds</code></summary>

Enables skip-specific sounds

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_speed_reduce</code></summary>

Speed lost when skipping

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_speed_retain</code></summary>

Speed loss doesn't go below this

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_time</code></summary>

Time after landing that is considered "skipping" if the player jumps again

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sleep_when_meeting_framerate</code></summary>

Sleep instead of spinning if we're meeting the desired framerate.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sleep_when_meeting_framerate_headroom_ms</code></summary>

Only sleep if the current frame has at least this much time remaining, otherwise spin.

default: `"2.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_auto_stand</code></summary>

Automatically stand when slide ends

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>slide_max_angle_dot</code></summary>

Cosine of max angle from forward that you can slide when sprinting

default: `"0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>slide_step_velocity_reduction</code></summary>

Velocity reduction when going up a step (is multiplied by step height)

default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>slide_viewTiltDecreaseSpeed</code></summary>

Speed at which view tilt decreases while sliding in degrees per second

default: `"2.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_viewTiltIncreaseSpeed</code></summary>

Speed at which view tilt increases while sliding in degrees per second

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_viewTiltPlayerSpeed</code></summary>

Speed at which view tilt is full while sliding

default: `"400"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_viewTiltSide</code></summary>

View tilt when looking to the side while sliding in degrees

default: `"15"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_whileInAir</code></summary>

Allows beginning a slide (including the boost) while still in the air

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>slowconsolelog_old_logic</code></summary>

Flush console.log after each write.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>smoothstairs_lunge</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sort_opaque_meshes</code></summary>

Sort opaque meshes front to back to try to improve rendering speed.  This may not be worth the CPU cost.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_classic_music</code></summary>

classic music volume

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_entity_seek_snap</code></summary>

Play C_ImporantOnEntSound entity sound from beginning if we get it within this many seconds of its begin time.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_musicReduced</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_num_speakers</code></summary>

2 - headphones or stereo, 6 - 5.1 surround, 8 - 7.1 surround.  All other values invalid

default: `"2"`  
flags: `0x80`  
</details>
<details>
<summary><code>sound_only_warn_on_missing_sound_events_in_client_script</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_printloaderrors</code></summary>

Set to 1 to print sound errors on load.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_volume</code></summary>

master game volume

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_dialogue</code></summary>

dialogue volume (mp)

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_dialogue_sp</code></summary>

dialogue volume (sp)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_volume_music_game</code></summary>

music volume in game (mp)

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_music_game_sp</code></summary>

music volume in game (sp)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_volume_music_lobby</code></summary>

music volume in lobby

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_sfx</code></summary>

sound effect volume (mp)

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_sfx_sp</code></summary>

sound effect volume (sp)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_volume_voice</code></summary>

voice chat volume

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>sound_without_focus</code></summary>

Play sounds even when the app doesn't have focus.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>soundscape_fadetime</code></summary>

Time to crossfade sounds between soundscapes

default: `"2.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>soundscape_message</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>soundscape_radius_debug</code></summary>

Prints current volume of radius sounds

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>soundtrigger_repeat_interval</code></summary>

Decides how long to wait before repeating a soundtrigger event on the given player. Set to 0 to wait until the current sound ends.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sp_not_focus_pause</code></summary>

Pause the singleplayer game when the window is not in focus

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>spam_skinning_matrices_used</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spam_skinning_matrices_used_detailed</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spatial_partition_deadlock_assert</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>spectator_command_interval</code></summary>

Specify the minimum time between spectator command.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speech_queue_bytes</code></summary>



default: `"33000"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_audioenabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>speechtotext_forcedisabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_hostname</code></summary>



default: `"gateway-wdc.watsonplatform.net"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_msg_droptimeout</code></summary>



default: `"30.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_path</code></summary>



default: `"speech-to-text/api/v1/recognize?profanity_filter=true&smart_formatting=true"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_quiettime</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_errorspermin</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_interval</code></summary>



default: `"60.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_senderrors</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_sendrequests</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_sendsuccess</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotexttoken_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>speex_audio_recording</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_audio_value</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_agc_max_gain</code></summary>

Set maximal gain in dB. ( High values Risks swamping noise filter)

default: `"13"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_noise_suppress</code></summary>

Set maximum attenuation of the noise in dB (negative number)

default: `"-20"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_set_agc_decrenment</code></summary>

Set maximal gain decrease in dB/second.

default: `"-10"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_set_agc_increment</code></summary>

Set maximal gain increase in dB/second.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_set_agc_target</code></summary>

Set Automatic Gain Control target. 0/32767

default: `"8000"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_quiet_threshold</code></summary>



default: `"1300"`  
flags: `0x80`  
</details>
<details>
<summary><code>speex_quiet_window</code></summary>



default: `"40"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_set_enh</code></summary>

Set enhancement on/off (decoder only)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_use_highpass</code></summary>

Controlls the running o a lowpass filter do help remove DC.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_use_preproser</code></summary>

Controls the running of voice preprocessor.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>spinner_debug_info</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sprint_powerdrain</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sprint_view_shake_style</code></summary>



default: `"0"`  
flags: `0x41000000`  
</details>
<details>
<summary><code>sprinttilt_accel</code></summary>

Acceleration of sprint view tilt fraction

default: `"35"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sprinttilt_maxvel</code></summary>

Maximum speed of sprint view tilt

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sprinttilt_turnrange</code></summary>

Max turn rate that creates view tilt when sprinting

default: `"120"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ss_enable</code></summary>

Enables Split Screen support. Play Single Player now launches into split screen mode. NO ONLINE SUPPORT

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ss_force_primary_fullscreen</code></summary>

If enabled, all splitscreen users will only see the first user's screen full screen

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ss_mimic</code></summary>

Split screen users mimic base player's CUserCmds

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>ss_splitmode</code></summary>

Two player split screen mode (0 - recommended settings base on the width, 1 - horizontal, 2 - vertical (only allowed in widescreen)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ss_verticalsplit</code></summary>

Two player split screen uses vertical split (do not set this directly, use ss_splitmode instead).

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ss_viewmodelfov</code></summary>

Client-side viewmodel fov control that is global for all splitscreen players on this machine.  This gets overridden via splitscreen_config.txt for splitscreen.

default: `"54"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ss_voice_hearpartner</code></summary>

Route voice between splitscreen players on same system.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_allow_partial</code></summary>

When it's enabled, Partial SSAO could run when dynamic viewport is smaller than SSAO targets.
It doesn't look good and has a problem of some flickering. Try it with viewportscale_rand to see the problem.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_blur</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_blur_edge_sharpness</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_depth_max</code></summary>



default: `"10000"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_downsample</code></summary>

0 = 1:1, 1 = 2:1, 2 = 4:1

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>ssao_enabled</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>ssao_exponent</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_jitter_scale</code></summary>

in range of [0,1]

default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_max_res</code></summary>

SSAO render target size will be enforced to be this size when it's going to be ssao_max_res_threshold or greater

default: `"1080"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_max_res_threshold</code></summary>

ssao_max_res is enforced when SSAO render target size is at this size or greater

default: `"1440"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_num_directions</code></summary>



default: `"8"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_num_steps</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_on_everything</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_radius</code></summary>

occlusion hemisphere radius in world space unit

default: `"118"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_radius_in_lobby</code></summary>

occlusion hemisphere radius in world space unit

default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_show</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_show</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_show</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_snap_uv</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_tech</code></summary>

0 = Off, 1 = HBAO, 2 = GTAO uni, 3 = GTAO cos, 4 = HBAO basic, 5 = HBAO1x1, 6 = GTAOuni1x1, 7 = GTAOcos1x1

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_tech</code></summary>

0 = Off, 1 = HBAO, 2 = GTAO uni, 3 = GTAO cos, 4 = HBAO basic, 5 = HBAO1x1, 6 = GTAOuni1x1, 7 = GTAOcos1x1

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_upsample_ranged</code></summary>

It improves downsampled SSAO quality. it works for GTAO 4x4 mode only.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>startButtonCommand</code></summary>

What command to send when start is pressed

default: `"ingamemenu_activate"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_budget</code></summary>

The maximum number of static props that will be drawn.

default: `"8192"`  
flags: `0x2`  
min value: `100`  
max value: `8192`  
</details>
<details>
<summary><code>staticProp_buildlists_on_worker</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_debug_draw</code></summary>

Orange - regular culled prop. Red - "do not fade" prop Green - out of range

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_earlyDepthPrepass</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_earlyDepthPrepassDist</code></summary>



default: `"1500000"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_earlyDepthPrepassIncludeOpaques</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_earlyDepthPrepassIncludeOpaquesDist</code></summary>



default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_gather_size_weight</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_max_scaled_dist</code></summary>



default: `"2500.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_no_fade_scalar</code></summary>



default: `"0.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_refineDrawOnWorker</code></summary>

0 - none, 1 - mainview, 2 - depth-prepass, 3 - mainview & depth-prepass

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow</code></summary>

0 : off, 1 : generate once, 2 : minimum update with cache, 3 : update dirty rects with cache, 4 : update dirty rects without cache, 5 : always refresh

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow</code></summary>

0 : off, 1 : generate once, 2 : minimum update with cache, 3 : update dirty rects with cache, 4 : update dirty rects without cache, 5 : always refresh

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_bounds_per_env</code></summary>

0 - use world min/max, 1 - use current light environment's head box

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_debug_2d</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_debug_dirty_rects</code></summary>

only works with static_shadow_debug_2d = 1

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_depth_bias_scale</code></summary>

only effective on materials with non-zero shadowBiasStatic values

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_expand_z</code></summary>

z range should be inflated to be able to cover flying objects higher than world min/max

default: `"30000"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_good_merge_ratio</code></summary>

merge ratio = merged extent / bigger one's extent. when merge ratio is less than this value, the pair can be merged even when merged extent > m_StaticShadowMaxExtentForDirtyRect

default: `"1.01"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_good_merge_score</code></summary>

score = merged extent + wasted extent, where 4.0 is full screen

default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_prop_min_size</code></summary>

Minimum size of prop to be drawn in static shadow

default: `"40.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_res</code></summary>

Set the static shadow maps rendertarget resolution

default: `"4096"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_shrink_culler</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_use_d16</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_uses_shadow_lod</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticfile_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>stats_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>status_effect_warning_level</code></summary>

Set to 0 for nothing, 1 for warnings, 2 for script errors

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>steam_debug</code></summary>

Enable Steam HTTP debug logging

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>steam_id</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>steam_name</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>steamlink_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_addnoise</code></summary>

Adds corruption to streamed-in MIP levels for debugging.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_bsp_bucket_bias</code></summary>

Tweak MIP of BSP coverage (higher = blurrier mips)

default: `"-0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_bsp_dist_scale</code></summary>

Scale BSP coverage (relative to models) (higher = more important)

default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_capacity</code></summary>

Stream Cache Capacity in MiB

default: `"400"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_capacity_while_loading</code></summary>

Stream Cache Capacity in MiB while loading, if < 0 will be ignored.

default: `"200"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_high_priority_static_models</code></summary>

Try never to drop (and always prioritize loading) static model geo.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_multithreaded</code></summary>

Use jobs to do upload for model geo.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_preload_from_rpak</code></summary>

0 = Never preload; 1 = Preload static models

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_read_buffer_cap</code></summary>

Concurrent read buffer capacity in MiB.

default: `"32"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_read_count_cap</code></summary>

Concurrent read limit.

default: `"24"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_speculative_add_level</code></summary>

Attempt to add models to reach this fraction of stream_cache_capacity.

default: `"0.75"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_speculative_drop</code></summary>

Attempt to drop models to reach this fraction of stream_cache_capacity.

default: `"0.9"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_drop_unused</code></summary>

Drop unused textures aggressively

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_enable</code></summary>

Enable texture streaming

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_freeze_camera</code></summary>

Freezes camera for purposes of streaming map textures.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_load_after_drop</code></summary>

Allow us to continue loading in a frame after dropping any textures.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_memory</code></summary>

Stream memory to target (in kb).

default: `"300000"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>stream_memory_ignore</code></summary>

Ignore stream_memory limit when streaming is enabled.

default: `"0"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>stream_memory_ignore_vram</code></summary>

Ignore vram size when setting streaming buffer size.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_memory_min</code></summary>

Minimum streaming memory (in kb).

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>stream_memory_while_loading</code></summary>

Stream memory to target (in kb).

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_mode</code></summary>

Stream mode: default all none

default: `"default"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>stream_never_high_priority_frac</code></summary>

Never assign 'high priority' to a texture that uses more than this fraction of total streaming buffer.

default: `"0.0125"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_overlay</code></summary>

Texture streaming debug overlay.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_overlay_mode</code></summary>

Which debug view to show (tex mtl bsp short)

default: `"short"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_pause</code></summary>

Pause texture streaming

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_picmip</code></summary>

Picmip used when stream mode is picmip. (Or the map doesn't have streaming data.)

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_max_commits_per_frame</code></summary>

Cap on number of streaming texture commits allowed in a GPU frame. (0 disables cap)

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_thread</code></summary>

Create resources on separate thread, and delay copy and binding of those resources.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_wait_copy_to_commit</code></summary>

Number of frames to wait between copying old texture data and actually using a new texture.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_wait_creation_to_copy</code></summary>

Number of frames to wait between creating a texture and copying old texture data in.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_wait_for_additional_gpus</code></summary>

Enable to reset the commit counter less frequently when you have multiple GPUs.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_temp_abort_old_inner_loop</code></summary>

Temp stability - don't check for IST_ABORTED before calling Gfx_TextureAsset_EndAddMipLevels_Failed

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_temp_old_abort_all_behavior</code></summary>

Temp stability - StreamedDataManager_AbortStreamingTexture( true ) has broken (abort just one) behavior.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_temp_skip_abort_all</code></summary>

Temp stability - call to StreamedDataManager_AbortStreamingTexture( true ) does nothing

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stringtable_alwaysrebuilddictionaries</code></summary>

Rebuild dictionary file on every level load


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stringtable_compress</code></summary>

Compress string table for networking


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stringtable_showsizes</code></summary>

Show sizes of string tables when building for signon


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stryder_forceOriginUsersInvisible</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stryder_security</code></summary>



default: `""`  
flags: `0x80000200`  
</details>
<details>
<summary><code>stuck_debugging</code></summary>

Debug getting stuck

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>stuck_debugging_world_only</code></summary>

Only check for stuck in world geo

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>studiobonecache_unlimited</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>subscription_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>superjump_disabled_from_water</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_drain_power_onfail</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_fail_sound_when_jump_limit</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_limit</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_limitreset_onwallrun</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_max_power_use</code></summary>



default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_min_height_fraction</code></summary>

Minimum fraction of desired superjump height that is acheived, even if already moving quickly upwards

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_min_power_use</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_powerreset_onground</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_airaccelerate</code></summary>



default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_allTicksFinal</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_allowSendTableTransmitToClients</code></summary>

Allow transmission of sendtable data to clients.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_allowSpectatorClients</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_asyncSendSnapshot</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_backspeed</code></summary>

How much to slow down backwards motion

default: `"0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_balanceTeams</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_bounce</code></summary>

Bounce multiplier for when physically simulated objects collide with other objects.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_cheats</code></summary>

Allow cheats on server

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_checkPropBudgets</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_compressPlaylists</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_compressTimeValEpsilon</code></summary>



default: `"0.0005"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_compressTimeVals</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_connectingClientDelay</code></summary>

Amount of time to wait between resends of data to a connecting client

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_debug_prop_send</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_debugmanualmode</code></summary>

Make sure entities correctly report whether or not their network data has changed.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_disconnectOnScriptError</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_disconnectOnTooManySnapshotFrames</code></summary>

Disconnect client when the server has sent 128 snapshot messages to client without the server getting any message from the client.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_dumpstringtables</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_earlyPersistenceRead</code></summary>

Should the server try to read persistence earlier in the connection process


default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_everyThirdTick</code></summary>

Do networking every third tick, regardless of how backed up we are


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_extra_client_connect_time</code></summary>

Seconds after client connect during which extra frames are buffered to prevent non-delta'd update

default: `"60.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_fakeClientBaseId</code></summary>

Base platform user ID for created fake clients. Useful, for example, when running multiple dedis with matchmaking bots; create a different base for each dedi to get unique IDs -- matchmaking uses user IDs as a primary key.

default: `"9990000"`  
flags: `0x26`  
</details>
<details>
<summary><code>sv_footsteps</code></summary>

Play footstep sound for players

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_friction</code></summary>

World friction. (Equivalent player setting is in player settings files)

default: `"4"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_gravity</code></summary>

World gravity.

default: `"750"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_hibernate_ms</code></summary>

# of milliseconds to sleep per frame while hibernating

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_hibernate_ms_vgui</code></summary>

# of milliseconds to sleep per frame while hibernating but running the vgui dedicated server frontend

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_hibernate_postgame_delay</code></summary>

# of seconds to wait after final client leaves before hibernating.

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_hibernate_when_empty</code></summary>

Puts the server into extremely low CPU usage mode when no clients connected

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_infinite_ammo</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_instancebaselines</code></summary>

Enable instanced baselines. Saves network overhead.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_loadMapModelEarly</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_lobbyType</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_max_prop_data_dwords_lobby</code></summary>

Maximum amount of prop data per-snapshot in dwords (huge lobby)

default: `"300000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_prop_data_dwords_multiplayer</code></summary>

Maximum amount of prop data per-snapshot in dwords (huge multiplayer)

default: `"2500000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_prop_data_dwords_singleplayer</code></summary>

Maximum amount of prop data per-snapshot in dwords (singleplayer)

default: `"400000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_props_lobby</code></summary>

Maximum amount of props per-snapshot (lobby)

default: `"250000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_props_multiplayer</code></summary>

Maximum number of props per-snapshot (huge multiplayer)

default: `"1250000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_props_singleplayer</code></summary>

Maximum number of props per-snapshot (singleplayer)

default: `"300000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_snapshots_lobby</code></summary>

Maximum number of snapshots for the lobby

default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_snapshots_multiplayer</code></summary>

Maximum number of snapshots for multiplayer levels

default: `"160"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_snapshots_singleplayer</code></summary>

Maximum number of snapshots for singleplayer levels

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_maxclientframes</code></summary>



default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_maxrate</code></summary>

Max bandwidth rate allowed on server, 0 == unlimited

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1000000000`  
</details>
<details>
<summary><code>sv_maxroutable</code></summary>

Server upper bound on net_maxroutable that a client can use.

default: `"1200"`  
flags: `0x2`  
min value: `576`  
max value: `1200`  
</details>
<details>
<summary><code>sv_maxspeed</code></summary>



default: `"320"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_maxupdaterate</code></summary>

Maximum updates per second that the server will allow

default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_maxvelocity</code></summary>

Maximum speed any ballistically moving object is allowed to attain per axis.

default: `"34000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_minrate</code></summary>

Min bandwidth rate allowed on server, 0 == unlimited

default: `"128000"`  
flags: `0x82000`  
min value: `0`  
max value: `1000000000`  
</details>
<details>
<summary><code>sv_minupdaterate</code></summary>

Minimum updates per second that the server will allow

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_optimizedmovement</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_parallel_sendsnapshot</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_pausable</code></summary>

Whether the server is allowed to pause

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_playerNameAppendCheater</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_players</code></summary>



default: `"1"`  
flags: `0x2012`  
</details>
<details>
<summary><code>sv_printHighWaterMark</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_pushaway_accel</code></summary>

How hard physics objects are pushed away from the players.

default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_clientside</code></summary>

Clientside physics push away (0=off, 1=only localplayer, 2=all players)

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_clientside_size</code></summary>

Physics props below this size are made client side

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_debug</code></summary>

Debug physics object pushaway

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_dist</code></summary>

Max distance at which physics objects are pushed from players.

default: `"15"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_min_player_speed</code></summary>

If a player is moving slower than this, don't push away physics objects (enables ducking behind things).

default: `"75"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_player_accel</code></summary>

How hard the player is pushed away from physics objects

default: `"3000"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_pushaway_player_dist</code></summary>

Max distance at which player is pushed from physics objects

default: `"5"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_rejectClientConnects</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_rejectConnections</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_requireOriginToken</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_resendSignonData</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_rollangle</code></summary>

Max view roll angle

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_rollspeed</code></summary>



default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_runSpatialOptimizeInJob</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_scarySnapDeltaPrints</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_sendEarlyServerInfo</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_sendReplayNetMessagesOnNoDeltaSnaps</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_separate_freq_change_prop_send</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showClientTickCmds</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showLargeSnapshotSize</code></summary>



default: `"10000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showSnapshots</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showUserCmds</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_single_core_dedi</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_skipSendingUnnecessaryPersistence</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_skyname</code></summary>

Current name of the skybox texture

default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_snapshot_uniform_interval</code></summary>

A snapshot is created at uniform intervals, rather than according to final_tick

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_specaccelerate</code></summary>



default: `"1000.0"`  
flags: `0x2080`  
</details>
<details>
<summary><code>sv_specnoclip</code></summary>



default: `"1"`  
flags: `0x2080`  
</details>
<details>
<summary><code>sv_specspeed</code></summary>



default: `"5.0"`  
flags: `0x2080`  
</details>
<details>
<summary><code>sv_stats</code></summary>

Collect CPU usage stats

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_stopspeed</code></summary>

Minimum stopping speed when on ground. (Equivalent player setting is in player settings files)

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_stressbots</code></summary>

If set to 1, the server calculates data and fills packets to bots. Used for perf testing.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_struggleCheck</code></summary>

How long ago the 60th server frame can have been. 1.0 means the server is running in realtime. Higher means small hitches are ok.

default: `"1.016"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_struggleSpam</code></summary>

How long ago the 60th server frame can have been before it starts yelling. 1.0 means the server is running in realtime. Higher means small hitches are ok.

default: `"1.4"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_struggleSpamInterval</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_tempents_send_from_delta</code></summary>

Causes snapshot send code to walk back to delta, instead of always sending just current snapshot.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_tempents_send_from_last_sent</code></summary>

Causes snapshot send code to walk back to last m_lastSnapshotTick, instead of always sending just current snapshot.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_testLargeDatablock</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_teststepsimulation</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_transmitToAllPlayersMask_allBitsSet</code></summary>

This enables the legacy behavior of setting all bits inside of PerPlayerBitMask when we want to transmit an entities to all clients. This includes setting bits to clients that can't even exist(compare GetMaxClients to ABSOLUTE_PLAYER_LIMIT)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_unnecessaryConnectDelay</code></summary>

Amount of time to wait before responding to a connecting client (or malicious hacker)

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_unreliableSnapMaxSize</code></summary>

If we're sending a snapshot this size or larger, send it via the datablock sender. If a player has 4% packet loss, 10k of data would have a 40% chance of making it across with no resends

default: `"10000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_updaterate_mp</code></summary>

Maximum update rate at which server sends packets to clients in MP (updates per-second).

default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_updaterate_sp</code></summary>

Maximum update rate at which server sends packets to clients in SP (updates per-second).

default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_useReputation</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_useThreadsForSnapshots</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_voiceEcho</code></summary>

Server will return a voice chat message back to the sending client.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_voiceenable</code></summary>



default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>sv_warnAboutCmdNumJumps</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_watchdogTimer</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_wateraccelerate</code></summary>



default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_waterdist</code></summary>

Vertical view fixup when eyes are near water plane.

default: `"12"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_writePersistenceOnShutdown</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sys_attract_mode_timeout</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>system_alt_f4_closes_window</code></summary>

If set to true, alt+f4 will close the window

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>teams_unassigned_are_friendly</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>telemetry_client_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>telemetry_client_enable</code></summary>

Enable sending telemetry data

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>telemetry_client_sendInterval</code></summary>

How often to send telemetry data (seconds)

default: `"10.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>telemetryevent_client_enable</code></summary>

Enable sending client telemetry events

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>tencent_restricted</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>test_fakeTimeDays</code></summary>

Days worth of seconds that will be added to the result of GetUnixTimestamp() for script and playlist rotation. Server authoritive.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_damageScale</code></summary>

amount that stretching the tether damages it

default: `"0.00"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_dodge_damage</code></summary>

Damage done to tether by dodging away from it

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_healthDrain</code></summary>

rate at which tether health drains even if it isn't stretched

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_healthDrainNPC</code></summary>

rate at which tether health drains even if it isn't stretched (when attached to an NPC)

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_maxvel</code></summary>

max velocity with which tether pulls you back

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_radius</code></summary>

radius below which the tether does nothing

default: `"250"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_strength</code></summary>

strength with which tether pulls back (per unit past the radius)

default: `"25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>thirdperson_mayamode</code></summary>

Set to 1 to enable maya-like controls in game (only in third person) [Also don't move the camera when the mouse moves.]

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>thirdperson_override</code></summary>

Set to -1 to stop overriding. Set to 0 to force first person, 1 to force third person

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>thirdperson_screenspace</code></summary>

Movement will be relative to the camera, eg: left means screen-left

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>timeout</code></summary>

Seconds without communication before clients or servers will decide to disconnect.

default: `"15"`  
flags: `0x2`  
</details>
<details>
<summary><code>timeout_during_load</code></summary>

Seconds without communication during a level load before clients or servers will decide to disconnect.

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>timeout_reconnect</code></summary>

Seconds after a disconnected client is completely removed from the server, stopping it from reconnecting

default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>titan_sprint_sound</code></summary>



default: `"titan_eject_servos_3p"`  
flags: `0x2`  
</details>
<details>
<summary><code>toggle_on_jump_to_deactivate</code></summary>

Toggle or release jump to deactivate the offhand weapon

default: `"1"`  
flags: `0x280`  
</details>
<details>
<summary><code>tracehull_height_error_check</code></summary>

Error checking for hull traces requiring extents with larger heights than widths. 0 = none, 1 = warnings, 2 = assert and script errors

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tracer_debug</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>tracer_extra</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>trail_optimizedRemove</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_anim</code></summary>

Enables automantle animation

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_cooldown</code></summary>

Minimum time between traversals (in seconds)

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_enable</code></summary>

Enables player traversals

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_hand_debug</code></summary>

Enables debugging of traversal hand positioning

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_hand_required_width</code></summary>

Required width of geometry for hands (from center)

default: `"6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_viewLerpInDuration</code></summary>

Duration of view lerp from normal at the start of a traversal

default: `"0.15"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_viewLerpOut</code></summary>

Controls whether traversal view position and angle lerp back to normal at the end of a traversal

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_viewLerpOutAngle</code></summary>

Controls whether traversal view angle lerps back to normal at the end of a traversal

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_viewLerpOutDebug</code></summary>

Debugs traversal view position lerping

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_viewLerpOutPos</code></summary>

Controls whether traversal view position lerps back to normal at the end of a traversal

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_window_duration</code></summary>

Duration of window side traversal animation

default: `"0.3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_enable</code></summary>

Enables window traversals

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_finish_angle</code></summary>

Finishing yaw relative to the window's forward direction when starting at a 90 degree angle

default: `"45"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_forward_offset</code></summary>

Distance of player through the window after completing window traversal

default: `"6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_hand_vertical_offset</code></summary>

Vertical distance from hand position to eye position at start and end of window traversal

default: `"22"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_sideways_offset</code></summary>

Distance of player from the edge of the window toward the center of the window after completing window traversal

default: `"18"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_view_pitch_max</code></summary>

Max view pitch when doing window traversal

default: `"35"`  
flags: `0x4000`  
</details>
<details>
<summary><code>traversal_window_view_pitch_min</code></summary>

Min view pitch when doing window traversal

default: `"-80"`  
flags: `0x4000`  
</details>
<details>
<summary><code>traversal_window_yaw_max</code></summary>

Max view yaw when doing window traversal

default: `"80"`  
flags: `0x4000`  
</details>
<details>
<summary><code>trigger_crowd_pusher_enabled</code></summary>

Enables logic for TT_CROWD_PUSHER triggers

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>trigger_ignore_nonsolids</code></summary>

If set to false, non solid objects will activate triggers.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tsaa_blendfactorincreaseatmaxvelocity</code></summary>



default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_blendfactorincreasewhenunoccluded</code></summary>



default: `"5.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_blendfactormaxesoutatvelocity</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_blendfactormodulationonsparklesandunocclusion</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_blendfactoroverride</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_curframeblendamount</code></summary>



default: `"0.05"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_debugresponsiveflag</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_neighborhoodclamping</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_neighborhoodclampingsoftened</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_numsamples</code></summary>



default: `"64"`  
flags: `0x2`  
</details>
<details>
<summary><code>tweak_light_shadows_every_frame</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>twitch_check_interval</code></summary>

how often we ask if this user has a linked twitch prime account if we think they don't have one

default: `"3600"`  
flags: `0x2`  
</details>
<details>
<summary><code>twitch_prime_rewards</code></summary>



default: `""`  
flags: `0x210`  
</details>
<details>
<summary><code>twitch_shouldQuery</code></summary>

true if we should check to see if this user has a linked twitch prime account

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_fadecloud_time</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_fadexui_time</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_gameui_ctrlr_title</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_gameui_modal</code></summary>

If set, the game UI pages will take modal input focus.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_autotransition_time</code></summary>



default: `"5.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_fadein_time</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_fadeout_time</code></summary>



default: `"0.2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_fadeout_time</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_mintransition_time</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_transition_time</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_lobby_jointimeout</code></summary>



default: `"75"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_lobby_noautostart</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_lobby_noresults_create_msg_time</code></summary>



default: `"2.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_posedebug_fade_in_time</code></summary>

Time during which a new pose activity layer is shown in green in +posedebug UI

default: `"0.2"`  
flags: `0x24000`  
</details>
<details>
<summary><code>ui_posedebug_fade_out_time</code></summary>

Time to keep a no longer active pose activity layer in red until removing it from +posedebug UI

default: `"0.8"`  
flags: `0x24000`  
</details>
<details>
<summary><code>ui_virtualnav_render</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>unique_entity_names</code></summary>

Should entities have permanently unique entity names.  Or just concurrently unique?

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>usePromptBaseColor</code></summary>



default: `"255 255 255 255"`  
flags: `0x2`  
</details>
<details>
<summary><code>usePromptButtonTextColor</code></summary>



default: `"255 255 255 255"`  
flags: `0x2`  
</details>
<details>
<summary><code>usePromptImageScale</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>usePromptImageYOffset</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>usePromptTextColor</code></summary>



default: `"220 215 210 255"`  
flags: `0x2`  
</details>
<details>
<summary><code>use_monitors</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>use_valve_auto_gain</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>use_vm_cloak_offset</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>user_tracking_enabled</code></summary>



default: `"0"`  
flags: `0x12`  
</details>
<details>
<summary><code>users_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>v_centermove</code></summary>



default: `"0.15"`  
flags: `0x2`  
</details>
<details>
<summary><code>v_centerspeed</code></summary>



default: `"500"`  
flags: `0x2`  
</details>
<details>
<summary><code>variable_sights_gravity_scale_override</code></summary>

Projectile Gravity Scale to be used for variable sights.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vehicle_predictViaPlayer</code></summary>

Predict this vehicle if the player's data says they're driving this vehicle...rather than checking if this vehicle has a driver

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_EnableFixedAspectScaling</code></summary>

Enables fixed screen size for vgui elements

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawPolyShapes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawfocus</code></summary>

Report which panel is under the mouse.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawfocus</code></summary>

Report which panel is under the mouse.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawkeyfocus</code></summary>

Report which panel has keyboard focus.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree</code></summary>

Draws the vgui panel hiearchy to the specified depth level.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_bounds</code></summary>

Show panel bounds.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_draw_selected</code></summary>

Highlight the selected panel

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_freeze</code></summary>

Set to 1 to stop updating the vgui_drawtree view.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_hidden</code></summary>

Draw the hidden panels.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_panelalpha</code></summary>

Show the panel alpha values in the vgui_drawtree view.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_panelptr</code></summary>

Show the panel pointer values in the vgui_drawtree view.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_popupsonly</code></summary>

Draws the vgui popup list in hierarchy(1) or most recently used(2) order.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_render_order</code></summary>

List the vgui_drawtree panels in render order.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_scheme</code></summary>

Show scheme file for each panel

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_visible</code></summary>

Draw the visible panels.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_interactive</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_noquads</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>vgui_notext</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>vgui_resize_on_resolution_change</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_show_glyph_miss</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_simulate_during_bone_setup</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>video_menu_uiscript_reset</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewDrift</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_ads_delay_debounce_time</code></summary>

Time between zoom-out and zoom-in before viewdrift_ads_delay is reset.

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base1_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base1_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base1_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base2_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base2_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base2_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_scaler_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_scaler_base</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_scaler_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_scaler_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_shifter_amp</code></summary>



default: `"0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_shifter_freq</code></summary>



default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_shifter_phase</code></summary>



default: `"1.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base1_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base1_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base1_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base2_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base2_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base2_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_scaler_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_scaler_base</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_scaler_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_scaler_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_shifter_amp</code></summary>



default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_shifter_freq</code></summary>



default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_shifter_phase</code></summary>



default: `"-0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>view_offset_entity_enable</code></summary>

Whether to apply camera animations from the view offset entity

default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>viewangle_debug</code></summary>



default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>viewangles_simpler</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodelShake</code></summary>

Enables viewmodel shake.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodelShake_sourceRollRange</code></summary>

The range of weapon kick roll that will be sampled for viewmodel shake.

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_attachment_fov_fix</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_bounds_draw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_bounds_draw_lock</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_selfshadow</code></summary>

Set whether to use viewmodel self shadow

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_selfshadow_debug_2d</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_selfshadow_tightbounds</code></summary>

Viewmodel bounds are sliced by Main view frustum

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewportscale</code></summary>

Scale down the main viewport (to reduce GPU impact on CPU profiling)

default: `"1.0"`  
flags: `0x80000`  
min value: `0.0015625`  
max value: `2`  
</details>
<details>
<summary><code>viewpunch_base_springConstantX</code></summary>

Default. Bigger number increases the speed at which the view corrects.

default: `"65.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springConstantY</code></summary>

Default. Bigger number increases the speed at which the view corrects.

default: `"65.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springConstantZ</code></summary>

Default. Bigger number increases the speed at which the view corrects.

default: `"65.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springDampingX</code></summary>

Default. Bigger number makes the response more damped.

default: `"9.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springDampingY</code></summary>

Default. Bigger number makes the response more damped.

default: `"9.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springDampingZ</code></summary>

Default. Bigger number makes the response more damped.

default: `"9.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>violence_ablood</code></summary>

Draw alien blood

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_ablood</code></summary>

Draw alien blood

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_agibs</code></summary>

Show alien gib entities

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_agibs</code></summary>

Show alien gib entities

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_hblood</code></summary>

Draw human blood

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_hblood</code></summary>

Draw human blood

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_hgibs</code></summary>

Show human gib entities

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_hgibs</code></summary>

Show human gib entities

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>visible_ent_cone_debug_duration_client</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_absTriggerAmount</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_allow_mute_self</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_avggain</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_clientdebug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_debugAddSecondTalker</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_debugThresholds</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_debugfeedback</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_decimate_at_bytes</code></summary>



default: `"22050"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_decimate_rate</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_enabled</code></summary>

Toggle voice transmit and receive.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>voice_energyPerZeroThreshold</code></summary>



default: `"8000"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_energyThreshold</code></summary>



default: `"12000"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_forcemicrecord</code></summary>



default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_inputfromfile</code></summary>

Get voice input from 'voice_input.wav' rather than from the microphone.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>voice_late_update</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_loopback</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>voice_maxgain</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_minEnergyPerZeroThreshold</code></summary>



default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_mixer_boost</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_mixer_mute</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_mixer_volume</code></summary>



default: `"1.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_modenable</code></summary>

Enable/disable voice in this mod.

default: `"1"`  
flags: `0x40000080`  
</details>
<details>
<summary><code>voice_noxplat</code></summary>

Only send voice data to players on the same platform as the talker

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_profile</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_recordtofile</code></summary>

Record mic data and decompressed voice data into 'voice_micdata.wav' and 'voice_decompressed.wav'

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>voice_scale</code></summary>



default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_showchannels</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_showincoming</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_threshold_delay</code></summary>



default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_triggerCrossingRate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_triggerRate</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_turn_off_new_filters</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_vox</code></summary>

Voice chat uses a vox-style always on

default: `"1"`  
flags: `0x80`  
max value: `1`  
</details>
<details>
<summary><code>voice_writevoices</code></summary>

Saves each speaker's voice data into separate .wav files


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_xsend_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_zeroCrossingThreshold</code></summary>



default: `"0.02"`  
flags: `0x2`  
</details>
<details>
<summary><code>vortex_damageimpulsescale</code></summary>

Scales impulse force from bullets when using the vortex

default: `"0.5"`  
flags: `0x6000`  
</details>
<details>
<summary><code>vprof_server_spike_threshold</code></summary>



default: `"999.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vprof_server_thread</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vscript_ui_do_delay_init</code></summary>



default: `"1"`  
flags: `0x12`  
</details>
<details>
<summary><code>vsm_culling</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>vsm_ignore_edge_planes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vsm_ignore_face_planes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vx_do_not_throttle_events</code></summary>

Force VXConsole updates every frame; smoother vprof data but at a slight (~0.2ms) perf cost.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>wall_climb_pose_paramteter_hands_enabled</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallclimb_vertical_gain_reduction</code></summary>

Amount of height the player loses when falling off a wall climb that can't be regained by future wall climbs before touching the ground.

default: `"128"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_angleChangeMinCos</code></summary>

Cosine of maximum angle the wall can change away from you without falling off

default: `"0.8"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_avoid_wall_top_decel</code></summary>

Deceleration applied to prevent the player from wall running too close to the top of a wall and falling off

default: `"3000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_curveDebug</code></summary>

Draws debugging information for wallrun curves

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_curveEnable</code></summary>

Enables usage of wallrun curve hints

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_debug</code></summary>

Shows wall run debug info

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_enable</code></summary>

Enables wall running

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_fallAwaySpeed</code></summary>

Velocity away from the wall when falling off

default: `"70.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_hangStopTime</code></summary>

Length of time to come to a stop when zooming

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_hangslipduration</code></summary>

Time it takes for slipping to become completely gravity based

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_hangslipstarttime</code></summary>

Time wall hanging before you start to slip down

default: `"3.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_hangslipvel</code></summary>

Impulse downward when slipping starts while wall hanging

default: `"70"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_maxViewTilt</code></summary>

Amount of roll applied to the view in degrees while wall running

default: `"10.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_minAngle_air</code></summary>

Angle at which you can start wall running when hitting a wall from a jump (0 to 180)

default: `"180.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_noInputSlipFrac</code></summary>

Min fraction of slip behavior when not pushing in any direction (applies more gravity)

default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_pushAwayFallOffTime</code></summary>

Pushing away from the wall for this many seconds causes you to fall off

default: `"0.05"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_repelEnable</code></summary>

Enables repelling players from walls they have jumped off of

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_repelSoftness</code></summary>

Softness of wall jump repel: higher values make it easier for players to reduce their speed away from the wall

default: `"5.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_repelTimeMax</code></summary>

Time after jumping off the wall that player is no longer repelled from the wall

default: `"0.4"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_repelTimeMin</code></summary>

Time after jumping off the wall that player is repelled from the wall

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_retry_interval</code></summary>

Length of time between checking for the ability to wallrun after hitting a wall in air movement

default: `"0.07"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_rotateMaxRate</code></summary>

Maximum rotation speed around a wall in radians per second; avoids sticking to walls that do tight curves

default: `"3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_sameWallDist</code></summary>

Within this distance of the previous wall run, wall run is prevented at a higher point on the same wall

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_sameWallDot</code></summary>

Dot product threshold for preventing wall running on the same wall twice

default: `"0.9"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_sameWallSlope</code></summary>

Beyond wallrun_samewalldist, wall running is permitted at higher points with this slope

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_slipduration</code></summary>

Time it takes for slipping to become completely gravity based

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_slipslowdown</code></summary>

Fraction of velocity lost when slipping starts

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_slipstarttime</code></summary>

Time wall running before you start to slip down

default: `"1.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_slipvel</code></summary>

Impulse downward when slipping starts

default: `"70"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_strengthLossEnd</code></summary>

Number of wall runs at which point upward strength is fully lost (scales upWallBoost, jumpUpSpeed, and gravityRampUpTime to zero)

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_strengthLossStart</code></summary>

Number of wall runs allowed before starting to lose upward strength (scales upWallBoost, jumpUpSpeed, and gravityRampUpTime)

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_upwardAutoPush</code></summary>

The amount of automatic up-the-wall input applied when the player pushes forward along the wall. Helps to fight gravity when pushing forward.

default: `"0.65"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_viewTiltPredictTime</code></summary>

Time before you start wall running where your view starts tilting. Predicts upcoming wall running

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_viewTiltSpeed</code></summary>

Speed at which the view tilts while wall running

default: `"6.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>was_loaded</code></summary>

Current game from a restore?

default: `""`  
flags: `0x12002`  
</details>
<details>
<summary><code>weaponFastHolsterScale</code></summary>

Scales holster animations if swapping to a weapon with "fast_swap_to" enabled.

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weaponSwitch3p_checkNewWeapon</code></summary>

Only play 3p weapon switch if there is a new weapon.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weaponSwitch3p_onHolster</code></summary>

Start third person weapon switch animation as soon as the current weapon starts being holstered.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_auto_swap_ordnance_no_ammo</code></summary>

If you touch a new ordnance weapon with no ammo in your current it will auto replace it

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_debugScript</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_doIdleForSurvivalMelee</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_friendly_fire_prevent_ui</code></summary>

UI to show on friendly fire prevention

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_meleeButtonPressProtection</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_parentingFixLerp</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_pickup_allow_dupes</code></summary>

Whether or not you are allowed 2 of the same weapon in your inventory

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_poseParamMaxDistance</code></summary>



default: `"6000"`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_render_with_fastpath</code></summary>

Allow weapons to draw using the fast path.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_setting_autocycle_on_empty</code></summary>



default: `"1"`  
flags: `0x41000200`  
</details>
<details>
<summary><code>weapon_sprint_raise_delay</code></summary>

Enables weapon delay between sprint and shooting

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weaponx_predicting_client_only_optimization</code></summary>

Enable/disable weaponx optimization for burst fire, shot count and charge data only being sent to predicting client

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weaponx_smartammo_data_optimization</code></summary>

Enable/disable weaponx smartammo data optimization. Only applies with net_optimize_weapons >= 2

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_debug</code></summary>

Debugs search for window hints

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_fov_down</code></summary>

Window hints below this vertical FOV will be ignored

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_fov_horz</code></summary>

Window hints beyond this horizontal FOV will be ignored

default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_fov_up</code></summary>

Window hints above this vertical FOV will be ignored

default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_keyboard_fov_horz</code></summary>

Window hints beyond this horizontal FOV will be ignored

default: `"7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_lookahead_time</code></summary>

Lookahead prediction time for window checks

default: `"0.8"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_max_horz_vel_change_dot</code></summary>

Min dot product of velocity change when adjusting for windows

default: `"0.966f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_max_vel_change_down</code></summary>

Max removed vertical velocity when adjusting for windows

default: `"150"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_max_vel_change_up</code></summary>

Max added vertical velocity when adjusting for windows

default: `"80"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_min_horz_vel</code></summary>

Horizontal velocity is increased to at least this when adjusting for windows

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_permissive_max_horz_vel_change_dot</code></summary>

Min dot product of velocity change when adjusting for windows (off grapple)

default: `"0.88f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_permissive_max_vel_change_down</code></summary>

Max removed vertical velocity when adjusting for windows (off grapple)

default: `"300"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_permissive_max_vel_change_up</code></summary>

Max added vertical velocity when adjusting for windows (off grapple)

default: `"300"`  
flags: `0x2002`  
</details>
<details>
<summary><code>z_ragdoll_impact_strength</code></summary>



default: `"500"`  
flags: `0x2`  
</details>
<details>
<summary><code>zipline_check_usable_before_deploy</code></summary>

For Zipline grenades, check if the zipline will be usable before allowing the player to deploy it.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>zipline_cooldown_time_0</code></summary>

Zipline Cooldown.

default: `"0.1"`  
flags: `0x2`  
min value: `0`  
</details>
<details>
<summary><code>zipline_cooldown_time_1</code></summary>

Zipline Cooldown.

default: `"0.2"`  
flags: `0x2`  
min value: `0`  
</details>
<details>
<summary><code>zipline_cooldown_time_2</code></summary>

Zipline Cooldown.

default: `"1.0"`  
flags: `0x2`  
min value: `0`  
</details>
<details>
<summary><code>zipline_cooldown_time_3</code></summary>

Zipline Cooldown.

default: `"3.0"`  
flags: `0x2`  
min value: `0`  
</details>
<details>
<summary><code>zipline_cooldown_time_4</code></summary>

Zipline Cooldown.

default: `"5.0"`  
flags: `0x2`  
min value: `0`  
</details>
<details>
<summary><code>zipline_fade_dist</code></summary>



default: `"6000"`  
flags: `0x2`  
</details>
<details>
<summary><code>zipline_subdiv_lod_dist_base</code></summary>

The base distance that ziplines will begin using zipline_subdiv_slices_lod. This value is scaled by the diameter of the zipline.

default: `"150"`  
flags: `0x2`  
</details>
<details>
<summary><code>zipline_subdiv_slices</code></summary>

Zipline subdivision amount around the rope, affects roundedness.

default: `"6"`  
flags: `0x2`  
min value: `1`  
max value: `12`  
</details>
<details>
<summary><code>zipline_subdiv_slices_lod</code></summary>

Zipline subdivision amount around the rope when lod is active, affects roundedness.

default: `"4"`  
flags: `0x2`  
min value: `0`  
max value: `12`  
</details>
<details>
<summary><code>zipline_subdiv_stacks</code></summary>

Zipline subdivision amount between each zipline node.

default: `"6"`  
flags: `0x2`  
min value: `1`  
max value: `24`  
</details>

### Addresses

```
r5apex.exe!0x01ea03e0 ConVar 
r5apex.exe!0x0582d710 ConVar Allow_auto_Party
r5apex.exe!0x011961c0 ConVar BlendBonesMode
r5apex.exe!0x012a9430 ConVar CrossPlay_user_optin
r5apex.exe!0x012dd230 ConVar DigiCertGlobalRoot_usable
r5apex.exe!0x01c63c40 ConVar DoorSoundPrefixDouble
r5apex.exe!0x01c98d20 ConVar DoorSoundPrefixSingle
r5apex.exe!0x012dd190 ConVar EADP_RTM_Productid
r5apex.exe!0x012d63c0 ConVar EAPD_search_accountname_count
r5apex.exe!0x01e65a00 ConVar ScriptDisallowedToUsePersistenceOnSP
r5apex.exe!0x01e67690 ConVar ScriptSaveAllowed
r5apex.exe!0x011aaf30 ConVar StreamMicDisabled
r5apex.exe!0x012dd2d0 ConVar TLS_trust_cert
r5apex.exe!0x011ab750 ConVar TalkIsStream
r5apex.exe!0x01c4a260 ConVar TextDataFromCommunityOnlyInLobby
r5apex.exe!0x011ac590 ConVar VoiceDataFromCommunityOnlyInLobby
r5apex.exe!0x011ab570 ConVar VoiceNeedsReset
r5apex.exe!0x01c975e0 ConVar add_to_parent_realms_default
r5apex.exe!0x01e87410 ConVar ai_titan_grapple_max_len
r5apex.exe!0x01c9bb40 ConVar airslowmo_enabled
r5apex.exe!0x01c92780 ConVar airslowmo_enter_time
r5apex.exe!0x01c96c10 ConVar airslowmo_ground_immediate_end
r5apex.exe!0x01c665e0 ConVar airslowmo_leave_time
r5apex.exe!0x01c97aa0 ConVar airslowmo_scripted_speed
r5apex.exe!0x01c9b9a0 ConVar airslowmo_when_hovering
r5apex.exe!0x01895ea0 ConVar animEvent_debug
r5apex.exe!0x01897360 ConVar animEvent_debugEnt
r5apex.exe!0x01897880 ConVar animEvent_debug_cl
r5apex.exe!0x01e7a1b0 ConVar anim_estimateVelocity
r5apex.exe!0x01e7aa10 ConVar anim_playerMovementAngleMargin
r5apex.exe!0x01e7a2f0 ConVar anim_player_ragdoll_fix
r5apex.exe!0x01ca7bb0 ConVar anim_print_transition_overflow
r5apex.exe!0x01e7a8d0 ConVar anim_runGestureAnimEventsToCompletionOnReset_client
r5apex.exe!0x01e7a070 ConVar anim_showstate
r5apex.exe!0x01e7a430 ConVar anim_showstatelog
r5apex.exe!0x01caaae0 ConVar anim_transitionsequences
r5apex.exe!0x01cae980 ConVar anim_view_entity_third_person_camera_use_move_parent
r5apex.exe!0x012d87c0 ConVar announcement
r5apex.exe!0x012d8d60 ConVar announcementImage
r5apex.exe!0x012d89a0 ConVar announcementVersion
r5apex.exe!0x012aa660 ConVar assetdownloads_desiredState
r5apex.exe!0x012aa700 ConVar assetdownloads_enabled
r5apex.exe!0x012aa5c0 ConVar assetdownloads_hostname
r5apex.exe!0x011a7af0 ConVar async_serialize
r5apex.exe!0x01c961b0 ConVar automantle_backoff_anim_maxfrac
r5apex.exe!0x01c63d80 ConVar automantle_cooldown
r5apex.exe!0x01c946b0 ConVar automantle_dangle_required_space
r5apex.exe!0x01c66680 ConVar automantle_debug
r5apex.exe!0x01c9cff0 ConVar automantle_disable_hang
r5apex.exe!0x01c95a20 ConVar automantle_enable
r5apex.exe!0x01c97b40 ConVar automantle_forwarddist
r5apex.exe!0x01c97420 ConVar automantle_gun_enable_height
r5apex.exe!0x01c81cb0 ConVar automantle_height_above
r5apex.exe!0x01c98e20 ConVar automantle_height_below
r5apex.exe!0x01c68cf0 ConVar automantle_height_level
r5apex.exe!0x01c69270 ConVar automantle_jumpoff_anim_maxfrac
r5apex.exe!0x01c95980 ConVar automantle_jumpoff_duration
r5apex.exe!0x01c9a8f0 ConVar automantle_max_frac
r5apex.exe!0x01c94a20 ConVar automantle_maxangle_push
r5apex.exe!0x01c97720 ConVar automantle_maxangle_view
r5apex.exe!0x01c671a0 ConVar automantle_min_frac
r5apex.exe!0x01c693b0 ConVar automantle_mindist
r5apex.exe!0x01c97380 ConVar automantle_rest_frac
r5apex.exe!0x01c69310 ConVar automantle_rest_frac_below
r5apex.exe!0x01c9cf50 ConVar automantle_searchdist
r5apex.exe!0x0189b8a0 ConVar automantle_view_correction_speed
r5apex.exe!0x0189b780 ConVar automantle_view_high_yaw_max
r5apex.exe!0x0189c890 ConVar automantle_view_pitch_max
r5apex.exe!0x0189ed20 ConVar automantle_view_pitch_min
r5apex.exe!0x0189ab40 ConVar automantle_view_yaw_max
r5apex.exe!0x01c96e90 ConVar automantle_wallrun_maxangle_view
r5apex.exe!0x01c63450 ConVar baseanimatingoverlay_playbackRateThreshold
r5apex.exe!0x012aa290 ConVar baselines_print
r5apex.exe!0x01e82de0 ConVar bhit_enable
r5apex.exe!0x01e865f0 ConVar bhit_reliable
r5apex.exe!0x011ac8b0 ConVar bink_materials_enabled
r5apex.exe!0x01c5fac0 ConVar bink_preload_videopanel_movies
r5apex.exe!0x0189ee60 ConVar boost_jetwash_prediction_factor
r5apex.exe!0x012a6ed0 ConVar bot_lagOut
r5apex.exe!0x0119a650 ConVar budget_animatingEntities
r5apex.exe!0x01198500 ConVar budget_animationOverlayEntities
r5apex.exe!0x01199a10 ConVar budget_combatCharEntities
r5apex.exe!0x011982b0 ConVar budget_weaponEntities
r5apex.exe!0x01199e90 ConVar budget_ziplineEntities
r5apex.exe!0x01e65940 ConVar bug_reproNum
r5apex.exe!0x0119b550 ConVar buildcubemaps_async
r5apex.exe!0x0119aca0 ConVar buildcubemaps_index
r5apex.exe!0x01198ac0 ConVar buildcubemaps_pvs_start_early
r5apex.exe!0x0119af90 ConVar buildcubemaps_single_step
r5apex.exe!0x01199f30 ConVar building_cubemaps
r5apex.exe!0x01e8cb20 ConVar bulletPredictionDebug
r5apex.exe!0x01ca69f0 ConVar bullet_trace_test_debug
r5apex.exe!0x01ca8c80 ConVar bullet_trace_test_enable
r5apex.exe!0x01c443e0 ConVar c_dropship_ground_fx_dist_interval
r5apex.exe!0x018b0010 ConVar c_dropship_ground_fx_time_interval
r5apex.exe!0x018ac670 ConVar c_dropship_rope_debug
r5apex.exe!0x018b6ab0 ConVar c_dropship_rope_events
r5apex.exe!0x01c39c90 ConVar c_dropship_rope_magnitude
r5apex.exe!0x018aa730 ConVar c_dropship_rope_range
r5apex.exe!0x01c49540 ConVar c_maxdistance
r5apex.exe!0x01c467c0 ConVar c_maxpitch
r5apex.exe!0x01c57a90 ConVar c_maxyaw
r5apex.exe!0x01c4b4f0 ConVar c_mindistance
r5apex.exe!0x01c4e990 ConVar c_minpitch
r5apex.exe!0x01c50850 ConVar c_minyaw
r5apex.exe!0x01c54330 ConVar c_orthoheight
r5apex.exe!0x01c45de0 ConVar c_orthowidth
r5apex.exe!0x01c59530 ConVar c_thirdpersonshoulderaimdistADS_110
r5apex.exe!0x01c595d0 ConVar c_thirdpersonshoulderaimdistADS_70
r5apex.exe!0x01c59670 ConVar c_thirdpersonshoulderaimdistADS_90
r5apex.exe!0x01c597b0 ConVar c_thirdpersonshoulderaimdist_110
r5apex.exe!0x01c59850 ConVar c_thirdpersonshoulderaimdist_70
r5apex.exe!0x01c59710 ConVar c_thirdpersonshoulderaimdist_90
r5apex.exe!0x01c56b70 ConVar c_thirdpersonshoulderdist
r5apex.exe!0x01c59490 ConVar c_thirdpersonshouldergetsviewpunch
r5apex.exe!0x01c59990 ConVar c_thirdpersonshoulderheight
r5apex.exe!0x01c598f0 ConVar c_thirdpersonshoulderoffset
r5apex.exe!0x01897040 ConVar c_threadedAnimPostData
r5apex.exe!0x01c4a7e0 ConVar cam_collision
r5apex.exe!0x01c51b40 ConVar cam_idealdelta
r5apex.exe!0x01c48140 ConVar cam_idealdist
r5apex.exe!0x01c54210 ConVar cam_ideallag
r5apex.exe!0x01c57e30 ConVar cam_idealpitch
r5apex.exe!0x01c53ed0 ConVar cam_idealyaw
r5apex.exe!0x01c46380 ConVar cam_pitchLock_feetRelative
r5apex.exe!0x01c4bc90 ConVar cam_pitchlock_on
r5apex.exe!0x01c55c10 ConVar cam_pitchlock_period
r5apex.exe!0x01c483a0 ConVar cam_pitchlock_phase
r5apex.exe!0x01c56510 ConVar cam_pitchlock_pitchBase
r5apex.exe!0x01c48d50 ConVar cam_pitchlock_pitchRange
r5apex.exe!0x01c45ba0 ConVar cam_pitchlock_pitchWiggleRoom
r5apex.exe!0x01c59350 ConVar cam_player_viewheight_scale
r5apex.exe!0x01c49400 ConVar cam_showangles
r5apex.exe!0x01c55cb0 ConVar cc_captiontrace
r5apex.exe!0x01c4f960 ConVar cc_global_norepeat
r5apex.exe!0x01c589b0 ConVar cc_linger_time
r5apex.exe!0x01c4d620 ConVar cc_max_duration
r5apex.exe!0x01c4db60 ConVar cc_minvisibleitems
r5apex.exe!0x01c518e0 ConVar cc_predisplay_time
r5apex.exe!0x01c55ed0 ConVar cc_rui
r5apex.exe!0x01c4bfd0 ConVar cc_text_size
r5apex.exe!0x01c4a0a0 ConVar cc_timeshift_norepeat
r5apex.exe!0x01898600 ConVar chasecam_distanceMax_override
r5apex.exe!0x012aa7a0 ConVar chat_rules_log_details
r5apex.exe!0x011ab890 ConVar chatroom_console_ptt
r5apex.exe!0x012d92d0 ConVar chatroom_debug
r5apex.exe!0x012dcab0 ConVar chatroom_doRealNameLookups
r5apex.exe!0x012dceb0 ConVar chatroom_min_status_send_interval
r5apex.exe!0x01e7b630 ConVar chatroom_nameLength
r5apex.exe!0x01e7c430 ConVar chatroom_namePaddingX
r5apex.exe!0x01e7c390 ConVar chatroom_nameWidth
r5apex.exe!0x012aad40 ConVar chatroom_onlyWhenActive
r5apex.exe!0x012dca10 ConVar chatroom_sendServerMutes
r5apex.exe!0x01e7bfd0 ConVar chatroom_useSlopSpace
r5apex.exe!0x012dcb50 ConVar chatroom_voiceMode
r5apex.exe!0x01e7c610 ConVar chatroom_voiceMode
r5apex.exe!0x01c60ca0 ConVar cheap_captions_fadetime
r5apex.exe!0x01c60480 ConVar cheap_captions_test
r5apex.exe!0x011ac9f0 ConVar chroma_enable
r5apex.exe!0x0189a960 ConVar cl_NotifyAllLevelAssetsLoaded_endframe
r5apex.exe!0x01c3b730 ConVar cl_RunClientConnectScripts_Before_ProcessOnDataChangedEvents
r5apex.exe!0x018970e0 ConVar cl_SetupAllBones
r5apex.exe!0x018abf30 ConVar cl_ShowBoneSetupEnts
r5apex.exe!0x012a59d0 ConVar cl_adjustTimeEntsPerJob
r5apex.exe!0x01c62180 ConVar cl_aggregate_particles
r5apex.exe!0x018ae9f0 ConVar cl_allowABSCalculationDuringSnapshotScriptCalls
r5apex.exe!0x018a6e50 ConVar cl_allowABSDuringSnapshotScriptCalls
r5apex.exe!0x01895f40 ConVar cl_allowAnimsToInterpolateBackward
r5apex.exe!0x01e656c0 ConVar cl_always_draw_3p_player
r5apex.exe!0x01897700 ConVar cl_always_ragdoll_radius
r5apex.exe!0x01c56470 ConVar cl_anglespeedkey
r5apex.exe!0x018981c0 ConVar cl_anim_blend_transition_dist
r5apex.exe!0x01896740 ConVar cl_anim_detail_dist
r5apex.exe!0x01896600 ConVar cl_anim_face_dist
r5apex.exe!0x01896e00 ConVar cl_anim_sequence_transition_full_weight_optimization
r5apex.exe!0x01898260 ConVar cl_anim_sounds_seek
r5apex.exe!0x01895120 ConVar cl_approx_footstep_origin
r5apex.exe!0x01c58190 ConVar cl_approx_tracer_origin
r5apex.exe!0x01c451c0 ConVar cl_async_bone_setup
r5apex.exe!0x01e91100 ConVar cl_base_entity_effect_lock
r5apex.exe!0x01895c20 ConVar cl_bones_incremental_blend
r5apex.exe!0x01895cc0 ConVar cl_bones_incremental_transform
r5apex.exe!0x01897920 ConVar cl_bones_oldhack
r5apex.exe!0x01c9c510 ConVar cl_bounds_show_errors
r5apex.exe!0x01ca40a0 ConVar cl_burninggibs
r5apex.exe!0x0119b6e0 ConVar cl_clock_correction
r5apex.exe!0x01199ab0 ConVar cl_clock_correction_ahead_correct_interval
r5apex.exe!0x011988b0 ConVar cl_clock_correction_behind_correct_interval
r5apex.exe!0x0119a110 ConVar cl_clock_correction_force_server_tick
r5apex.exe!0x0126e310 ConVar cl_cmdbackup
r5apex.exe!0x0126d3e0 ConVar cl_cmdrate
r5apex.exe!0x012d7400 ConVar cl_configversion
r5apex.exe!0x012d6da0 ConVar cl_configversion_dummy
r5apex.exe!0x01e82570 ConVar cl_cull_weapon_fx
r5apex.exe!0x011a91d0 ConVar cl_dataBlockFragmentPL
r5apex.exe!0x01e67ab0 ConVar cl_deathhints_enabled
r5apex.exe!0x01e90ba0 ConVar cl_debugClientEntities
r5apex.exe!0x01ca68e0 ConVar cl_debug_deferred_trace
r5apex.exe!0x01ca8960 ConVar cl_debug_deferred_trace_overlay
r5apex.exe!0x01c562d0 ConVar cl_debug_model_fx_sounds
r5apex.exe!0x01c66de0 ConVar cl_decal_alwayswhite
r5apex.exe!0x01c9c880 ConVar cl_decal_backoff
r5apex.exe!0x01e65620 ConVar cl_deferred_effects
r5apex.exe!0x01ca6150 ConVar cl_deferred_trace_normal_priority
r5apex.exe!0x018b63e0 ConVar cl_demoviewoverride
r5apex.exe!0x01898120 ConVar cl_disable_ragdolls
r5apex.exe!0x018a6db0 ConVar cl_disable_splitscreen_cpu_level_cfgs_in_pip
r5apex.exe!0x011acc50 ConVar cl_disconnectOnTooManySnapshotFrames
r5apex.exe!0x01e8e030 ConVar cl_doNetworkAsserts
r5apex.exe!0x011acbb0 ConVar cl_doRecreateEnts
r5apex.exe!0x0189a3e0 ConVar cl_draw_player_model
r5apex.exe!0x01c3ba50 ConVar cl_drawhud
r5apex.exe!0x01cb0530 ConVar cl_drawmonitors
r5apex.exe!0x01897400 ConVar cl_ejectbrass
r5apex.exe!0x01c5b9b0 ConVar cl_enable_remote_splitscreen
r5apex.exe!0x011acf70 ConVar cl_entCreateDeleteDebug
r5apex.exe!0x018acfd0 ConVar cl_events_ignore_invalidate
r5apex.exe!0x011a8070 ConVar cl_failremoteconnections
r5apex.exe!0x01c57b30 ConVar cl_fasttempentcollision
r5apex.exe!0x01e90ce0 ConVar cl_flip_vis_bits
r5apex.exe!0x011acd90 ConVar cl_flushentitypacket
r5apex.exe!0x01897a60 ConVar cl_footstep_event_max_dist
r5apex.exe!0x01897220 ConVar cl_footstep_event_max_dist_titan
r5apex.exe!0x012a4ad0 ConVar cl_forceAdjustTime
r5apex.exe!0x012de590 ConVar cl_fovScale
r5apex.exe!0x012de4f0 ConVar cl_gib_allow
r5apex.exe!0x01c3ac00 ConVar cl_gib_attack_dir_scale
r5apex.exe!0x0189fb00 ConVar cl_gib_lifetime
r5apex.exe!0x01c5d5c0 ConVar cl_idealpitchscale
r5apex.exe!0x0126e800 ConVar cl_ignorepackets
r5apex.exe!0x01e913c0 ConVar cl_interp_all
r5apex.exe!0x012a4990 ConVar cl_interpolate
r5apex.exe!0x01e90ec0 ConVar cl_interpolate
r5apex.exe!0x01895d60 ConVar cl_interpolateSoAllAnimsLoop
r5apex.exe!0x01c442a0 ConVar cl_interpolation_before_prediction
r5apex.exe!0x011aa970 ConVar cl_isUnderAge
r5apex.exe!0x0126e9a0 ConVar cl_is_softened_locale
r5apex.exe!0x01cabd30 ConVar cl_jiggle_bone_debug
r5apex.exe!0x01cabbf0 ConVar cl_jiggle_bone_debug_pitch_constraints
r5apex.exe!0x01cabeb0 ConVar cl_jiggle_bone_debug_yaw_constraints
r5apex.exe!0x01cabc90 ConVar cl_jiggle_bone_invert
r5apex.exe!0x01cabdd0 ConVar cl_jiggle_bone_sanity
r5apex.exe!0x012a47b0 ConVar cl_keepPersistentDataOnDisconnect
r5apex.exe!0x01c5ea60 ConVar cl_lagcompensation
r5apex.exe!0x0126f5c0 ConVar cl_language
r5apex.exe!0x01c44840 ConVar cl_leafsystemvis
r5apex.exe!0x01e92600 ConVar cl_lerpIfChildrenLerp
r5apex.exe!0x011a9450 ConVar cl_loadBspFromServerInfo
r5apex.exe!0x011a60e0 ConVar cl_loadPostProcessShadersEarly
r5apex.exe!0x011a56a0 ConVar cl_loadStaticPropsInJob
r5apex.exe!0x012a57f0 ConVar cl_matchmaking_timeout
r5apex.exe!0x01c53ab0 ConVar cl_model_fx_gib_cull_front_dist
r5apex.exe!0x01c53630 ConVar cl_model_fx_gib_cull_radius
r5apex.exe!0x01c56810 ConVar cl_mouseenable
r5apex.exe!0x0126f0d0 ConVar cl_move_use_dt
r5apex.exe!0x0126d820 ConVar cl_noTimeoutLocalHost
r5apex.exe!0x0126f7a0 ConVar cl_overrideEventTimes
r5apex.exe!0x01e64dc0 ConVar cl_parallelParticlePreDrawWork
r5apex.exe!0x01895fe0 ConVar cl_parallel_clientside_animations
r5apex.exe!0x01cab270 ConVar cl_particle_batch_mode
r5apex.exe!0x012de310 ConVar cl_particle_fallback_base
r5apex.exe!0x012de630 ConVar cl_particle_fallback_multiplier
r5apex.exe!0x01c61140 ConVar cl_particle_limiter_display_killed
r5apex.exe!0x01e96fd0 ConVar cl_particle_limiter_hide_killable
r5apex.exe!0x01c5c2e0 ConVar cl_particle_limiter_max_particle_count
r5apex.exe!0x01c5c510 ConVar cl_particle_limiter_max_system_count
r5apex.exe!0x01c5d6e0 ConVar cl_particle_limiter_min_kill_distance
r5apex.exe!0x01c60e60 ConVar cl_particle_limiter_overlay
r5apex.exe!0x01c5cf30 ConVar cl_particle_max_count
r5apex.exe!0x01c620e0 ConVar cl_particle_sim_fallback_base_multiplier
r5apex.exe!0x01c5b410 ConVar cl_particle_sim_fallback_threshold_ms
r5apex.exe!0x01c5fc80 ConVar cl_particle_snoozetime
r5apex.exe!0x01c5da60 ConVar cl_particles_show_bbox
r5apex.exe!0x01c614e0 ConVar cl_particles_show_bbox_name
r5apex.exe!0x01c5e420 ConVar cl_particles_show_controlpoints
r5apex.exe!0x01c629f0 ConVar cl_pclass
r5apex.exe!0x01c60200 ConVar cl_pdump
r5apex.exe!0x01c5b550 ConVar cl_phys_maxticks
r5apex.exe!0x01c5e920 ConVar cl_phys_show_active
r5apex.exe!0x01c5ee80 ConVar cl_phys_timescale
r5apex.exe!0x01c617c0 ConVar cl_physics_invalidate_ents
r5apex.exe!0x01c61a20 ConVar cl_physics_maxvelocity
r5apex.exe!0x01cac3c0 ConVar cl_physicsshadowupdate_render
r5apex.exe!0x01c56c10 ConVar cl_pitchspeed
r5apex.exe!0x0126ef10 ConVar cl_playback_screenshots
r5apex.exe!0x0189ceb0 ConVar cl_player_fullupdate_predicted_origin_fix
r5apex.exe!0x018ae3e0 ConVar cl_player_resource_always_connected
r5apex.exe!0x01c977c0 ConVar cl_player_touch_triggers
r5apex.exe!0x011ace30 ConVar cl_postSnapshotTransitionBlockCount
r5apex.exe!0x01e91cc0 ConVar cl_preSnapshotTransitionBlockCount
r5apex.exe!0x01cab3b0 ConVar cl_pred_error_verbose
r5apex.exe!0x01c5dd80 ConVar cl_pred_optimize
r5apex.exe!0x0126ece0 ConVar cl_predict
r5apex.exe!0x0189f390 ConVar cl_predict_basetoggles
r5apex.exe!0x01c5b7d0 ConVar cl_predict_cmdlimit
r5apex.exe!0x01c61fa0 ConVar cl_predict_error_icon_duration
r5apex.exe!0x01c5cfd0 ConVar cl_predict_error_icon_show
r5apex.exe!0x01c60700 ConVar cl_predict_error_icon_threshold_angle
r5apex.exe!0x01c5bd60 ConVar cl_predict_error_icon_threshold_dist
r5apex.exe!0x01cac250 ConVar cl_predict_motioncontrol
r5apex.exe!0x01c62f10 ConVar cl_predict_viewangles
r5apex.exe!0x01c5bcc0 ConVar cl_prediction_error_timestamps
r5apex.exe!0x01c60120 ConVar cl_predictionlist
r5apex.exe!0x01c5de20 ConVar cl_predictweapons
r5apex.exe!0x01e7fd90 ConVar cl_prevent_weapon_text_hints
r5apex.exe!0x01898080 ConVar cl_ragdoll_force_fade_time
r5apex.exe!0x018979c0 ConVar cl_ragdoll_force_fade_time_local_view_player
r5apex.exe!0x01caab80 ConVar cl_ragdoll_force_fade_time_on_moving_geo
r5apex.exe!0x01897520 ConVar cl_ragdoll_force_fade_time_titan
r5apex.exe!0x012de450 ConVar cl_ragdoll_maxcount
r5apex.exe!0x012de6d0 ConVar cl_ragdoll_self_collision
r5apex.exe!0x012a54f0 ConVar cl_replayDelayTolerance
r5apex.exe!0x018983a0 ConVar cl_requireAnimForAnimEventsHdr
r5apex.exe!0x012a51f0 ConVar cl_resend
r5apex.exe!0x012a5630 ConVar cl_resend_timeout
r5apex.exe!0x0126dfd0 ConVar cl_retire_low_priority_lights
r5apex.exe!0x018b1610 ConVar cl_runWeaponCloneThinkWhenHidden
r5apex.exe!0x01e67b90 ConVar cl_safearea
r5apex.exe!0x0126e420 ConVar cl_screenshotname
r5apex.exe!0x01c630f0 ConVar cl_scriptCompileAsync
r5apex.exe!0x01c5e760 ConVar cl_script_perf_dump_on_shutdown
r5apex.exe!0x01cad190 ConVar cl_shadowupdatespacing
r5apex.exe!0x018b5730 ConVar cl_showClanTags
r5apex.exe!0x01c53750 ConVar cl_show_splashes
r5apex.exe!0x01c5b5f0 ConVar cl_showerror
r5apex.exe!0x01c5beb0 ConVar cl_showerror_watchfield
r5apex.exe!0x01e8d400 ConVar cl_showfiredbullets
r5apex.exe!0x01c60520 ConVar cl_showfps
r5apex.exe!0x01c5d400 ConVar cl_showfps_altframetime
r5apex.exe!0x01c5f300 ConVar cl_showpausedimage
r5apex.exe!0x01c5b250 ConVar cl_showpos
r5apex.exe!0x011a7690 ConVar cl_showsounds
r5apex.exe!0x01c5dce0 ConVar cl_showtime
r5apex.exe!0x01896ea0 ConVar cl_simulateAllModelsRegardless
r5apex.exe!0x01e92940 ConVar cl_simulationtimefix
r5apex.exe!0x01895e00 ConVar cl_skipAnimEventsOnProps
r5apex.exe!0x01c61000 ConVar cl_skipfastpath
r5apex.exe!0x0189de00 ConVar cl_smooth
r5apex.exe!0x0189dd50 ConVar cl_smooth_debug
r5apex.exe!0x0189ec80 ConVar cl_smoothtime
r5apex.exe!0x01895b80 ConVar cl_threaded_bone_setup
r5apex.exe!0x01e91de0 ConVar cl_update_visibility_for_non_predicted_local_player_always
r5apex.exe!0x01c82630 ConVar cl_updatedirty_async
r5apex.exe!0x018ad110 ConVar cl_updatedirty_early
r5apex.exe!0x0126f660 ConVar cl_updaterate_mp
r5apex.exe!0x01c56ad0 ConVar cl_upspeed
r5apex.exe!0x0126f700 ConVar cl_useFutureSnapForEvents
r5apex.exe!0x012a5050 ConVar cl_useLobbyTypeForChatroom
r5apex.exe!0x0189e660 ConVar cl_view_cone
r5apex.exe!0x0189d070 ConVar cl_view_cone_debug
r5apex.exe!0x01c44c40 ConVar cl_viewmodel_pre_animate
r5apex.exe!0x01c54e50 ConVar cl_warnAboutSoundsOnInvalidEntities
r5apex.exe!0x01c45cc0 ConVar cl_yawspeed
r5apex.exe!0x011a1380 ConVar clampHostFrameTimeToOneTick_enable
r5apex.exe!0x018966a0 ConVar clearOnAnimChange
r5apex.exe!0x01c62fb0 ConVar client_deferredSnapshotScriptCalls
r5apex.exe!0x011a0470 ConVar clientport
r5apex.exe!0x012e88b0 ConVar cloak_enabled
r5apex.exe!0x012ee360 ConVar cloak_pilotNoiseFactor
r5apex.exe!0x012ee4a0 ConVar cloak_pilotTint1
r5apex.exe!0x012ee040 ConVar cloak_pilotTint2
r5apex.exe!0x012eddc0 ConVar cloak_pilotTint3
r5apex.exe!0x0119a470 ConVar clock_bias_mp
r5apex.exe!0x0119b3b0 ConVar clock_bias_sp
r5apex.exe!0x0119aa90 ConVar clock_showcorrections
r5apex.exe!0x0119b2c0 ConVar clock_showdebuginfo
r5apex.exe!0x058282b0 ConVar closecaption
r5apex.exe!0x01cac700 ConVar cockpitDrift_scalePitch
r5apex.exe!0x01cacf10 ConVar cockpitDrift_scaleYaw
r5apex.exe!0x01cade60 ConVar cockpitDrift_speedPitch
r5apex.exe!0x01cac500 ConVar cockpitDrift_speedYaw
r5apex.exe!0x01e7e800 ConVar cockpitShake_sourceRollRange
r5apex.exe!0x01e7f710 ConVar cockpitShake_translateRange
r5apex.exe!0x01e7d490 ConVar cockpit_damage_chroma_scale
r5apex.exe!0x01e7d910 ConVar cockpit_hit_chroma_max_time
r5apex.exe!0x01e7f200 ConVar cockpit_hit_chroma_scale
r5apex.exe!0x01e7ea00 ConVar cockpit_pitch_down_frac
r5apex.exe!0x01e7ff70 ConVar cockpit_pitch_up_frac
r5apex.exe!0x01e7edf0 ConVar cockpit_screen_boot_chroma_scale
r5apex.exe!0x01e7e460 ConVar cockpit_screen_boot_delay_bottom
r5apex.exe!0x01e80010 ConVar cockpit_screen_boot_delay_left
r5apex.exe!0x01e7d290 ConVar cockpit_screen_boot_delay_mid
r5apex.exe!0x01e7e8c0 ConVar cockpit_screen_boot_delay_right
r5apex.exe!0x01e803c0 ConVar cockpit_screen_boot_delay_top
r5apex.exe!0x012a6150 ConVar coll_spatial_entry_limit_client
r5apex.exe!0x012a61f0 ConVar coll_spatial_optimize_prefetch
r5apex.exe!0x01196260 ConVar coll_use_bolt_size
r5apex.exe!0x01c5e000 ConVar colorblind_mode
r5apex.exe!0x012aa8e0 ConVar communities_doRealNameLookupsForCommunityCreators
r5apex.exe!0x012ab240 ConVar communities_enabled
r5apex.exe!0x012d5e90 ConVar communities_hostname
r5apex.exe!0x012aaa20 ConVar community
r5apex.exe!0x012aab60 ConVar community_abortCommunitySettingsTime
r5apex.exe!0x012aae80 ConVar community_abortUserInfoTime
r5apex.exe!0x012d5570 ConVar community_browse_excludeMine
r5apex.exe!0x012a9e30 ConVar community_clantags
r5apex.exe!0x012d5050 ConVar community_doRealNameLookupsForInbox
r5apex.exe!0x012ab1a0 ConVar community_frame_run
r5apex.exe!0x012d9cb0 ConVar community_queryServerWhenOrphaned
r5apex.exe!0x012d4fb0 ConVar community_replaceInboxTokens
r5apex.exe!0x012d5970 ConVar community_replaceInboxTokens
r5apex.exe!0x012d4e90 ConVar community_resolveNames
r5apex.exe!0x012d5290 ConVar community_resolveNames
r5apex.exe!0x012aade0 ConVar community_send_server_voice
r5apex.exe!0x012d5330 ConVar community_spam
r5apex.exe!0x012ab2e0 ConVar community_staleCommunitySettingsTime
r5apex.exe!0x012aac00 ConVar community_staleUserInfoTime
r5apex.exe!0x012dc5a0 ConVar con_logfile
r5apex.exe!0x0119a2f0 ConVar con_timestamp
r5apex.exe!0x012eb5d0 ConVar cpu_level
r5apex.exe!0x018b5370 ConVar cpu_level
r5apex.exe!0x01e92360 ConVar createentitydecals
r5apex.exe!0x012a9250 ConVar crossPlay_Enabled
r5apex.exe!0x01c39d30 ConVar csm0_on_worker
r5apex.exe!0x012eb710 ConVar csm_cascade_res
r5apex.exe!0x018b5690 ConVar csm_cascade_res
r5apex.exe!0x012e89f0 ConVar csm_coverage
r5apex.exe!0x01c3b7d0 ConVar csm_culling_use_base_planes
r5apex.exe!0x018b1c50 ConVar csm_culling_use_exclusion_planes
r5apex.exe!0x0189f2f0 ConVar csm_culling_use_inclusion_planes
r5apex.exe!0x01c44ba0 ConVar csm_culling_use_planes
r5apex.exe!0x018ab120 ConVar csm_debug_2d
r5apex.exe!0x018ae590 ConVar csm_debug_culling
r5apex.exe!0x01c3a050 ConVar csm_debug_vis_hi_range
r5apex.exe!0x018ae170 ConVar csm_debug_vis_lo_range
r5apex.exe!0x018ae340 ConVar csm_depth_bias
r5apex.exe!0x01c3b060 ConVar csm_dropsequence_adjusted_coverage
r5apex.exe!0x0189f5e0 ConVar csm_dropsequence_adjustment
r5apex.exe!0x012eb350 ConVar csm_enabled
r5apex.exe!0x018a8630 ConVar csm_fadeModels
r5apex.exe!0x018ab000 ConVar csm_force_no_csm_in_reflections
r5apex.exe!0x0189f0e0 ConVar csm_frustum_draw
r5apex.exe!0x0189f040 ConVar csm_frustum_draw_lock
r5apex.exe!0x01c39bf0 ConVar csm_ignore_cascade12
r5apex.exe!0x018a7830 ConVar csm_ignore_edge_planes
r5apex.exe!0x018af950 ConVar csm_ignore_face_planes
r5apex.exe!0x01c3ae80 ConVar csm_max_z_offset
r5apex.exe!0x018ad4b0 ConVar csm_min_z_offset
r5apex.exe!0x0189f6a0 ConVar csm_renderable_shadows
r5apex.exe!0x01c3af20 ConVar csm_rope_shadows
r5apex.exe!0x018acf30 ConVar csm_rot_override
r5apex.exe!0x018a7510 ConVar csm_rot_x
r5apex.exe!0x018aa5f0 ConVar csm_rot_y
r5apex.exe!0x01c3b5f0 ConVar csm_shadow_split_lerp_factor_range
r5apex.exe!0x018ab6a0 ConVar csm_texel_size_cascade_0
r5apex.exe!0x018ae8b0 ConVar csm_texel_size_cascade_1
r5apex.exe!0x018a7c20 ConVar csm_texel_size_cascade_2
r5apex.exe!0x018b67b0 ConVar csm_texel_size_cascade_onecascade
r5apex.exe!0x018a7470 ConVar csm_use_env_light_direction
r5apex.exe!0x018b1bb0 ConVar csm_world_shadow_meshes
r5apex.exe!0x018b68f0 ConVar csm_world_shadows
r5apex.exe!0x018b6660 ConVar csm_z_cover_world
r5apex.exe!0x012d8fe0 ConVar curl_allowHTTPS
r5apex.exe!0x012d91c0 ConVar curl_preloadDlls
r5apex.exe!0x012d9120 ConVar curl_spamAllQueryStates
r5apex.exe!0x0789c190 ConVar cursorWide
r5apex.exe!0x01e7fc20 ConVar damageIndicatorReplayTimeOffset
r5apex.exe!0x01e67c50 ConVar damage_indicator_style_pilot
r5apex.exe!0x01c633b0 ConVar damageinfo_defendInvalidValues
r5apex.exe!0x018977a0 ConVar debugFootstepEffects
r5apex.exe!0x011983f0 ConVar debug_debug_overlay
r5apex.exe!0x012ab100 ConVar debug_force_textRestriction
r5apex.exe!0x012aaac0 ConVar debug_force_ugcRestriction
r5apex.exe!0x012aaf20 ConVar debug_force_voiceRestriction
r5apex.exe!0x0119a6f0 ConVar debug_map_crc
r5apex.exe!0x012f1040 ConVar decal_clip_debug_draw
r5apex.exe!0x012f0f60 ConVar decal_clip_debug_groups
r5apex.exe!0x01e82d40 ConVar defer_weapon_effects
r5apex.exe!0x0189f880 ConVar delayPostSnapshotNotificationsToAfterInterpolation
r5apex.exe!0x012a53d0 ConVar demo_autoRecord
r5apex.exe!0x012a4f30 ConVar demo_autoRecordName
r5apex.exe!0x01e7b1d0 ConVar demo_connect_string
r5apex.exe!0x01e7aeb0 ConVar demo_ui_enable
r5apex.exe!0x01e66010 ConVar devStats
r5apex.exe!0x012d85c0 ConVar developer
r5apex.exe!0x01e7efd0 ConVar disable_player_use_prompts
r5apex.exe!0x012a5e30 ConVar discord_largeImage
r5apex.exe!0x012a5d90 ConVar discord_smallImage
r5apex.exe!0x012a5ed0 ConVar discord_updatePresence
r5apex.exe!0x01198a20 ConVar dlight_default_falloff
r5apex.exe!0x0119a820 ConVar dlight_early_clear
r5apex.exe!0x0126eb40 ConVar dlight_enable
r5apex.exe!0x0126da60 ConVar dlight_overlay
r5apex.exe!0x01c66e80 ConVar dodge_cockpitHack
r5apex.exe!0x01c98120 ConVar dodge_cockpitOffsetMax
r5apex.exe!0x01c82110 ConVar dodge_cockpitTiltMax
r5apex.exe!0x01c643c0 ConVar dodge_vertical_enable
r5apex.exe!0x01c67c60 ConVar dodge_vertical_horzspeedscale
r5apex.exe!0x01c9ab60 ConVar dodge_vertical_in_air
r5apex.exe!0x01c9b6c0 ConVar dodge_vertical_threshold
r5apex.exe!0x01c64000 ConVar dodge_viewTiltDecreaseSpeed
r5apex.exe!0x01c95ee0 ConVar dodge_viewTiltFalloffTime
r5apex.exe!0x01c63b00 ConVar dodge_viewTiltIncreaseSpeed
r5apex.exe!0x01c95e40 ConVar dodge_viewTiltMax
r5apex.exe!0x01c5f8e0 ConVar dof_enable
r5apex.exe!0x0188e150 ConVar dof_farDepthEnd
r5apex.exe!0x0188e1f0 ConVar dof_farDepthStart
r5apex.exe!0x0188e330 ConVar dof_monitorFarDepthEnd
r5apex.exe!0x0188e010 ConVar dof_monitorFarDepthStart
r5apex.exe!0x0188ded0 ConVar dof_monitorNearDepthEnd
r5apex.exe!0x0188e0b0 ConVar dof_monitorNearDepthStart
r5apex.exe!0x0188e290 ConVar dof_nearDepthEnd
r5apex.exe!0x0188df70 ConVar dof_nearDepthStart
r5apex.exe!0x0188e3d0 ConVar dof_overrideParams
r5apex.exe!0x01c5ce90 ConVar dof_variable_blur
r5apex.exe!0x01e909c0 ConVar dormant_debug
r5apex.exe!0x01e7d7d0 ConVar draw_target_info_offscreen
r5apex.exe!0x01198810 ConVar dtwatchclass
r5apex.exe!0x0119b910 ConVar dtwatchdecode
r5apex.exe!0x011980f0 ConVar dtwatchencode
r5apex.exe!0x0119b7f0 ConVar dtwatchent
r5apex.exe!0x01198620 ConVar dtwatchvar
r5apex.exe!0x01e6aad0 ConVar dump_varsights_calculations
r5apex.exe!0x0189b200 ConVar durango_voice_chat_team_only
r5apex.exe!0x012eb530 ConVar dvs_enable
r5apex.exe!0x018b6a10 ConVar dvs_enable_in_lobby
r5apex.exe!0x012ec4b0 ConVar dvs_gpuframetime_max
r5apex.exe!0x012ec5f0 ConVar dvs_gpuframetime_min
r5apex.exe!0x012ec550 ConVar dvs_scale_min
r5apex.exe!0x011a0b10 ConVar eadpAuth_hostname
r5apex.exe!0x011a1c50 ConVar eadpFriends_hostname
r5apex.exe!0x012d61d0 ConVar eadpGroups_Enabled
r5apex.exe!0x0119d3c0 ConVar eadpGroups_hostname
r5apex.exe!0x011a2f70 ConVar eadpRtm_hostname
r5apex.exe!0x011a1560 ConVar eadpSearch_hostname
r5apex.exe!0x012dd0d0 ConVar eadp_rtm_spam_log
r5apex.exe!0x01c62db0 ConVar effect_update_array_spam
r5apex.exe!0x011a9b50 ConVar enable_KVFileOverrides
r5apex.exe!0x01199330 ConVar enable_debug_overlays
r5apex.exe!0x01c99250 ConVar enable_height_based_land_anims
r5apex.exe!0x01c972e0 ConVar enable_height_based_land_anims_titans
r5apex.exe!0x01896d60 ConVar enable_skeleton_draw
r5apex.exe!0x011a4f20 ConVar encrypt_multiKey
r5apex.exe!0x01e91460 ConVar ent_lightweightEnts
r5apex.exe!0x01e90640 ConVar ent_repack_almostFull
r5apex.exe!0x01e90a60 ConVar ent_repack_threshhold
r5apex.exe!0x01c97930 ConVar entity_error_on_hitbox_count_mismatch
r5apex.exe!0x01c81c10 ConVar entity_skipRedundantAddEffects
r5apex.exe!0x012a4670 ConVar entity_useNetworkFieldBuffer
r5apex.exe!0x01e90c40 ConVar error_if_non_standard_ent_create
r5apex.exe!0x01e69e20 ConVar eula_version
r5apex.exe!0x01e697b0 ConVar eula_version_accepted
r5apex.exe!0x018adb70 ConVar eventseq_debug
r5apex.exe!0x01e65760 ConVar everything_unlocked
r5apex.exe!0x01e679d0 ConVar fast_intro
r5apex.exe!0x01e9b5d0 ConVar fatal_script_error_prompt
r5apex.exe!0x01e9b490 ConVar fatal_script_errors
r5apex.exe!0x01e9b530 ConVar fatal_script_errors_client
r5apex.exe!0x01e9b670 ConVar fatal_script_errors_server
r5apex.exe!0x012df110 ConVar filesystem_buffer_size
r5apex.exe!0x012df410 ConVar filesystem_max_stdio_read
r5apex.exe!0x012df4b0 ConVar filesystem_native
r5apex.exe!0x012df2d0 ConVar filesystem_report_buffered_io
r5apex.exe!0x012df230 ConVar filesystem_unbuffered_io
r5apex.exe!0x012df370 ConVar filesystem_use_overlapped_io
r5apex.exe!0x01c9e4b0 ConVar fire_animevents_overlay_not_active
r5apex.exe!0x01e89f20 ConVar first_person_bullet_delay
r5apex.exe!0x0189efa0 ConVar first_person_proxy_blend_distance
r5apex.exe!0x01e860b0 ConVar first_person_proxy_debug
r5apex.exe!0x01e69650 ConVar firsttime_mp_message
r5apex.exe!0x01e65580 ConVar fog_enable
r5apex.exe!0x011a49b0 ConVar fog_enable_water_fog
r5apex.exe!0x01e64fa0 ConVar fog_enableskybox
r5apex.exe!0x01e8d360 ConVar force3PLaserAttachment
r5apex.exe!0x012ab060 ConVar force_EAAccess
r5apex.exe!0x011aa2c0 ConVar fps_max
r5apex.exe!0x011a89d0 ConVar fps_max_use_refresh
r5apex.exe!0x011a9270 ConVar fps_max_vsync
r5apex.exe!0x0189aaa0 ConVar freecam_swallowButtonInput
r5apex.exe!0x01c636d0 ConVar freefall_sound_autoplay_time
r5apex.exe!0x01c94390 ConVar freefall_sound_height
r5apex.exe!0x012dba30 ConVar friends_onlineUpdateInterval
r5apex.exe!0x012de890 ConVar fs_intralevel_reads
r5apex.exe!0x012ded50 ConVar fs_monitor_read_from_pack
r5apex.exe!0x012de9d0 ConVar fs_report_intra_level_readopens
r5apex.exe!0x012dee60 ConVar fs_report_long_reads
r5apex.exe!0x012de930 ConVar fs_report_sync_opens
r5apex.exe!0x012deb90 ConVar fs_report_sync_opens_callstack
r5apex.exe!0x012decb0 ConVar fs_report_sync_opens_fatal
r5apex.exe!0x012deaf0 ConVar fs_showAllReads
r5apex.exe!0x012df580 ConVar fs_vpk_file_open
r5apex.exe!0x012df000 ConVar fs_warning_mode
r5apex.exe!0x01c51980 ConVar func_break_max_pieces
r5apex.exe!0x018b5b60 ConVar fx_debug
r5apex.exe!0x01e959b0 ConVar fx_deferWorldTraceConstraint
r5apex.exe!0x01c4aa20 ConVar fx_glass_velocity_cap
r5apex.exe!0x01c61e60 ConVar fx_impact_ally
r5apex.exe!0x01c5b070 ConVar fx_impact_enemy
r5apex.exe!0x01c5e140 ConVar fx_impact_neutral
r5apex.exe!0x01e96e90 ConVar fx_screenspacepass
r5apex.exe!0x01cab590 ConVar g_debug_ragdoll_removal
r5apex.exe!0x01897fe0 ConVar g_ragdoll_fadespeed
r5apex.exe!0x01caac20 ConVar g_ragdoll_important_maxcount
r5apex.exe!0x018972c0 ConVar g_ragdoll_lvfadespeed
r5apex.exe!0x01c4eab0 ConVar gameCursor_ModeActive
r5apex.exe!0x01c4f8c0 ConVar gameCursor_Velocity
r5apex.exe!0x018aba30 ConVar gamepad_ads_advanced_sensitivity_scalar_0
r5apex.exe!0x018abad0 ConVar gamepad_ads_advanced_sensitivity_scalar_1
r5apex.exe!0x018abb70 ConVar gamepad_ads_advanced_sensitivity_scalar_2
r5apex.exe!0x018abc10 ConVar gamepad_ads_advanced_sensitivity_scalar_3
r5apex.exe!0x018abcb0 ConVar gamepad_ads_advanced_sensitivity_scalar_4
r5apex.exe!0x018abd50 ConVar gamepad_ads_advanced_sensitivity_scalar_5
r5apex.exe!0x018abdf0 ConVar gamepad_ads_advanced_sensitivity_scalar_6
r5apex.exe!0x018abe90 ConVar gamepad_ads_advanced_sensitivity_scalar_7
r5apex.exe!0x018b6200 ConVar gamepad_aim_assist_ads_high_power_scopes
r5apex.exe!0x018b1570 ConVar gamepad_aim_assist_ads_low_power_scopes
r5apex.exe!0x018a05b0 ConVar gamepad_aim_assist_hip_high_power_scopes
r5apex.exe!0x018afa70 ConVar gamepad_aim_assist_hip_low_power_scopes
r5apex.exe!0x018b99f0 ConVar gamepad_aim_assist_melee
r5apex.exe!0x018b55f0 ConVar gamepad_aim_speed
r5apex.exe!0x018b0300 ConVar gamepad_aim_speed_ads_0
r5apex.exe!0x018b03a0 ConVar gamepad_aim_speed_ads_1
r5apex.exe!0x018b0440 ConVar gamepad_aim_speed_ads_2
r5apex.exe!0x018b04e0 ConVar gamepad_aim_speed_ads_3
r5apex.exe!0x018b0580 ConVar gamepad_aim_speed_ads_4
r5apex.exe!0x018b0620 ConVar gamepad_aim_speed_ads_5
r5apex.exe!0x018b06c0 ConVar gamepad_aim_speed_ads_6
r5apex.exe!0x018b0760 ConVar gamepad_aim_speed_ads_7
r5apex.exe!0x01e69710 ConVar gamepad_button_layout
r5apex.exe!0x01e693e0 ConVar gamepad_buttons_are_southpaw
r5apex.exe!0x018b6480 ConVar gamepad_custom_ads_pitch
r5apex.exe!0x018adcb0 ConVar gamepad_custom_ads_turn_delay
r5apex.exe!0x018a0150 ConVar gamepad_custom_ads_turn_pitch
r5apex.exe!0x01c3b910 ConVar gamepad_custom_ads_turn_time
r5apex.exe!0x018a7cc0 ConVar gamepad_custom_ads_turn_yaw
r5apex.exe!0x01c3aac0 ConVar gamepad_custom_ads_yaw
r5apex.exe!0x018a6ef0 ConVar gamepad_custom_assist_on
r5apex.exe!0x018b3240 ConVar gamepad_custom_curve
r5apex.exe!0x018b54b0 ConVar gamepad_custom_deadzone_in
r5apex.exe!0x0189fce0 ConVar gamepad_custom_deadzone_out
r5apex.exe!0x018b0a20 ConVar gamepad_custom_enabled
r5apex.exe!0x018aa690 ConVar gamepad_custom_hip_pitch
r5apex.exe!0x01c39dd0 ConVar gamepad_custom_hip_turn_delay
r5apex.exe!0x01c3b2f0 ConVar gamepad_custom_hip_turn_pitch
r5apex.exe!0x01c3ade0 ConVar gamepad_custom_hip_turn_time
r5apex.exe!0x018b5870 ConVar gamepad_custom_hip_turn_yaw
r5apex.exe!0x018b1110 ConVar gamepad_custom_hip_yaw
r5apex.exe!0x01e684d0 ConVar gamepad_custom_pilot
r5apex.exe!0x01e68010 ConVar gamepad_custom_titan
r5apex.exe!0x018b0980 ConVar gamepad_deadzone_index_look
r5apex.exe!0x018ab740 ConVar gamepad_deadzone_index_move
r5apex.exe!0x01c59210 ConVar gamepad_enabled
r5apex.exe!0x018b62a0 ConVar gamepad_look_curve
r5apex.exe!0x01e69fc0 ConVar gamepad_stick_layout
r5apex.exe!0x01c459e0 ConVar gamepad_toggle_ads
r5apex.exe!0x01c505d0 ConVar gamepad_togglecrouch_hold
r5apex.exe!0x012e2b40 ConVar gamepad_trigger_threshold
r5apex.exe!0x018b6160 ConVar gamepad_use_per_scope_ads_settings
r5apex.exe!0x018b1370 ConVar gamepad_use_per_scope_sensitivity_scalars
r5apex.exe!0x01e6aa30 ConVar gamepad_use_type
r5apex.exe!0x011a6520 ConVar gameui_xbox
r5apex.exe!0x01e69d20 ConVar gamma_adjusted
r5apex.exe!0x018a8c50 ConVar gatherprops_no_wait
r5apex.exe!0x012eee00 ConVar gfx_desaturate_force
r5apex.exe!0x01e654e0 ConVar gl_clear_color_buffer
r5apex.exe!0x01e646e0 ConVar gl_clear_fogcolor
r5apex.exe!0x01e64960 ConVar gl_clear_randomcolor
r5apex.exe!0x018ae210 ConVar glass_break_required_speed
r5apex.exe!0x01c57ed0 ConVar glass_shatter_direction_force_scale
r5apex.exe!0x01c4c3b0 ConVar glass_shatter_force_scale
r5apex.exe!0x01c51aa0 ConVar glass_shatter_size_scale
r5apex.exe!0x01c4b150 ConVar glass_shatter_use_real_direction
r5apex.exe!0x012eed60 ConVar glitch_aberrationScale
r5apex.exe!0x0188ccc0 ConVar global_lighting_partial_update
r5apex.exe!0x01e9c980 ConVar gpu_count
r5apex.exe!0x012f0ba0 ConVar gpu_driven_tex_stream
r5apex.exe!0x012f10e0 ConVar gpu_driven_tex_stream_single_thread
r5apex.exe!0x012ec690 ConVar gpu_level
r5apex.exe!0x01c44160 ConVar gpu_level
r5apex.exe!0x012e3190 ConVar gpu_mem_level
r5apex.exe!0x018a0090 ConVar gpu_mem_level
r5apex.exe!0x012ec370 ConVar gpu_vram_size_mb
r5apex.exe!0x01c9b620 ConVar grapple_accel_human
r5apex.exe!0x01c98220 ConVar grapple_accel_titan
r5apex.exe!0x01c9ada0 ConVar grapple_around_obstacle_accel
r5apex.exe!0x01c99870 ConVar grapple_autoMantle
r5apex.exe!0x01c9cb60 ConVar grapple_autoMeleeConvergeTime
r5apex.exe!0x01c68f90 ConVar grapple_autoMeleeOnDetach
r5apex.exe!0x01c9d680 ConVar grapple_autoMeleePredict
r5apex.exe!0x01c9f510 ConVar grapple_autoMeleePredictTime
r5apex.exe!0x01c9f470 ConVar grapple_autoMeleeViewRotateSpeedFar
r5apex.exe!0x01c9f3d0 ConVar grapple_autoMeleeViewRotateSpeedNear
r5apex.exe!0x01c9b4b0 ConVar grapple_debug
r5apex.exe!0x01c63e20 ConVar grapple_decelMeleeStrength
r5apex.exe!0x01c947f0 ConVar grapple_decel_human
r5apex.exe!0x01c66720 ConVar grapple_decel_titan
r5apex.exe!0x01c67240 ConVar grapple_detachExtraAllowedLength
r5apex.exe!0x01c67f80 ConVar grapple_disableMeleeWhenActive
r5apex.exe!0x01c93e60 ConVar grapple_dontFightGravity
r5apex.exe!0x01c95180 ConVar grapple_fallSpeed
r5apex.exe!0x01c9a6e0 ConVar grapple_forcedRetractVel
r5apex.exe!0x01c68b80 ConVar grapple_gracePeriod
r5apex.exe!0x01c68190 ConVar grapple_gravityPushUnderContribution
r5apex.exe!0x01c98fc0 ConVar grapple_initialImpulseOffGround_human
r5apex.exe!0x01c9c680 ConVar grapple_initialImpulseOffGround_human_npc
r5apex.exe!0x01c821b0 ConVar grapple_initialImpulseOffGround_titan
r5apex.exe!0x01c98580 ConVar grapple_initialImpulse_human
r5apex.exe!0x01c679e0 ConVar grapple_initialImpulse_titan
r5apex.exe!0x01c68050 ConVar grapple_initialSlowFracVert_human
r5apex.exe!0x01c82590 ConVar grapple_initialSlowFracVert_titan
r5apex.exe!0x01c94b60 ConVar grapple_initialSlowFrac_human
r5apex.exe!0x01c94890 ConVar grapple_initialSlowFrac_titan
r5apex.exe!0x01c99940 ConVar grapple_initialSpeedMin_human
r5apex.exe!0x01c9cc00 ConVar grapple_initialSpeedMin_titan
r5apex.exe!0x01c93fa0 ConVar grapple_jumpFrac
r5apex.exe!0x01c94220 ConVar grapple_letGravityHelpCosAngle
r5apex.exe!0x01c942c0 ConVar grapple_lift
r5apex.exe!0x01c9e330 ConVar grapple_pullDelay_human
r5apex.exe!0x01c9e290 ConVar grapple_pullDelay_titan
r5apex.exe!0x01c9ceb0 ConVar grapple_retractVel
r5apex.exe!0x01c9c3d0 ConVar grapple_rodeoVerticalImpulse
r5apex.exe!0x01c9f5b0 ConVar grapple_shootVel
r5apex.exe!0x01c940e0 ConVar grapple_speedRampMax_human
r5apex.exe!0x01c9d400 ConVar grapple_speedRampMax_titan
r5apex.exe!0x01c67940 ConVar grapple_speedRampMin_human
r5apex.exe!0x01c94c00 ConVar grapple_speedRampMin_titan
r5apex.exe!0x01c984e0 ConVar grapple_speedRampTime_human
r5apex.exe!0x01c97680 ConVar grapple_speedRampTime_titan
r5apex.exe!0x01c94040 ConVar grapple_swingAngle
r5apex.exe!0x01c68410 ConVar grapple_swingPullAngle
r5apex.exe!0x01c67e40 ConVar grapple_swingPullSpeedLength
r5apex.exe!0x01c64280 ConVar grapple_swingPullSpeedScale
r5apex.exe!0x01c9ad00 ConVar grapple_titanEmbarkDist
r5apex.exe!0x01c94180 ConVar grapple_windowCheckDist
r5apex.exe!0x01ca7a70 ConVar gravity_grenade_decel
r5apex.exe!0x01ca39d0 ConVar gravity_grenade_projectile_min_speed
r5apex.exe!0x01c640a0 ConVar ground_debug
r5apex.exe!0x01c9f760 ConVar ground_trace_hull_radius
r5apex.exe!0x011ac7a0 ConVar grx_hasUnknownItems
r5apex.exe!0x012f2240 ConVar gtao_angle_bias
r5apex.exe!0x012f1ac0 ConVar gtao_intensity
r5apex.exe!0x012f24c0 ConVar gtao_intensity_in_lobby
r5apex.exe!0x012f2560 ConVar gtao_thickness_heuristic
r5apex.exe!0x012a4d50 ConVar hasAnyAssetsWithDiscardedStreamableData
r5apex.exe!0x012a5750 ConVar hasMic
r5apex.exe!0x012a4cb0 ConVar hasPartialInstall
r5apex.exe!0x012f22e0 ConVar hbao_angle_bias
r5apex.exe!0x012f1de0 ConVar hbao_intensity
r5apex.exe!0x012f1fc0 ConVar hbao_stepsize_random
r5apex.exe!0x012f1ca0 ConVar hbaobasic_tangent_bias
r5apex.exe!0x01c56dd0 ConVar hidehud
r5apex.exe!0x018ae030 ConVar highlight_deferred_update
r5apex.exe!0x012f1180 ConVar highlight_draw
r5apex.exe!0x012f1340 ConVar highlight_lazy_clear_buffers
r5apex.exe!0x012f12a0 ConVar highlight_object_max_count
r5apex.exe!0x01e95300 ConVar hitbox_bodygroup_check
r5apex.exe!0x01c5e9c0 ConVar hitch_alert_active
r5apex.exe!0x01c5e300 ConVar hitch_alert_color
r5apex.exe!0x01c5c380 ConVar hitch_alert_show_large_snapshots
r5apex.exe!0x012d80e0 ConVar host_RunFrameServerAlways
r5apex.exe!0x012d7b40 ConVar host_ShowIPCCallCount
r5apex.exe!0x012a7650 ConVar host_flush_threshold
r5apex.exe!0x012d75c0 ConVar host_forceTakeHomeBuild
r5apex.exe!0x012d8520 ConVar host_framerate
r5apex.exe!0x011a3d60 ConVar host_hasIrreversibleShutdown
r5apex.exe!0x012d8480 ConVar host_limitlocal
r5apex.exe!0x0119c890 ConVar host_map
r5apex.exe!0x012d8220 ConVar host_print_frame_times
r5apex.exe!0x012d7080 ConVar host_profile
r5apex.exe!0x012d7a00 ConVar host_runframe_input_parcelremainder
r5apex.exe!0x012d7660 ConVar host_server_thread_min_ticks
r5apex.exe!0x012d6780 ConVar host_sleep
r5apex.exe!0x012d6ac0 ConVar host_speeds
r5apex.exe!0x0126e5d0 ConVar host_syncfps
r5apex.exe!0x012d6e40 ConVar host_thread_join_fast
r5apex.exe!0x012d8040 ConVar host_thread_mode
r5apex.exe!0x012d6c80 ConVar host_threaded_sound
r5apex.exe!0x012d8660 ConVar host_timescale
r5apex.exe!0x0119fed0 ConVar hostname
r5apex.exe!0x01e7a970 ConVar hover_vehicle_passenger_left_attachment_name
r5apex.exe!0x012d8e00 ConVar http_StryderKey
r5apex.exe!0x012d8860 ConVar http_debug
r5apex.exe!0x012d8cc0 ConVar http_debug_forceFailRate
r5apex.exe!0x012d8ae0 ConVar http_debug_forceFailStatus
r5apex.exe!0x012d8900 ConVar http_failuresAsErrors
r5apex.exe!0x012d8c20 ConVar http_maxAllocateAttempts
r5apex.exe!0x012d9080 ConVar http_recv_fail_realloc
r5apex.exe!0x012d8b80 ConVar http_sandbox
r5apex.exe!0x012d8a40 ConVar http_showQueries
r5apex.exe!0x01e934e0 ConVar hud_autoreloadscript
r5apex.exe!0x01c607a0 ConVar hud_setting_accessibleChat
r5apex.exe!0x01c5c000 ConVar hud_setting_adsDof
r5apex.exe!0x01e7d5d0 ConVar hud_setting_compactOverHeadNames
r5apex.exe!0x01c61380 ConVar hud_setting_damageIndicatorStyle
r5apex.exe!0x01c5dec0 ConVar hud_setting_damageTextStyle
r5apex.exe!0x01c5b110 ConVar hud_setting_enableModWheel
r5apex.exe!0x01e687f0 ConVar hud_setting_healthUseOnHold
r5apex.exe!0x01e68b10 ConVar hud_setting_healthWheelToggle
r5apex.exe!0x01e68d30 ConVar hud_setting_healthWheelUseOnRelease
r5apex.exe!0x01c5d8a0 ConVar hud_setting_lootPromptStyle
r5apex.exe!0x01c5dba0 ConVar hud_setting_minimapRotate
r5apex.exe!0x01e683b0 ConVar hud_setting_ordnanceUseOnHold
r5apex.exe!0x01e6a120 ConVar hud_setting_ordnanceWheelToggle
r5apex.exe!0x01e68570 ConVar hud_setting_ordnanceWheelUseOnRelease
r5apex.exe!0x01c61260 ConVar hud_setting_pingAlpha
r5apex.exe!0x01c5e620 ConVar hud_setting_pingDoubleTapEnemy
r5apex.exe!0x01e68e70 ConVar hud_setting_pingWheelToggle
r5apex.exe!0x01c5b4b0 ConVar hud_setting_showButtonHints
r5apex.exe!0x01c60dc0 ConVar hud_setting_showCallsigns
r5apex.exe!0x01c5be10 ConVar hud_setting_showHopUpPopUp
r5apex.exe!0x01c5ffb0 ConVar hud_setting_showLevelUp
r5apex.exe!0x01c5ef20 ConVar hud_setting_showMedals
r5apex.exe!0x01c62040 ConVar hud_setting_showMeter
r5apex.exe!0x01c5f060 ConVar hud_setting_showObituary
r5apex.exe!0x01c5c5b0 ConVar hud_setting_showTips
r5apex.exe!0x01c61c00 ConVar hud_setting_showWeaponFlyouts
r5apex.exe!0x01c61ca0 ConVar hud_setting_streamerMode
r5apex.exe!0x01c4a540 ConVar hudchat_new_message_fade_duration
r5apex.exe!0x01c51be0 ConVar hudchat_new_message_shown_duration
r5apex.exe!0x01c4b290 ConVar hudchat_play_text_to_speech
r5apex.exe!0x01c4a380 ConVar hudchat_transition_message_mode_fade_duration
r5apex.exe!0x01c494a0 ConVar hudchat_visibility
r5apex.exe!0x01e9a720 ConVar hudwarp_chopsize
r5apex.exe!0x01e9a400 ConVar hudwarp_override
r5apex.exe!0x01e9a2c0 ConVar hudwarp_viewDist
r5apex.exe!0x01e9a680 ConVar hudwarp_xScale
r5apex.exe!0x01e993e0 ConVar hudwarp_xWarp
r5apex.exe!0x01e99480 ConVar hudwarp_yScale
r5apex.exe!0x01e9a360 ConVar hudwarp_yWarp
r5apex.exe!0x01c5bf50 ConVar idcolor_ally
r5apex.exe!0x01c622c0 ConVar idcolor_ally_cb1
r5apex.exe!0x01c60080 ConVar idcolor_ally_cb2
r5apex.exe!0x01c605c0 ConVar idcolor_ally_cb3
r5apex.exe!0x01c62c90 ConVar idcolor_enemy
r5apex.exe!0x01c5f720 ConVar idcolor_enemy_cb1
r5apex.exe!0x01c5f1c0 ConVar idcolor_enemy_cb2
r5apex.exe!0x01c5b1b0 ConVar idcolor_enemy_cb3
r5apex.exe!0x01c5c240 ConVar idcolor_neutral
r5apex.exe!0x011aa5e0 ConVar ignore_fatal_errors
r5apex.exe!0x01e65e20 ConVar ignore_script_errors
r5apex.exe!0x01c9f6c0 ConVar ik_debug
r5apex.exe!0x01c63230 ConVar ik_debug_chain
r5apex.exe!0x01c99a10 ConVar ik_debug_ent
r5apex.exe!0x01c9d9a0 ConVar ik_debug_text
r5apex.exe!0x01c67060 ConVar ik_enable
r5apex.exe!0x01c93dc0 ConVar ik_enable_client
r5apex.exe!0x01c9b000 ConVar ik_height_adjust
r5apex.exe!0x01c674c0 ConVar ik_height_adjust_debug
r5apex.exe!0x01c65fc0 ConVar ik_height_adjust_move_speed
r5apex.exe!0x01c63ce0 ConVar ik_height_adjust_sine
r5apex.exe!0x01c9c5e0 ConVar ik_height_adjust_speed
r5apex.exe!0x01c97210 ConVar ik_latch
r5apex.exe!0x01c82470 ConVar ik_normal_lerp_rate
r5apex.exe!0x01c96070 ConVar ik_unlatch_max_rate
r5apex.exe!0x012e0e30 ConVar ime_enabled
r5apex.exe!0x01e978a0 ConVar imgui_buildmode
r5apex.exe!0x01e97c00 ConVar imgui_buildmode
r5apex.exe!0x01c54f70 ConVar impact_allow
r5apex.exe!0x01c5f100 ConVar impact_debug_info
r5apex.exe!0x01c56d30 ConVar impact_victim_offset_dist
r5apex.exe!0x01cac1b0 ConVar impulse_low_decel_duration_scalar
r5apex.exe!0x012a5890 ConVar inPartyChat
r5apex.exe!0x011a1cf0 ConVar in_forceuser
r5apex.exe!0x011a97f0 ConVar in_syncRT
r5apex.exe!0x01c45520 ConVar in_usekeyboardsampletime
r5apex.exe!0x012aafc0 ConVar inbox_enabled
r5apex.exe!0x012aaca0 ConVar infoblock_requestInterval
r5apex.exe!0x01c57870 ConVar input_did_turn_threshold
r5apex.exe!0x01e68c90 ConVar intro_viewed
r5apex.exe!0x011a2c10 ConVar ip
r5apex.exe!0x01c55a70 ConVar joy_advaxisr
r5apex.exe!0x01c464a0 ConVar joy_advaxisu
r5apex.exe!0x01c580f0 ConVar joy_advaxisv
r5apex.exe!0x01c50a20 ConVar joy_advaxisx
r5apex.exe!0x01c531f0 ConVar joy_advaxisy
r5apex.exe!0x01c50670 ConVar joy_advaxisz
r5apex.exe!0x01c577d0 ConVar joy_inverty
r5apex.exe!0x01c592b0 ConVar joy_legacy
r5apex.exe!0x01c4b590 ConVar joy_movement_stick
r5apex.exe!0x01c45740 ConVar joy_requireFocus
r5apex.exe!0x01c53d10 ConVar joy_rumble
r5apex.exe!0x01c55780 ConVar joy_xcontroller_cfg_loaded
r5apex.exe!0x0126dc10 ConVar jpeg_quality
r5apex.exe!0x0188fae0 ConVar jt_help_with_anything_ignore_preference
r5apex.exe!0x01c9b580 ConVar jump_graceperiod
r5apex.exe!0x01c98720 ConVar jump_keyboardgrace_max
r5apex.exe!0x01c81b70 ConVar jump_keyboardgrace_strength
r5apex.exe!0x01c680f0 ConVar jump_keyboardgraceperiodmax
r5apex.exe!0x01c98b70 ConVar jump_keyboardgraceperiodmin
r5apex.exe!0x012a68b0 ConVar killReplay_lagCompensate
r5apex.exe!0x01e7fa70 ConVar killReplay_playNonReplayRemoteCallsOnLocalClientPlayer
r5apex.exe!0x018b59e0 ConVar leaf_threadedRecompute
r5apex.exe!0x018b0800 ConVar leaf_threadedRecompute_batchSize
r5apex.exe!0x01e82610 ConVar leech_npc_angle_cos
r5apex.exe!0x01e91a80 ConVar lerp_careAboutAttachmentBonePosition
r5apex.exe!0x01c63ec0 ConVar lerp_debugEnt
r5apex.exe!0x01e924c0 ConVar lerp_opt
r5apex.exe!0x01e90780 ConVar lerp_threaded
r5apex.exe!0x01e919e0 ConVar lerp_threaded_numEntsPerTask
r5apex.exe!0x0119d500 ConVar light_maxcone
r5apex.exe!0x012e8b30 ConVar lightmap_realtimelight
r5apex.exe!0x012ec110 ConVar lightmap_realtimeshadows
r5apex.exe!0x012d7aa0 ConVar load_during_video
r5apex.exe!0x01e7c070 ConVar loaderrorsCount
r5apex.exe!0x01e7b950 ConVar loaderrorsNeedShown
r5apex.exe!0x01e7c570 ConVar localClientPlayerCachedLevel
r5apex.exe!0x0126f4b0 ConVar locationInfo
r5apex.exe!0x0126f410 ConVar locationInfo_nucleus
r5apex.exe!0x01c495e0 ConVar locator_background_border_color
r5apex.exe!0x01c45400 ConVar locator_background_border_thickness
r5apex.exe!0x01c4be30 ConVar locator_background_color
r5apex.exe!0x01c4c070 ConVar locator_background_shift_x
r5apex.exe!0x01c4cea0 ConVar locator_background_shift_y
r5apex.exe!0x01c507b0 ConVar locator_background_style
r5apex.exe!0x01c4cd80 ConVar locator_background_thickness_x
r5apex.exe!0x01c460c0 ConVar locator_background_thickness_y
r5apex.exe!0x01c4a900 ConVar locator_fade_time
r5apex.exe!0x01c576b0 ConVar locator_icon_max_size_non_ss
r5apex.exe!0x01c51020 ConVar locator_icon_min_size_non_ss
r5apex.exe!0x01c4aac0 ConVar locator_lerp_rest
r5apex.exe!0x01c4d740 ConVar locator_lerp_speed
r5apex.exe!0x01c4f820 ConVar locator_lerp_time
r5apex.exe!0x01c538f0 ConVar locator_pulse_time
r5apex.exe!0x01c45b00 ConVar locator_split_len
r5apex.exe!0x01c4d9a0 ConVar locator_split_maxwide_percent
r5apex.exe!0x01c53290 ConVar locator_start_at_crosshair
r5apex.exe!0x01c517c0 ConVar locator_target_offset_x
r5apex.exe!0x01c48cb0 ConVar locator_target_offset_y
r5apex.exe!0x01c55f70 ConVar locator_topdown_style
r5apex.exe!0x01c51d90 ConVar lookspring
r5apex.exe!0x01c566d0 ConVar lookstrafe
r5apex.exe!0x01c4c4d0 ConVar m_acceleration
r5apex.exe!0x01c565b0 ConVar m_forward
r5apex.exe!0x01c4b1f0 ConVar m_invert_pitch
r5apex.exe!0x01c50980 ConVar m_side
r5apex.exe!0x01e7bc70 ConVar mainmenu_background_movie
r5apex.exe!0x01c559a0 ConVar map_settings_override
r5apex.exe!0x01c5b910 ConVar mat_autoexposure_compensation
r5apex.exe!0x0188cf40 ConVar mat_autoexposure_force_value
r5apex.exe!0x01c60840 ConVar mat_autoexposure_max
r5apex.exe!0x01c5e6c0 ConVar mat_autoexposure_max_multiplier
r5apex.exe!0x01c61dc0 ConVar mat_autoexposure_min
r5apex.exe!0x01c5ede0 ConVar mat_autoexposure_min_multiplier
r5apex.exe!0x01c5d070 ConVar mat_autoexposure_speed
r5apex.exe!0x01c62950 ConVar mat_autoexposure_uncap
r5apex.exe!0x01c61980 ConVar mat_bloom_cutoff
r5apex.exe!0x0188d080 ConVar mat_bloom_max_lighting_value
r5apex.exe!0x01c5c120 ConVar mat_bloom_scalefactor_scalar
r5apex.exe!0x012edbe0 ConVar mat_bloom_streak_amount
r5apex.exe!0x01c5f980 ConVar mat_bloom_streak_cutoff
r5apex.exe!0x01c5efc0 ConVar mat_bloom_streak_cutoff_exposure_adapt
r5apex.exe!0x012eec20 ConVar mat_bloom_streak_exponent_post
r5apex.exe!0x01c5e880 ConVar mat_bloom_streak_exponent_pre
r5apex.exe!0x012ee720 ConVar mat_bloom_wide_amount
r5apex.exe!0x01c5f840 ConVar mat_bloom_wide_exponent_pre
r5apex.exe!0x01c61580 ConVar mat_bloomamount_rate
r5apex.exe!0x01c62e70 ConVar mat_bloomscale
r5apex.exe!0x012f3400 ConVar mat_checkStalls
r5apex.exe!0x012e8e30 ConVar mat_cloudmask
r5apex.exe!0x0119a950 ConVar mat_colcorrection_disableentities
r5apex.exe!0x012a5f70 ConVar mat_colcorrection_disableentities
r5apex.exe!0x01c39b50 ConVar mat_colcorrection_disableentities
r5apex.exe!0x011991f0 ConVar mat_colcorrection_editor
r5apex.exe!0x018ad410 ConVar mat_colcorrection_editor
r5apex.exe!0x018ada30 ConVar mat_colcorrection_forceentitiesclientside
r5apex.exe!0x01199900 ConVar mat_colorcorrection
r5apex.exe!0x012eecc0 ConVar mat_debug_postprocess_allowed
r5apex.exe!0x01c5fda0 ConVar mat_debug_postprocessing_effects
r5apex.exe!0x012eea40 ConVar mat_debug_tonemapping
r5apex.exe!0x012ee540 ConVar mat_debug_tonemapping_disable
r5apex.exe!0x012ee400 ConVar mat_debug_tonemapping_mid1
r5apex.exe!0x012ee0e0 ConVar mat_debug_tonemapping_mid2
r5apex.exe!0x012ee2c0 ConVar mat_debug_tonemapping_shoulder
r5apex.exe!0x012ee220 ConVar mat_debug_tonemapping_toe
r5apex.exe!0x012ec410 ConVar mat_debugalttab
r5apex.exe!0x0188d9d0 ConVar mat_depthbias_decal
r5apex.exe!0x0188dbb0 ConVar mat_depthbias_normal
r5apex.exe!0x0188d890 ConVar mat_depthbias_shadowmap
r5apex.exe!0x0188da70 ConVar mat_depthbias_tightshadowmap
r5apex.exe!0x0188d570 ConVar mat_depthbias_ui
r5apex.exe!0x0188db10 ConVar mat_depthbias_zfill
r5apex.exe!0x0188d430 ConVar mat_depthbiasclamp_decal
r5apex.exe!0x0188de30 ConVar mat_depthbiasclamp_normal
r5apex.exe!0x0188d610 ConVar mat_depthbiasclamp_shadowmap
r5apex.exe!0x0188d7f0 ConVar mat_depthbiasclamp_ui
r5apex.exe!0x0188d750 ConVar mat_depthbiasclamp_zfill
r5apex.exe!0x0188d4d0 ConVar mat_depthtest_force_disabled
r5apex.exe!0x012ebf30 ConVar mat_detail_tex
r5apex.exe!0x012e2fb0 ConVar mat_diffuse
r5apex.exe!0x01c5f560 ConVar mat_disable_bloom
r5apex.exe!0x0188cea0 ConVar mat_disable_lightmap_ambient
r5apex.exe!0x012eb170 ConVar mat_disable_lightmaps
r5apex.exe!0x012e65b0 ConVar mat_disable_model_ambient
r5apex.exe!0x011a7870 ConVar mat_drawMenuGrid
r5apex.exe!0x011aa360 ConVar mat_drawTitleSafe
r5apex.exe!0x012e3230 ConVar mat_drawflat
r5apex.exe!0x012e8d90 ConVar mat_dxlevel
r5apex.exe!0x01199290 ConVar mat_dynamic_tonemapping
r5apex.exe!0x012ec7d0 ConVar mat_dynamic_tonemapping
r5apex.exe!0x012eeea0 ConVar mat_enable_ssr
r5apex.exe!0x0188d120 ConVar mat_envmap_scale
r5apex.exe!0x01198350 ConVar mat_envmaptgasize
r5apex.exe!0x012eb990 ConVar mat_fastnobump
r5apex.exe!0x011a31d0 ConVar mat_fastspecular
r5apex.exe!0x012ebdf0 ConVar mat_filterlightmaps
r5apex.exe!0x012e63f0 ConVar mat_filtertextures
r5apex.exe!0x01c5c470 ConVar mat_force_bloom
r5apex.exe!0x012eb8f0 ConVar mat_forceaniso
r5apex.exe!0x01c5cc80 ConVar mat_frame_color_bias
r5apex.exe!0x01c5e260 ConVar mat_frame_color_enabled
r5apex.exe!0x01c62a90 ConVar mat_frame_color_scale
r5apex.exe!0x01c5d2b0 ConVar mat_frame_color_spot_metering_screen_ratio
r5apex.exe!0x011a17e0 ConVar mat_fullbright
r5apex.exe!0x012ee860 ConVar mat_fxaa_enable
r5apex.exe!0x012e2f10 ConVar mat_global_lighting
r5apex.exe!0x012f3700 ConVar mat_global_lighting
r5apex.exe!0x01cafe50 ConVar mat_global_lighting
r5apex.exe!0x011a0f40 ConVar mat_hdr_level
r5apex.exe!0x012a6010 ConVar mat_hdrcolcorrection_editor
r5apex.exe!0x012e2dd0 ConVar mat_hdrcolorcorrection
r5apex.exe!0x012f3850 ConVar mat_hide_sun_in_last_cascade
r5apex.exe!0x012f37b0 ConVar mat_instancing
r5apex.exe!0x012f3360 ConVar mat_letterbox_aspect_goal
r5apex.exe!0x012f30a0 ConVar mat_letterbox_aspect_threshold
r5apex.exe!0x01cb0170 ConVar mat_lightcull_subview
r5apex.exe!0x01e64460 ConVar mat_lightcull_subviews
r5apex.exe!0x012eeae0 ConVar mat_local_contrast_edge_scale_override
r5apex.exe!0x012ede60 ConVar mat_local_contrast_midtone_mask_override
r5apex.exe!0x012edd20 ConVar mat_local_contrast_scale_override
r5apex.exe!0x012edf00 ConVar mat_local_contrast_vignette_end_override
r5apex.exe!0x012edfa0 ConVar mat_local_contrast_vignette_start_override
r5apex.exe!0x012ed7a0 ConVar mat_materialmip_character_0
r5apex.exe!0x012ecaf0 ConVar mat_materialmip_character_1
r5apex.exe!0x012eceb0 ConVar mat_materialmip_character_2
r5apex.exe!0x012ed270 ConVar mat_materialmip_character_3
r5apex.exe!0x012ed1d0 ConVar mat_materialmip_character_4
r5apex.exe!0x012ed520 ConVar mat_materialmip_cockpit_0
r5apex.exe!0x012eccd0 ConVar mat_materialmip_cockpit_1
r5apex.exe!0x012ec910 ConVar mat_materialmip_cockpit_2
r5apex.exe!0x012eca50 ConVar mat_materialmip_cockpit_3
r5apex.exe!0x012ed660 ConVar mat_materialmip_cockpit_4
r5apex.exe!0x012ec9b0 ConVar mat_materialmip_model_0
r5apex.exe!0x012ecff0 ConVar mat_materialmip_model_1
r5apex.exe!0x012ed700 ConVar mat_materialmip_model_2
r5apex.exe!0x012eda00 ConVar mat_materialmip_model_3
r5apex.exe!0x012ed130 ConVar mat_materialmip_model_4
r5apex.exe!0x012ed5c0 ConVar mat_materialmip_other_0
r5apex.exe!0x012ece10 ConVar mat_materialmip_other_1
r5apex.exe!0x012edaa0 ConVar mat_materialmip_other_2
r5apex.exe!0x012ecb90 ConVar mat_materialmip_other_3
r5apex.exe!0x012ed8e0 ConVar mat_materialmip_other_4
r5apex.exe!0x012ed090 ConVar mat_materialmip_world_0
r5apex.exe!0x012ec870 ConVar mat_materialmip_world_1
r5apex.exe!0x012ecc30 ConVar mat_materialmip_world_2
r5apex.exe!0x012ed840 ConVar mat_materialmip_world_3
r5apex.exe!0x012ecd70 ConVar mat_materialmip_world_4
r5apex.exe!0x0119e4e0 ConVar mat_maxframelatency
r5apex.exe!0x012e8c50 ConVar mat_mip_linear
r5apex.exe!0x012e8ed0 ConVar mat_mipmaptextures
r5apex.exe!0x0119c250 ConVar mat_norendering
r5apex.exe!0x012e3050 ConVar mat_norendering
r5apex.exe!0x012eb490 ConVar mat_phong
r5apex.exe!0x012e6650 ConVar mat_picmip
r5apex.exe!0x012ee7c0 ConVar mat_postprocess_enable
r5apex.exe!0x01c60a00 ConVar mat_postprocess_enable
r5apex.exe!0x012ebb70 ConVar mat_proxy
r5apex.exe!0x012ebc10 ConVar mat_reducefillrate
r5apex.exe!0x012ec730 ConVar mat_report_queue_status
r5apex.exe!0x012eb030 ConVar mat_reversedepth
r5apex.exe!0x01c5cb90 ConVar mat_screen_blur_enabled
r5apex.exe!0x012ee180 ConVar mat_screen_blur_override
r5apex.exe!0x0119e080 ConVar mat_shadowstate
r5apex.exe!0x012edb40 ConVar mat_sharpen_amount
r5apex.exe!0x012ee680 ConVar mat_sharpen_threshold
r5apex.exe!0x012ee9a0 ConVar mat_sharpen_width
r5apex.exe!0x012a4460 ConVar mat_show_texture_memory_usage
r5apex.exe!0x012ef1d0 ConVar mat_showenvmapmask
r5apex.exe!0x012e2e70 ConVar mat_showlowresimage
r5apex.exe!0x012eb3f0 ConVar mat_showmiplevels
r5apex.exe!0x011a0510 ConVar mat_skipid
r5apex.exe!0x011a2ed0 ConVar mat_sky_color
r5apex.exe!0x011a01f0 ConVar mat_sky_scale
r5apex.exe!0x0188dd90 ConVar mat_slopescaledepthbias_decal
r5apex.exe!0x0188d6b0 ConVar mat_slopescaledepthbias_normal
r5apex.exe!0x0188dcf0 ConVar mat_slopescaledepthbias_shadowmap
r5apex.exe!0x0188d930 ConVar mat_slopescaledepthbias_ui
r5apex.exe!0x0188dc50 ConVar mat_slopescaledepthbias_zfill
r5apex.exe!0x011a2d50 ConVar mat_sun_color
r5apex.exe!0x0119ec60 ConVar mat_sun_scale
r5apex.exe!0x011a0950 ConVar mat_surfacefilter
r5apex.exe!0x0119fc50 ConVar mat_surfaceid
r5apex.exe!0x011a3310 ConVar mat_surfacemat
r5apex.exe!0x012f35c0 ConVar mat_syncGPU
r5apex.exe!0x012f3520 ConVar mat_syncInterval
r5apex.exe!0x012eb670 ConVar mat_sync_rt
r5apex.exe!0x012ef080 ConVar mat_sync_rt_flushes_gpu
r5apex.exe!0x012a4320 ConVar mat_texture_list
r5apex.exe!0x012a43c0 ConVar mat_texture_list_view
r5apex.exe!0x012ef120 ConVar mat_translucency_errors
r5apex.exe!0x012eeb80 ConVar mat_vignette_enable
r5apex.exe!0x012ecf50 ConVar mat_warn_texture_convert
r5apex.exe!0x0119d6b0 ConVar match_backingOutMaxTimeToWait
r5apex.exe!0x011a2a30 ConVar match_backoutslow
r5apex.exe!0x0119cb30 ConVar match_connect
r5apex.exe!0x0119ea00 ConVar match_defaultMap_party
r5apex.exe!0x011a3570 ConVar match_dir
r5apex.exe!0x0119f2e0 ConVar match_dumpSearchResults
r5apex.exe!0x0119c570 ConVar match_emptyUpdateRate
r5apex.exe!0x011a41c0 ConVar match_enabled
r5apex.exe!0x0119e8c0 ConVar match_fakePort
r5apex.exe!0x0119c6b0 ConVar match_fakeS2SPort
r5apex.exe!0x011a5420 ConVar match_forceVerboseSearches
r5apex.exe!0x011a0c50 ConVar match_goodReputation
r5apex.exe!0x011a1600 ConVar match_maxPingsSent
r5apex.exe!0x011a3450 ConVar match_myBestDatacenter
r5apex.exe!0x011a33b0 ConVar match_myDatacenter
r5apex.exe!0x011a3730 ConVar match_myRankedDatacenter
r5apex.exe!0x0119d140 ConVar match_myTeam
r5apex.exe!0x011a27b0 ConVar match_partyChangeNum
r5apex.exe!0x0119cf90 ConVar match_partySize
r5apex.exe!0x0119c7f0 ConVar match_partySub
r5apex.exe!0x0119bee0 ConVar match_pingWaveInterval
r5apex.exe!0x011a4080 ConVar match_playlist
r5apex.exe!0x0119db70 ConVar match_precachemap
r5apex.exe!0x0119ee40 ConVar match_privateMatchListWithStryder
r5apex.exe!0x0119fb10 ConVar match_rankedMaxPing
r5apex.exe!0x0119d750 ConVar match_rankedSwitchETA
r5apex.exe!0x011a3bd0 ConVar match_resetPlaylistBetweenMatches
r5apex.exe!0x011a2b70 ConVar match_roleToken
r5apex.exe!0x0119fd90 ConVar match_searchInterval
r5apex.exe!0x011a4620 ConVar match_searching
r5apex.exe!0x011a1880 ConVar match_teamNoFill
r5apex.exe!0x011a2710 ConVar match_updateNotableRate
r5apex.exe!0x011a2850 ConVar match_updateRate
r5apex.exe!0x011a0fe0 ConVar match_useMatchmaking
r5apex.exe!0x011a3b30 ConVar match_verbosePrintsInterval
r5apex.exe!0x0119f1a0 ConVar match_visiblePlaylists
r5apex.exe!0x011a0810 ConVar matchmaking_hostname
r5apex.exe!0x01ca46c0 ConVar max_explosive_damage_mass
r5apex.exe!0x01ca88c0 ConVar max_explosive_damage_velocity
r5apex.exe!0x018a0230 ConVar max_tweak_shadow_updates
r5apex.exe!0x01e824d0 ConVar melee_aim_assist_can_lock_pitch
r5apex.exe!0x01e87230 ConVar melee_aim_assist_use_target_velocity
r5apex.exe!0x01e807e0 ConVar melee_attack_trace_can_use_lunge_distance
r5apex.exe!0x01e82180 ConVar melee_cone_trace_box_check
r5apex.exe!0x01c96cb0 ConVar melee_lunge_abort_distance
r5apex.exe!0x01c67420 ConVar melee_lunge_abort_if_blocked
r5apex.exe!0x01e86290 ConVar melee_lunge_adjust_trace_distance
r5apex.exe!0x01e86390 ConVar melee_lunge_align_eye_position
r5apex.exe!0x01e82430 ConVar melee_lunge_dot_check
r5apex.exe!0x01c9b900 ConVar melee_lunge_force_enable_flying
r5apex.exe!0x01c9dc70 ConVar melee_lunge_lag_compensate_target
r5apex.exe!0x01e8a340 ConVar melee_lunge_scale_by_speed
r5apex.exe!0x01c69130 ConVar melee_lunge_slide
r5apex.exe!0x01c63f60 ConVar melee_lunge_use_closest_distance_between_cylinders
r5apex.exe!0x01e87e70 ConVar melee_lunge_use_command_time
r5apex.exe!0x01cac320 ConVar melee_queue_attack_anim_event
r5apex.exe!0x012d83e0 ConVar mem_dumpstats
r5apex.exe!0x011979b0 ConVar mem_force_flush
r5apex.exe!0x01197910 ConVar mem_force_flush_section
r5apex.exe!0x012d6a20 ConVar mem_incremental_compact_rate
r5apex.exe!0x012e8950 ConVar mem_level
r5apex.exe!0x018aa110 ConVar mem_level
r5apex.exe!0x011a8e30 ConVar mem_max_heapsize
r5apex.exe!0x011a71d0 ConVar mem_max_heapsize_dedicated
r5apex.exe!0x011a69c0 ConVar mem_min_heapsize
r5apex.exe!0x01890970 ConVar mem_runheapchecks
r5apex.exe!0x012d7360 ConVar mem_test_each_frame
r5apex.exe!0x012d7700 ConVar mem_test_every_n_seconds
r5apex.exe!0x012d6980 ConVar mem_test_quiet
r5apex.exe!0x01e69230 ConVar menu_faq_community_version
r5apex.exe!0x01e68bd0 ConVar menu_faq_patchnotes_version
r5apex.exe!0x01e68100 ConVar menu_faq_viewed
r5apex.exe!0x01e68f30 ConVar menu_was_multiplayer_played_last
r5apex.exe!0x012d95f0 ConVar migrate_attempt_interval
r5apex.exe!0x012d9410 ConVar migrate_attempt_max_retries
r5apex.exe!0x01e7eaa0 ConVar miles_actor_occlusion_radius
r5apex.exe!0x01e7f4d0 ConVar miles_channels
r5apex.exe!0x018add50 ConVar miles_flip_active_window_logic
r5apex.exe!0x01e7d1f0 ConVar miles_force_emitter_environment
r5apex.exe!0x01e7e960 ConVar miles_force_listener_environment
r5apex.exe!0x01e802a0 ConVar miles_freeze
r5apex.exe!0x01e7f930 ConVar miles_initial_occlusion_delay
r5apex.exe!0x01e7f320 ConVar miles_language
r5apex.exe!0x01e7d3d0 ConVar miles_listener_freeze
r5apex.exe!0x01e7ebe0 ConVar miles_nonactor_occlusion
r5apex.exe!0x01e7fb50 ConVar miles_nonactor_occlusion_radius
r5apex.exe!0x01e7ed50 ConVar miles_nopandist
r5apex.exe!0x01e7f9d0 ConVar miles_occlusion
r5apex.exe!0x01e7f0a0 ConVar miles_occlusion_force
r5apex.exe!0x01e7d870 ConVar miles_occlusion_partial
r5apex.exe!0x01e7e5a0 ConVar miles_occlusion_use_reset_after_deferred_initial
r5apex.exe!0x01e80160 ConVar miles_samplerate
r5apex.exe!0x011ac450 ConVar miles_server_sounds_debug
r5apex.exe!0x011ac4f0 ConVar miles_server_sounds_print
r5apex.exe!0x01e7d330 ConVar miles_solo_ents
r5apex.exe!0x01c481e0 ConVar miles_soundscape_imgui
r5apex.exe!0x01e7e320 ConVar miles_spatialize_front_degrees
r5apex.exe!0x01e7fcc0 ConVar miles_spatialize_offplane_strength
r5apex.exe!0x01e7e500 ConVar miles_spatialize_on
r5apex.exe!0x01e7ee90 ConVar miles_spatialize_rear_degrees
r5apex.exe!0x01e7fed0 ConVar miles_suffixes
r5apex.exe!0x01ca5bd0 ConVar min_explosive_damage_mass
r5apex.exe!0x01e872d0 ConVar missile_default_speed
r5apex.exe!0x01e86b70 ConVar missile_homing_speed
r5apex.exe!0x01e95260 ConVar mod_trace_load
r5apex.exe!0x011aaa10 ConVar model_defaultFadeDistMin
r5apex.exe!0x01c44660 ConVar model_defaultFadeDistMin
r5apex.exe!0x011a74b0 ConVar model_defaultFadeDistScale
r5apex.exe!0x018aae70 ConVar model_defaultFadeDistScale
r5apex.exe!0x01c452c0 ConVar model_fadeRangeFraction
r5apex.exe!0x01c45360 ConVar model_fadeRangeFractionNear
r5apex.exe!0x01cb00d0 ConVar monitor_cc
r5apex.exe!0x012edc80 ConVar monitor_mat_sharpen_amount
r5apex.exe!0x01cb0650 ConVar monitor_postfx
r5apex.exe!0x01e64aa0 ConVar monitor_rui_world_enabled
r5apex.exe!0x0189ff50 ConVar monitor_snapshot_frame_delay
r5apex.exe!0x018a7d60 ConVar monitor_zfar_default
r5apex.exe!0x01e648c0 ConVar monitor_zfar_override
r5apex.exe!0x01cafdb0 ConVar monitor_zfar_override_enabled
r5apex.exe!0x0119ff70 ConVar motd
r5apex.exe!0x01c56090 ConVar mouse_sensitivity
r5apex.exe!0x01c4abb0 ConVar mouse_use_per_scope_sensitivity_scalars
r5apex.exe!0x01c4ac50 ConVar mouse_zoomed_sensitivity_scalar_0
r5apex.exe!0x01c4acf0 ConVar mouse_zoomed_sensitivity_scalar_1
r5apex.exe!0x01c4ad90 ConVar mouse_zoomed_sensitivity_scalar_2
r5apex.exe!0x01c4ae30 ConVar mouse_zoomed_sensitivity_scalar_3
r5apex.exe!0x01c4aed0 ConVar mouse_zoomed_sensitivity_scalar_4
r5apex.exe!0x01c4af70 ConVar mouse_zoomed_sensitivity_scalar_5
r5apex.exe!0x01c4b010 ConVar mouse_zoomed_sensitivity_scalar_6
r5apex.exe!0x01c4b0b0 ConVar mouse_zoomed_sensitivity_scalar_7
r5apex.exe!0x012d8720 ConVar move_one_cmd_per_client_frame
r5apex.exe!0x01e7aab0 ConVar movement_anim_downed_playback_maxrate
r5apex.exe!0x01e7a250 ConVar movement_anim_playback_maxrate
r5apex.exe!0x01e7a110 ConVar movement_anim_playback_minrate
r5apex.exe!0x01e7a610 ConVar movement_anim_sprint_playback_maxrate
r5apex.exe!0x011a43a0 ConVar mp_accountLink_requestInterval
r5apex.exe!0x0119ce10 ConVar mp_allowed
r5apex.exe!0x01e7a750 ConVar mp_bodyyawrate
r5apex.exe!0x012a5330 ConVar mp_countRRNobodyAsLobby
r5apex.exe!0x01c98f20 ConVar mp_enablematchending
r5apex.exe!0x01c67100 ConVar mp_enabletimelimit
r5apex.exe!0x01c9fd10 ConVar mp_gamemode
r5apex.exe!0x012dbc10 ConVar mp_linkingAccountTime
r5apex.exe!0x012dbb70 ConVar mp_linkingAccountWindow
r5apex.exe!0x01e7a390 ConVar mp_maxbodyyaw
r5apex.exe!0x011a5100 ConVar mp_permission_requestInterval
r5apex.exe!0x011a39b0 ConVar mp_permission_rerequestInterval
r5apex.exe!0x01c68ef0 ConVar mp_player_level
r5apex.exe!0x01e7a570 ConVar mp_scaleAnimationSpeeds
r5apex.exe!0x01e7a6b0 ConVar mp_showgestureslots
r5apex.exe!0x011ac630 ConVar mtx_svEdition
r5apex.exe!0x01e8a240 ConVar muteWeaponSounds
r5apex.exe!0x012a5c50 ConVar name
r5apex.exe!0x018ae950 ConVar net_RunInvalidatePhysics
r5apex.exe!0x011a1ed0 ConVar net_async_sendto
r5apex.exe!0x011a08b0 ConVar net_autoUnthrottle
r5apex.exe!0x011a5fa0 ConVar net_bandwidthPrintThreshold
r5apex.exe!0x011a4d40 ConVar net_bindToSpecificAddress
r5apex.exe!0x0119c9b0 ConVar net_blockmsg
r5apex.exe!0x012dce10 ConVar net_chatThroughChatserver
r5apex.exe!0x011a5880 ConVar net_chokeloop
r5apex.exe!0x011a0770 ConVar net_clearReliableDataOnReset
r5apex.exe!0x01e899c0 ConVar net_client_side_weapon_animations
r5apex.exe!0x011a5dc0 ConVar net_compressDataBlock
r5apex.exe!0x011a2010 ConVar net_compressLZValue
r5apex.exe!0x0119fe30 ConVar net_compresspackets
r5apex.exe!0x0119dcb0 ConVar net_compresspackets_minsize
r5apex.exe!0x012a4df0 ConVar net_connectPacketWarningThreshhold
r5apex.exe!0x012a83b0 ConVar net_connectingDataRate
r5apex.exe!0x0119a5b0 ConVar net_createUndoDeltas
r5apex.exe!0x012aa3d0 ConVar net_data_block_enabled
r5apex.exe!0x011a3f40 ConVar net_datablockPrintSummaries
r5apex.exe!0x012a6b30 ConVar net_datablock_fastRate
r5apex.exe!0x011a00b0 ConVar net_datablock_longSendTime
r5apex.exe!0x011a4ac0 ConVar net_datablock_minResendInterval
r5apex.exe!0x012a86b0 ConVar net_datablock_networkLossForSlowSpeed
r5apex.exe!0x012a7010 ConVar net_datablock_resendRateForSlowSpeed
r5apex.exe!0x012a96b0 ConVar net_datablock_slowRate
r5apex.exe!0x0119c430 ConVar net_debugDataBlockReceiver
r5apex.exe!0x011a4120 ConVar net_debugDataBlockSender
r5apex.exe!0x011accf0 ConVar net_debugLerping
r5apex.exe!0x012a5930 ConVar net_deltaFieldEntityBlockSize
r5apex.exe!0x012a4710 ConVar net_disconnectIfDeltaBufferIsFull
r5apex.exe!0x0119bf80 ConVar net_drawslider
r5apex.exe!0x011a24b0 ConVar net_droppackets
r5apex.exe!0x0119a9f0 ConVar net_dumpChangesPrecise
r5apex.exe!0x0119e120 ConVar net_encrypt_copyCtx
r5apex.exe!0x011a0290 ConVar net_encryptionDebug
r5apex.exe!0x012a45d0 ConVar net_forceDeltaBufferToOverflow
r5apex.exe!0x01199740 ConVar net_forceUnnecessaryUndoDeltas
r5apex.exe!0x011a3e00 ConVar net_forcetimeout
r5apex.exe!0x011a5ba0 ConVar net_fullyConnectedDataRate
r5apex.exe!0x0126e6e0 ConVar net_highPacketLatencyThreshold
r5apex.exe!0x0126d6e0 ConVar net_highPacketLossThreshold
r5apex.exe!0x0126d2a0 ConVar net_ignoreAllSnapshots
r5apex.exe!0x012a4c10 ConVar net_largeSnapshotThreshold
r5apex.exe!0x011aced0 ConVar net_lerpFields
r5apex.exe!0x012a8270 ConVar net_lowBandwidthConnect
r5apex.exe!0x0119c4d0 ConVar net_maxAccumulatedClearTimeBalance
r5apex.exe!0x0119d1e0 ConVar net_maxcleartime
r5apex.exe!0x0119df40 ConVar net_maxfilesize
r5apex.exe!0x0119d280 ConVar net_maxfragments
r5apex.exe!0x0119c750 ConVar net_maxroutable
r5apex.exe!0x0119cbd0 ConVar net_minConnectionTimeForSpam
r5apex.exe!0x011a3690 ConVar net_minQueuedPacketsForPrint
r5apex.exe!0x01e698f0 ConVar net_minResetIdleTimerInterval
r5apex.exe!0x0119d460 ConVar net_minimumPacketLossDC
r5apex.exe!0x0119d320 ConVar net_minroutable
r5apex.exe!0x0126e180 ConVar net_netGraph2
r5apex.exe!0x012a5a70 ConVar net_noPostDataForDeletedEnts
r5apex.exe!0x01ca4620 ConVar net_old_seed_generation
r5apex.exe!0x012a7eb0 ConVar net_optimize_persistent_data
r5apex.exe!0x012aa150 ConVar net_optimize_playlists
r5apex.exe!0x01e8d140 ConVar net_optimize_weapons
r5apex.exe!0x01e90d80 ConVar net_predictParentEntities
r5apex.exe!0x012a5150 ConVar net_predictedEntsUseFirstAvailableSnapshot
r5apex.exe!0x012a4b70 ConVar net_predictionDebug
r5apex.exe!0x011ad010 ConVar net_pretendSnapshotArrayFull
r5apex.exe!0x0119ed00 ConVar net_printCompression
r5apex.exe!0x012a5290 ConVar net_printOutOfSnapshots
r5apex.exe!0x01199470 ConVar net_printUnnecessaryDeltas
r5apex.exe!0x0119b5f0 ConVar net_propSkipPrintThreshold
r5apex.exe!0x0119eaa0 ConVar net_queue_trace
r5apex.exe!0x0119bbb0 ConVar net_queuedPackets_PrintOversleeps
r5apex.exe!0x011a0650 ConVar net_queuedPackets_SkipSmallSleeps
r5apex.exe!0x0119f060 ConVar net_queued_packet_sender_nopacket_sleep
r5apex.exe!0x0119e260 ConVar net_queued_packet_thread
r5apex.exe!0x0119f7f0 ConVar net_recentNetworkGapWindow
r5apex.exe!0x011a1240 ConVar net_recentNetworkGapsNeeded
r5apex.exe!0x012a48f0 ConVar net_recreateScriptInstanceOnReplayTransition
r5apex.exe!0x011ad150 ConVar net_recv_dumpChanges
r5apex.exe!0x01199df0 ConVar net_recv_dumpNetworkedChangesOnEntCreate
r5apex.exe!0x0119a1b0 ConVar net_recv_watchEnt
r5apex.exe!0x011997e0 ConVar net_recv_watchField1
r5apex.exe!0x0119aef0 ConVar net_recv_watchField2
r5apex.exe!0x011a2ad0 ConVar net_resourcePrintMinimum
r5apex.exe!0x01199070 ConVar net_sendFloatDeltas
r5apex.exe!0x012a9390 ConVar net_sendProfileTotals
r5apex.exe!0x011a2150 ConVar net_sendtoInJob
r5apex.exe!0x011a54c0 ConVar net_showFailedAuth
r5apex.exe!0x012a4e90 ConVar net_showLargeSnapshot
r5apex.exe!0x0119f240 ConVar net_showQueued
r5apex.exe!0x01198770 ConVar net_showUndoDeltas
r5apex.exe!0x0126db00 ConVar net_showUserWarnings
r5apex.exe!0x011a5740 ConVar net_showchoke
r5apex.exe!0x0119c390 ConVar net_showchokeInterval
r5apex.exe!0x0119fcf0 ConVar net_showdrop
r5apex.exe!0x0119c610 ConVar net_showfragments
r5apex.exe!0x011a12e0 ConVar net_showmsg
r5apex.exe!0x0119e3a0 ConVar net_showpeaks
r5apex.exe!0x0119f6b0 ConVar net_showsendrecv
r5apex.exe!0x0119e440 ConVar net_showsplits
r5apex.exe!0x0119fbb0 ConVar net_showudp
r5apex.exe!0x0119c020 ConVar net_showudp_oob
r5apex.exe!0x011a5f00 ConVar net_showudp_remoteonly
r5apex.exe!0x01ca87a0 ConVar net_showusercmd
r5apex.exe!0x01197ee0 ConVar net_skipUnnecessaryDeltas
r5apex.exe!0x011a3010 ConVar net_splitrate
r5apex.exe!0x011a1e30 ConVar net_splitrateDefaultMP
r5apex.exe!0x0119f100 ConVar net_splitrateDefaultSP
r5apex.exe!0x0119bd60 ConVar net_tamperPackets
r5apex.exe!0x012a4a30 ConVar net_threadedEntityDeltas
r5apex.exe!0x012a5b10 ConVar net_threadedProcessPacket
r5apex.exe!0x0119d030 ConVar net_timeoutUsesLastReadTime
r5apex.exe!0x011a25f0 ConVar net_trackerWarningInterval
r5apex.exe!0x011a4b60 ConVar net_usesocketsforloopback
r5apex.exe!0x011a37d0 ConVar net_verifyEncryption
r5apex.exe!0x012dccf0 ConVar net_voiceEchoFromChatServer
r5apex.exe!0x0119cd70 ConVar net_warnAboutSocketReadGaps
r5apex.exe!0x011a3ea0 ConVar net_warnGapTime
r5apex.exe!0x011a4de0 ConVar net_wifi
r5apex.exe!0x012a4850 ConVar net_worldHitchSlopTime
r5apex.exe!0x012d77a0 ConVar next
r5apex.exe!0x01caca60 ConVar noReloadAfterUse
r5apex.exe!0x012ee900 ConVar noise_filter_scale
r5apex.exe!0x011a7310 ConVar not_focus_sleep
r5apex.exe!0x012d50f0 ConVar notification_displayTime
r5apex.exe!0x011a8370 ConVar nucleus_id
r5apex.exe!0x011a79b0 ConVar nucleus_pid
r5apex.exe!0x01c5ba50 ConVar number_shortenToMillionsAfter
r5apex.exe!0x01e86150 ConVar object_placement_debug
r5apex.exe!0x01e8dc20 ConVar offhandTossOverheadPitchThreshold
r5apex.exe!0x01c66f20 ConVar offhand_alignEndAnim1p3p
r5apex.exe!0x011a9d30 ConVar old_culling
r5apex.exe!0x011a7cf0 ConVar old_gather_props
r5apex.exe!0x01c9db40 ConVar one_handed_change_rate
r5apex.exe!0x01e64500 ConVar opaque_renderable_worker
r5apex.exe!0x012d5a70 ConVar openInvite_spam
r5apex.exe!0x012d53d0 ConVar openInvites_filterByLanguage
r5apex.exe!0x012d5690 ConVar openInvites_filterByRegion
r5apex.exe!0x012d9ad0 ConVar openinvite_duration_default
r5apex.exe!0x01c46020 ConVar ordnanceSwapSelectCooldown
r5apex.exe!0x0582dae0 ConVar origin_Errorlevel_OldBehaviour
r5apex.exe!0x0582dd60 ConVar origin_Errorlevel_Telementry
r5apex.exe!0x0582dcc0 ConVar origin_authCodeFailureMaxBackoffSeconds
r5apex.exe!0x05828750 ConVar origin_autoRefreshTokenClient
r5apex.exe!0x0582db80 ConVar origin_autoRefreshTokenServer
r5apex.exe!0x0582d9a0 ConVar origin_debug
r5apex.exe!0x05828610 ConVar origin_disconnectWhenOffline
r5apex.exe!0x058286b0 ConVar origin_ignoreInvitesOnLoadScreen
r5apex.exe!0x058284e0 ConVar origin_igo_mutes_sound_enabled
r5apex.exe!0x018ad070 ConVar origin_igo_muting_sound
r5apex.exe!0x0582da40 ConVar origin_presense_updateRate
r5apex.exe!0x0582dc20 ConVar origin_tokenFailureMaxBackoffSeconds
r5apex.exe!0x01e98c90 ConVar panel_showVisChanges
r5apex.exe!0x01e988e0 ConVar panel_test_title_safe
r5apex.exe!0x01e906e0 ConVar parenting_debug
r5apex.exe!0x01cab450 ConVar particleEffect_checkShouldStillPlay
r5apex.exe!0x01e97070 ConVar particle_alwayswakeonstop
r5apex.exe!0x012de3b0 ConVar particle_cpu_level
r5apex.exe!0x01e97430 ConVar particle_delete_all_except
r5apex.exe!0x01c3b870 ConVar particle_dlights_enable
r5apex.exe!0x01c447a0 ConVar particle_dlights_spew
r5apex.exe!0x01c62bd0 ConVar particle_gpu_level
r5apex.exe!0x012f1480 ConVar particle_lighting_clear_enable
r5apex.exe!0x012f13e0 ConVar particle_lighting_size
r5apex.exe!0x01e96c70 ConVar particle_lighting_viewmodel_enable
r5apex.exe!0x01c5d190 ConVar particle_overlay
r5apex.exe!0x01c5e0a0 ConVar particle_overlay_detail_attributes
r5apex.exe!0x01c61f00 ConVar particle_overlay_detail_filter
r5apex.exe!0x01c61b60 ConVar particle_overlay_detail_list_particles
r5apex.exe!0x01c5ecc0 ConVar particle_overlay_detail_scroll
r5apex.exe!0x01c60340 ConVar particle_overlay_hide_sleeping
r5apex.exe!0x01c60b60 ConVar particle_overlay_list_filter
r5apex.exe!0x01c5cdc0 ConVar particle_overlay_list_tally
r5apex.exe!0x01c5df60 ConVar particle_overlay_list_tally_collapse_children
r5apex.exe!0x01c5f4c0 ConVar particle_overlay_old
r5apex.exe!0x01c5fa20 ConVar particle_overlay_scroll
r5apex.exe!0x01e96630 ConVar particle_remap_vol2cp_debug
r5apex.exe!0x01e957a0 ConVar particle_script_dump
r5apex.exe!0x01e95840 ConVar particle_script_list
r5apex.exe!0x01e958e0 ConVar particle_script_log
r5apex.exe!0x01c5d520 ConVar particle_scrub_debug
r5apex.exe!0x01e96d50 ConVar particle_scrub_debug_effect
r5apex.exe!0x01e97570 ConVar particle_scrub_is_using_time_scrub
r5apex.exe!0x01e97110 ConVar particle_scrub_max_dt
r5apex.exe!0x01e971b0 ConVar particle_scrub_play_speed
r5apex.exe!0x01e97390 ConVar particle_scrub_quality
r5apex.exe!0x01e97250 ConVar particle_scrub_time
r5apex.exe!0x01c608e0 ConVar particle_simulateoverflow
r5apex.exe!0x01e974d0 ConVar particles_cull_dlights
r5apex.exe!0x01e972f0 ConVar particles_max_passes
r5apex.exe!0x01e96df0 ConVar particles_spawncull
r5apex.exe!0x01e96f30 ConVar particles_spawncull_report
r5apex.exe!0x012d9b70 ConVar parties_alwaysReadSubs
r5apex.exe!0x012d9370 ConVar party_autoCreatePartyAlways
r5apex.exe!0x012d9e70 ConVar party_autoCreatePartyDelay
r5apex.exe!0x01e70e70 ConVar party_color_enabled
r5apex.exe!0x012d9690 ConVar party_doRealNameLookups
r5apex.exe!0x012d9550 ConVar party_doRealNameLookupsForOwner
r5apex.exe!0x012d5d50 ConVar party_hostname
r5apex.exe!0x012d94b0 ConVar party_httpHandleTimeout
r5apex.exe!0x012d5730 ConVar party_keepAliveTime
r5apex.exe!0x012d9a30 ConVar party_keepAliveTime
r5apex.exe!0x012d9870 ConVar party_leaderAlwaysDetectsChanges
r5apex.exe!0x012d58d0 ConVar party_leaveMatchOnJoin
r5apex.exe!0x012d9990 ConVar party_lookupRealNamesForOpenInvites
r5apex.exe!0x012d9c10 ConVar party_lookupRealNamesForOpenInvitesForOwner
r5apex.exe!0x012d97d0 ConVar party_minSize
r5apex.exe!0x012d9dd0 ConVar party_privacy
r5apex.exe!0x012d9f10 ConVar party_readyToSearch
r5apex.exe!0x012aa980 ConVar party_relyOnPartyForMemberUserInfo
r5apex.exe!0x012d9fb0 ConVar party_requireConsensusForSearch
r5apex.exe!0x011aabf0 ConVar perTriangleCollisionForced
r5apex.exe!0x0188f900 ConVar perfcharts_killswitch
r5apex.exe!0x0188f9a0 ConVar perfcharts_pin_enabled
r5apex.exe!0x0188fa40 ConVar perfcharts_pin_print
r5apex.exe!0x011a59c0 ConVar persistenceDef_hostname
r5apex.exe!0x012dd870 ConVar persistenceDef_queryMaxHttpRetries
r5apex.exe!0x012dd7d0 ConVar persistenceDef_readMaxHttpRetries
r5apex.exe!0x012dd910 ConVar persistenceDef_retryReadAfterErrorTime
r5apex.exe!0x012dd9b0 ConVar persistenceDef_writeMaxHttpRetries
r5apex.exe!0x012ddaf0 ConVar persistence_clForceNew
r5apex.exe!0x012ddb90 ConVar persistence_disableForBuildProcess
r5apex.exe!0x012ddc30 ConVar persistence_enforce_manifest
r5apex.exe!0x011a4ca0 ConVar persistence_hostname
r5apex.exe!0x012ddd70 ConVar persistence_new_player_if_upgrade_fails
r5apex.exe!0x012dda50 ConVar persistence_upload_def
r5apex.exe!0x012ddcd0 ConVar persistence_upload_failure_is_error
r5apex.exe!0x012a5590 ConVar persistent_warningRate
r5apex.exe!0x011a6c40 ConVar pertrianglecollision
r5apex.exe!0x01e949f0 ConVar phys_bounce
r5apex.exe!0x01e94ad0 ConVar phys_cfm
r5apex.exe!0x01e94810 ConVar phys_cfm_anglejointstop
r5apex.exe!0x01e948b0 ConVar phys_drawContacts
r5apex.exe!0x01e94950 ConVar phys_drawContactsDuration
r5apex.exe!0x01e944f0 ConVar phys_drawGeoms
r5apex.exe!0x01e94630 ConVar phys_drawTunnelChecks
r5apex.exe!0x01e94270 ConVar phys_enableObjectPairCollidePrototype
r5apex.exe!0x01e94130 ConVar phys_erp
r5apex.exe!0x01e94450 ConVar phys_erp_anglejointstop
r5apex.exe!0x01e94770 ConVar phys_frictionDefault
r5apex.exe!0x01c610a0 ConVar phys_showObjectCount
r5apex.exe!0x01e94310 ConVar phys_threadGoWide
r5apex.exe!0x01e94db0 ConVar physics_async_cl
r5apex.exe!0x01e94c70 ConVar physics_autoSleepAngularThreshold
r5apex.exe!0x01e946d0 ConVar physics_autoSleepDebug
r5apex.exe!0x01e94590 ConVar physics_autoSleepGroundHysteresis
r5apex.exe!0x01e94d10 ConVar physics_autoSleepSpeedThreshold
r5apex.exe!0x01e941d0 ConVar physics_collideWithMovingGeo
r5apex.exe!0x01c81ad0 ConVar physics_defaultMaxAngularSpeed
r5apex.exe!0x01c9d540 ConVar physics_defaultMaxSpeed
r5apex.exe!0x01892fc0 ConVar physics_scaled_mem
r5apex.exe!0x01e943b0 ConVar physics_tunnelChecks
r5apex.exe!0x01e94bd0 ConVar physics_tunnelChecksForceAlways
r5apex.exe!0x011aa0e0 ConVar pin_opt_in
r5apex.exe!0x012a5bb0 ConVar pin_plat_id
r5apex.exe!0x011aa400 ConVar pin_sid
r5apex.exe!0x011a6920 ConVar pin_telemetry_actually_send
r5apex.exe!0x011aa720 ConVar pin_telemetry_debug_code
r5apex.exe!0x011a6700 ConVar pin_telemetry_debug_payload
r5apex.exe!0x01e67df0 ConVar pin_telemetry_debug_script
r5apex.exe!0x011a6ba0 ConVar pin_telemetry_dont_send_events
r5apex.exe!0x011a7eb0 ConVar pin_telemetry_hostname
r5apex.exe!0x011a7a50 ConVar pin_telemetry_inactivity_send_time
r5apex.exe!0x011a7c30 ConVar pin_telemetry_max_payload_size
r5apex.exe!0x011a9310 ConVar pin_telemetry_send_debug
r5apex.exe!0x011a1080 ConVar ping_debug
r5apex.exe!0x01e7be90 ConVar ping_max_green
r5apex.exe!0x01e7bd10 ConVar ping_max_red
r5apex.exe!0x01e7c2f0 ConVar ping_max_yellow
r5apex.exe!0x0119fa70 ConVar ping_minSentForChoice
r5apex.exe!0x011a14c0 ConVar ping_qos_units
r5apex.exe!0x011a51a0 ConVar ping_usePacketLoss
r5apex.exe!0x0189fc40 ConVar pixvis_enable
r5apex.exe!0x0188d2e0 ConVar pixvis_maxquads
r5apex.exe!0x0189f9c0 ConVar pixvis_spew
r5apex.exe!0x012d8f40 ConVar plat_environment
r5apex.exe!0x012d4df0 ConVar plat_retryNameLookups
r5apex.exe!0x012a5cf0 ConVar platform_user_id
r5apex.exe!0x01e7b450 ConVar playerListPartyColorB
r5apex.exe!0x01e7c250 ConVar playerListPartyColorG
r5apex.exe!0x01e7b130 ConVar playerListPartyColorR
r5apex.exe!0x01e7c4d0 ConVar playerListUseFriendColor
r5apex.exe!0x01cac7c0 ConVar player_ADS_buffer_time_seconds
r5apex.exe!0x0189eb40 ConVar player_debugPredictedPosition
r5apex.exe!0x01898440 ConVar player_deltaAnimsMakeMeUnpredicted
r5apex.exe!0x0189d7d0 ConVar player_doJetwashEffects
r5apex.exe!0x01c95220 ConVar player_extraairaccelleration
r5apex.exe!0x0189ea00 ConVar player_highFrequencyThinkDistance
r5apex.exe!0x01c9dd10 ConVar player_movementBounds_predictionShare
r5apex.exe!0x01e7a7f0 ConVar player_movingDeathThreshold
r5apex.exe!0x0189dea0 ConVar player_respawnInputDebounceDuration
r5apex.exe!0x01c52070 ConVar player_setting_autosprint
r5apex.exe!0x01e69f00 ConVar player_setting_damage_closes_deathbox_menu
r5apex.exe!0x01cad2d0 ConVar player_setting_stickysprintforward
r5apex.exe!0x0189c930 ConVar player_showEyePosition
r5apex.exe!0x01c639e0 ConVar player_useMovementBounds
r5apex.exe!0x0189dfc0 ConVar player_viewchange_debug_pitch
r5apex.exe!0x0189ebe0 ConVar player_viewchange_debug_roll
r5apex.exe!0x0189d690 ConVar player_viewchange_debug_yaw
r5apex.exe!0x011a9bf0 ConVar playlist_changeGamemodeAutomatically
r5apex.exe!0x011a8930 ConVar playlist_debug
r5apex.exe!0x011a6660 ConVar playlist_debug_getvar
r5apex.exe!0x011a7e10 ConVar playlist_debug_localization
r5apex.exe!0x011a8ed0 ConVar playlist_dump
r5apex.exe!0x011a85f0 ConVar playlist_privateMatchEnabled
r5apex.exe!0x011a65c0 ConVar playlist_rotationGroup
r5apex.exe!0x011a9a70 ConVar playlist_rotationInterval
r5apex.exe!0x011a7020 ConVar playlist_rotationIntervalDefault
r5apex.exe!0x011a9610 ConVar playlist_rotationIntervalOverride
r5apex.exe!0x011a8ff0 ConVar playlist_rotationNextTime
r5apex.exe!0x01c93d20 ConVar playlist_variableErrorsChecks
r5apex.exe!0x01e7d0b0 ConVar portal_pointpush_debug
r5apex.exe!0x01e7d150 ConVar portal_pointpush_think_rate
r5apex.exe!0x01cad0f0 ConVar portal_use_player_avoidance
r5apex.exe!0x01e911a0 ConVar postdataupdate_threaded
r5apex.exe!0x01e90920 ConVar postdataupdate_threaded_chunksize
r5apex.exe!0x011a21f0 ConVar printConnectTimings
r5apex.exe!0x018b9350 ConVar print_timeprefix
r5apex.exe!0x01e861f0 ConVar process_pending_vm_effects
r5apex.exe!0x01e7b270 ConVar progressbar_allow_wrap
r5apex.exe!0x01e7b6d0 ConVar progressbar_high_precision
r5apex.exe!0x01e7b4f0 ConVar progressbar_single_bar
r5apex.exe!0x01e8c160 ConVar projectile_fake_prediction_in_kill_replay
r5apex.exe!0x01e890e0 ConVar projectile_faketrails
r5apex.exe!0x01e89580 ConVar projectile_filltrails
r5apex.exe!0x01e87f10 ConVar projectile_lagCompensationDebug
r5apex.exe!0x01e89440 ConVar projectile_lagCompensationDebugDrawNoReg
r5apex.exe!0x01e88250 ConVar projectile_lagCompensationDebugDrawTime
r5apex.exe!0x01e89300 ConVar projectile_lagCompensationDebugExtra
r5apex.exe!0x01e894e0 ConVar projectile_lagCompensationDebugForceNoReg
r5apex.exe!0x01e881b0 ConVar projectile_lagCompensationDebugServerOffset
r5apex.exe!0x01e820e0 ConVar projectile_lagCompensationMissileTimeStepScalar
r5apex.exe!0x01e89260 ConVar projectile_muzzleOffsetFirstPersonDecayDist
r5apex.exe!0x01e88960 ConVar projectile_muzzleOffsetFirstPersonDecayMaxTime
r5apex.exe!0x01e88840 ConVar projectile_muzzleOffsetThirdPersonDecayDist
r5apex.exe!0x01e893a0 ConVar projectile_muzzleOffsetThirdPersonDecayMaxTime
r5apex.exe!0x01e89180 ConVar projectile_prediction
r5apex.exe!0x01e88f80 ConVar projectile_predictionErrorCorrectTime
r5apex.exe!0x01c39f90 ConVar prop_lightweightPropsSkipAnimData
r5apex.exe!0x0189f7e0 ConVar prop_survivalSkipsAnimData
r5apex.exe!0x01ca54d0 ConVar props_break_burst_rotation
r5apex.exe!0x01ca53c0 ConVar props_break_max_pieces
r5apex.exe!0x01ca93e0 ConVar props_break_max_pieces_perframe
r5apex.exe!0x012d5df0 ConVar publication_hostname
r5apex.exe!0x01e92ec0 ConVar push_cl
r5apex.exe!0x01e90e20 ConVar push_cl_always_update_prev_matrix
r5apex.exe!0x01ca9340 ConVar push_debug
r5apex.exe!0x01ca7b10 ConVar push_debug_ent
r5apex.exe!0x01ca4800 ConVar push_ragdolls
r5apex.exe!0x01e65ee0 ConVar pve_debug
r5apex.exe!0x01caf590 ConVar pvs_addWorkItemsAccum
r5apex.exe!0x01caf950 ConVar pvs_addWorkItemsThreshold_edges
r5apex.exe!0x01cafb30 ConVar pvs_addWorkItemsThreshold_leaves
r5apex.exe!0x01cafbd0 ConVar pvs_cullBoxes
r5apex.exe!0x01caf770 ConVar pvs_debug
r5apex.exe!0x01caf9f0 ConVar pvs_drawPortals
r5apex.exe!0x01caf810 ConVar pvs_frustumCullOnly
r5apex.exe!0x01c448e0 ConVar pvs_start_early
r5apex.exe!0x01c995d0 ConVar r_AirboatViewDampenDamp
r5apex.exe!0x01c9ce10 ConVar r_AirboatViewDampenFreq
r5apex.exe!0x01c9a640 ConVar r_AirboatViewZHeight
r5apex.exe!0x01c9d5e0 ConVar r_JeepViewDampenDamp
r5apex.exe!0x01c9aa20 ConVar r_JeepViewDampenFreq
r5apex.exe!0x01c67b20 ConVar r_VehicleViewDampen
r5apex.exe!0x01caf8b0 ConVar r_WaterDrawReflection
r5apex.exe!0x012eb0d0 ConVar r_WaterDrawRefraction
r5apex.exe!0x0119d5a0 ConVar r_aspectratio
r5apex.exe!0x01cb02b0 ConVar r_blurmenubg
r5apex.exe!0x0188cc20 ConVar r_bone_matrix_bulk_update_threshold
r5apex.exe!0x011a8a70 ConVar r_brush_queue_mode
r5apex.exe!0x011a9750 ConVar r_createmodeldecals
r5apex.exe!0x011a05b0 ConVar r_cullshadowworldmeshes
r5apex.exe!0x01c64140 ConVar r_debug_draw_box_depth_test
r5apex.exe!0x011a9e70 ConVar r_decal_cover_count
r5apex.exe!0x0119ade0 ConVar r_decal_cull_stretch_limit
r5apex.exe!0x011a77d0 ConVar r_decal_draw_basis
r5apex.exe!0x0119a070 ConVar r_decal_drawclipped
r5apex.exe!0x011aa4a0 ConVar r_decal_overlap_area
r5apex.exe!0x011a96b0 ConVar r_decal_overlap_count
r5apex.exe!0x011a7910 ConVar r_decal_test_scale
r5apex.exe!0x011a1420 ConVar r_decals
r5apex.exe!0x012f0070 ConVar r_delay_texture_destroy
r5apex.exe!0x012eb2b0 ConVar r_ditherFade
r5apex.exe!0x018b26b0 ConVar r_ditherFade
r5apex.exe!0x018ae0d0 ConVar r_ditherFadeShadows
r5apex.exe!0x01c445a0 ConVar r_drawallrenderables
r5apex.exe!0x01cb0490 ConVar r_drawalphasort
r5apex.exe!0x011a5600 ConVar r_drawbrushmodels
r5apex.exe!0x01e653a0 ConVar r_drawbrushmodels
r5apex.exe!0x011a93b0 ConVar r_drawdecals
r5apex.exe!0x01e651c0 ConVar r_drawdepth_of_blend2transparent
r5apex.exe!0x01198ec0 ConVar r_drawdlights
r5apex.exe!0x0119e1c0 ConVar r_drawentities
r5apex.exe!0x0119ad40 ConVar r_drawlightdist
r5apex.exe!0x0119a3d0 ConVar r_drawlightinfo
r5apex.exe!0x01896fa0 ConVar r_drawmodelsinzfill
r5apex.exe!0x01e65440 ConVar r_drawopaquerenderables
r5apex.exe!0x01c62b30 ConVar r_drawparticles
r5apex.exe!0x01e90b00 ConVar r_drawrenderboxes
r5apex.exe!0x01cb0210 ConVar r_drawscreenspaceparticles
r5apex.exe!0x01e64820 ConVar r_drawsky
r5apex.exe!0x01e64a00 ConVar r_drawskybox_deprecated
r5apex.exe!0x0119abd0 ConVar r_drawstaticlight
r5apex.exe!0x01e65120 ConVar r_drawstaticprops
r5apex.exe!0x01c4ec80 ConVar r_drawtracers
r5apex.exe!0x011a7b90 ConVar r_drawvgui
r5apex.exe!0x01e64780 ConVar r_drawviewmodel
r5apex.exe!0x011a3c70 ConVar r_drawworld
r5apex.exe!0x0119c0c0 ConVar r_dynamic
r5apex.exe!0x01e64f00 ConVar r_earlyRenderables
r5apex.exe!0x018ade70 ConVar r_enableOriginSort
r5apex.exe!0x01cafef0 ConVar r_fadeincode
r5apex.exe!0x01c44a00 ConVar r_farz
r5apex.exe!0x011a3270 ConVar r_fastzreject
r5apex.exe!0x01caf630 ConVar r_forcecheapwater
r5apex.exe!0x018975c0 ConVar r_jiggle_bones
r5apex.exe!0x011a4580 ConVar r_lightmap
r5apex.exe!0x011a3fe0 ConVar r_lightprobe_force_trans_dist
r5apex.exe!0x011a4800 ConVar r_lightstyle
r5apex.exe!0x011a20b0 ConVar r_lod
r5apex.exe!0x011a8cf0 ConVar r_lod
r5apex.exe!0x01c5f420 ConVar r_lod
r5apex.exe!0x011a6340 ConVar r_lod_shift
r5apex.exe!0x01c5b870 ConVar r_lod_shift
r5apex.exe!0x01e7f890 ConVar r_lod_switch_scale
r5apex.exe!0x018af3b0 ConVar r_mapextents
r5apex.exe!0x0188f4c0 ConVar r_modeldecal_maxtotal
r5apex.exe!0x0189fa60 ConVar r_nearz
r5apex.exe!0x012f3660 ConVar r_no_stalls
r5apex.exe!0x0188ce00 ConVar r_no_stalls
r5apex.exe!0x0188d380 ConVar r_no_stalls
r5apex.exe!0x0119dc10 ConVar r_norefresh
r5apex.exe!0x01e645a0 ConVar r_particle_lighting_debug
r5apex.exe!0x01cafc70 ConVar r_particle_lighting_enable
r5apex.exe!0x01e96900 ConVar r_particle_lighting_enable
r5apex.exe!0x01e95cc0 ConVar r_particle_lighting_force
r5apex.exe!0x01e96bb0 ConVar r_particle_lighting_force
r5apex.exe!0x01caff90 ConVar r_particle_low_res_debug
r5apex.exe!0x012f15c0 ConVar r_particle_low_res_draw_weight_tex
r5apex.exe!0x01e96a00 ConVar r_particle_low_res_enable
r5apex.exe!0x01e96aa0 ConVar r_particle_low_res_force
r5apex.exe!0x012f1520 ConVar r_particle_low_res_tiled_composite
r5apex.exe!0x01c5d9c0 ConVar r_particle_sim_spike_increment_ms
r5apex.exe!0x01c61420 ConVar r_particle_sim_spike_threshold_ms
r5apex.exe!0x01c59b50 ConVar r_particle_timescale
r5apex.exe!0x018a9b90 ConVar r_particles_early_distance_cull
r5apex.exe!0x01e91ba0 ConVar r_pos_debug
r5apex.exe!0x01e91060 ConVar r_render_pos_debug
r5apex.exe!0x012eb7b0 ConVar r_rimlight
r5apex.exe!0x011a4c00 ConVar r_rootlod
r5apex.exe!0x01e95120 ConVar r_rootlod
r5apex.exe!0x01c4b330 ConVar r_ropetranslucent
r5apex.exe!0x01c5f680 ConVar r_setupBoneWorkSize
r5apex.exe!0x01c63050 ConVar r_setupBoneWorkerThreadhold
r5apex.exe!0x012eb850 ConVar r_shadowrendertotexture
r5apex.exe!0x01cb03f0 ConVar r_sky_ignoreAngles
r5apex.exe!0x01c3b9b0 ConVar r_sort_trans_debug
r5apex.exe!0x018ae630 ConVar r_sort_trans_debug_dist
r5apex.exe!0x01c5cd20 ConVar r_threaded_particles
r5apex.exe!0x01c5bbf0 ConVar r_updaterefracttexture
r5apex.exe!0x01c62220 ConVar r_updaterefracttexture_allowmultiple
r5apex.exe!0x01198950 ConVar r_visambient
r5apex.exe!0x0119a510 ConVar r_visambient_orig
r5apex.exe!0x0119ab30 ConVar r_visambient_point
r5apex.exe!0x011aa040 ConVar r_vismodellighting_lightpos
r5apex.exe!0x01c5b730 ConVar r_vismodellighting_maxdist
r5apex.exe!0x01c5f260 ConVar r_vismodellighting_mindist
r5apex.exe!0x011a8c50 ConVar r_vismodellighting_offset_x
r5apex.exe!0x011a75f0 ConVar r_vismodellighting_offset_y
r5apex.exe!0x011a8890 ConVar r_vismodellighting_offset_z
r5apex.exe!0x018b9290 ConVar r_visualizeproplightcaching
r5apex.exe!0x01caa560 ConVar r_visualizetraces
r5apex.exe!0x01ca9fe0 ConVar r_visualizetraces_duration
r5apex.exe!0x012f2ec0 ConVar r_volumetric_lighting_blur_count
r5apex.exe!0x012f2e20 ConVar r_volumetric_lighting_blur_type
r5apex.exe!0x012f2d80 ConVar r_volumetric_lighting_enabled
r5apex.exe!0x012f3000 ConVar r_volumetric_lighting_numSteps
r5apex.exe!0x012f2f60 ConVar r_volumetric_lighting_rotate_dither
r5apex.exe!0x01caf6d0 ConVar r_waterforceexpensive
r5apex.exe!0x01cafa90 ConVar r_waterforcereflectentities
r5apex.exe!0x01e65300 ConVar r_zfill
r5apex.exe!0x01896cc0 ConVar ragdoll_debug
r5apex.exe!0x01c61860 ConVar ragdoll_sleepaftertime
r5apex.exe!0x01e68dd0 ConVar rankedplay_display_enabled
r5apex.exe!0x01e68270 ConVar rankedplay_voice_enabled
r5apex.exe!0x0126f030 ConVar rate
r5apex.exe!0x01cab4f0 ConVar reactive_wakeOnStop
r5apex.exe!0x01c4a140 ConVar real_time_update_dt
r5apex.exe!0x011ad0b0 ConVar recalculateOrigin_threaded_chunksize
r5apex.exe!0x012da210 ConVar reconnect_available_timespan
r5apex.exe!0x012da170 ConVar reconnect_check_platform_id
r5apex.exe!0x012aa330 ConVar reconnect_debug
r5apex.exe!0x012da2b0 ConVar reconnect_delay
r5apex.exe!0x011a46c0 ConVar reconnect_enabled
r5apex.exe!0x01e7e760 ConVar remoteCalls_requireConnectionScriptsForViewPlayer
r5apex.exe!0x011a63e0 ConVar remoteMatchInfo_print
r5apex.exe!0x011aad50 ConVar replay_enable
r5apex.exe!0x011a82d0 ConVar replay_prediction_smooth
r5apex.exe!0x01e92560 ConVar report_cliententitysim
r5apex.exe!0x018ae4f0 ConVar report_clientthinklist
r5apex.exe!0x018a8450 ConVar roamingcam_cameraspacemotion
r5apex.exe!0x018aa390 ConVar roamingcam_downSpeedMult
r5apex.exe!0x018a0490 ConVar roamingcam_forwardSpeed
r5apex.exe!0x01c3b390 ConVar roamingcam_forwardSpeed_fast
r5apex.exe!0x018aa2f0 ConVar roamingcam_forwardSpeed_slow
r5apex.exe!0x018b1410 ConVar roamingcam_movesmoothing
r5apex.exe!0x0189f4d0 ConVar roamingcam_rollSpeed
r5apex.exe!0x018b0260 ConVar roamingcam_sideSpeedMult
r5apex.exe!0x018a08e0 ConVar roamingcam_smoothtime
r5apex.exe!0x018aeb30 ConVar roamingcam_viewsmoothing
r5apex.exe!0x0189edc0 ConVar rodeo_camera_smooth_blend_out_time
r5apex.exe!0x0189d5f0 ConVar rodeo_camera_smooth_enable
r5apex.exe!0x01e7a4d0 ConVar rodeoed_anims_enabled
r5apex.exe!0x01c57d90 ConVar rope_collide
r5apex.exe!0x01c56770 ConVar rope_debug_shake
r5apex.exe!0x01c48b90 ConVar rope_parallelMeshBuilder
r5apex.exe!0x01c57590 ConVar rope_regenMeshEachDraw
r5apex.exe!0x01c4a4a0 ConVar rope_shake
r5apex.exe!0x01c56f70 ConVar rope_texels_per_world_unit
r5apex.exe!0x01c461e0 ConVar rope_wiggle_harmonic_falloff
r5apex.exe!0x01c58810 ConVar rope_wiggle_magnitude_loose
r5apex.exe!0x01c4b3d0 ConVar rope_wiggle_magnitude_tight
r5apex.exe!0x01c53ff0 ConVar rope_wiggle_oscillate_speed
r5apex.exe!0x01c45f00 ConVar rope_wiggle_rotate_speed
r5apex.exe!0x01c50710 ConVar rope_wiggle_zipline_min_points
r5apex.exe!0x01c45940 ConVar rope_wind_dist
r5apex.exe!0x0189cfd0 ConVar rotate_ents
r5apex.exe!0x0119cc70 ConVar rspn_motd
r5apex.exe!0x012eef40 ConVar rt_sync_message_pump
r5apex.exe!0x012eefe0 ConVar rt_worker
r5apex.exe!0x01e7bf30 ConVar ruiPanel_resArgName
r5apex.exe!0x01c60c00 ConVar rui_asyncTracks
r5apex.exe!0x01e9a1e0 ConVar rui_defaultDebugFontFace
r5apex.exe!0x01e9a540 ConVar rui_defaultFontFace
r5apex.exe!0x01e99340 ConVar rui_defaultFontHeight
r5apex.exe!0x01e98a00 ConVar rui_overrideVguiTextRendering
r5apex.exe!0x012a6330 ConVar rui_padDist
r5apex.exe!0x012a6290 ConVar rui_safeAreaFrac
r5apex.exe!0x011a6d80 ConVar rui_standardTextHeight
r5apex.exe!0x012aa010 ConVar save_enable
r5apex.exe!0x01e98d40 ConVar scheme_manager_font_debug
r5apex.exe!0x01c5e560 ConVar scr_centertime
r5apex.exe!0x018a8590 ConVar screen_indicator_back_range
r5apex.exe!0x018b6850 ConVar screen_indicator_ellipse_height
r5apex.exe!0x0189fd90 ConVar screen_indicator_ellipse_width
r5apex.exe!0x018aa4d0 ConVar screen_indicator_pitch_limit
r5apex.exe!0x018aa870 ConVar screen_indicator_pitch_scale
r5apex.exe!0x01c60660 ConVar screenfade_debug
r5apex.exe!0x01e65800 ConVar script_compile_all_levels
r5apex.exe!0x01c5eb80 ConVar script_debugger_connect_client_on_mapspawn
r5apex.exe!0x01e6a7c0 ConVar script_debugger_connect_ui_auto
r5apex.exe!0x01e9b210 ConVar script_debugger_host
r5apex.exe!0x01e9b2b0 ConVar script_debugger_port_client
r5apex.exe!0x01e9b170 ConVar script_debugger_port_server
r5apex.exe!0x01e9b030 ConVar script_debugger_port_ui
r5apex.exe!0x01e9be10 ConVar script_disallow_newslot_on_globals
r5apex.exe!0x01e9b0d0 ConVar script_dump_simple
r5apex.exe!0x01e674d0 ConVar script_error_on_midgame_load
r5apex.exe!0x01e9af90 ConVar script_infinite_loop_ms
r5apex.exe!0x01e67790 ConVar script_parallel_trace_LOS_multiple
r5apex.exe!0x01c9af60 ConVar script_precache_errors
r5apex.exe!0x01c5b370 ConVar script_printDeferredCalls
r5apex.exe!0x01e67eb0 ConVar script_retry_after_compile_errors
r5apex.exe!0x01e67930 ConVar script_seasonNameQueryInterval
r5apex.exe!0x01e7b310 ConVar script_showErrorDialogs
r5apex.exe!0x012d66e0 ConVar script_slopTimeBeforeBudgetEnforcement
r5apex.exe!0x01c616a0 ConVar script_window_client_precache
r5apex.exe!0x01e66150 ConVar seasonquest_force_missionscleared_count
r5apex.exe!0x01e660b0 ConVar seasonquest_force_treasurepacks_count
r5apex.exe!0x0188f0d0 ConVar send_script_errors
r5apex.exe!0x01c9fe70 ConVar sequence_transitioner_enable
r5apex.exe!0x011a5380 ConVar serverFilter
r5apex.exe!0x012d5c10 ConVar serverReports_hostname
r5apex.exe!0x01198190 ConVar server_concommands_allways_network
r5apex.exe!0x012d9730 ConVar server_query_interval
r5apex.exe!0x01898300 ConVar sfm_record_hz
r5apex.exe!0x018adad0 ConVar shadow_always_update
r5apex.exe!0x0188cfe0 ConVar shadow_bleedfudge
r5apex.exe!0x0119dad0 ConVar shadow_capable
r5apex.exe!0x01e64be0 ConVar shadow_clear_dist
r5apex.exe!0x018ab990 ConVar shadow_dbg_draw
r5apex.exe!0x018ab850 ConVar shadow_default_filter_size
r5apex.exe!0x01c3aa20 ConVar shadow_depth_dimen_min
r5apex.exe!0x018ad1b0 ConVar shadow_depth_upres_factor_max
r5apex.exe!0x018b5410 ConVar shadow_drawfrustum
r5apex.exe!0x018a75b0 ConVar shadow_dynamic_blendfactor
r5apex.exe!0x011a3870 ConVar shadow_enable
r5apex.exe!0x018ae770 ConVar shadow_esm_enable
r5apex.exe!0x018a61c0 ConVar shadow_filter_maxstep
r5apex.exe!0x01c44200 ConVar shadow_info
r5apex.exe!0x01c39ef0 ConVar shadow_lobby_mode_allowed
r5apex.exe!0x01c3b100 ConVar shadow_max_downsizing_spot_updates
r5apex.exe!0x018b9470 ConVar shadow_max_dynamic_lobby
r5apex.exe!0x018aa1b0 ConVar shadow_max_old_dynamic
r5apex.exe!0x018b08a0 ConVar shadow_max_spot_updates
r5apex.exe!0x012e6350 ConVar shadow_maxdynamic
r5apex.exe!0x018a6830 ConVar shadow_maxdynamic
r5apex.exe!0x018adc10 ConVar shadow_min_count_smallest
r5apex.exe!0x0188d1c0 ConVar shadow_minvariance
r5apex.exe!0x01c44700 ConVar shadow_multisampled
r5apex.exe!0x018a84f0 ConVar shadow_noLOD
r5apex.exe!0x01c3a2b0 ConVar shadow_show_spot_udpate_infos
r5apex.exe!0x018b57d0 ConVar shadow_tools_depth_dimen_min
r5apex.exe!0x018adf90 ConVar shadow_tools_depth_upres_factor_max
r5apex.exe!0x018b31a0 ConVar shadow_tools_min_count_smallest
r5apex.exe!0x01c3a0f0 ConVar shadow_tools_mode
r5apex.exe!0x018b6520 ConVar shadow_update_culling
r5apex.exe!0x0189eaa0 ConVar shake_angleFactor_human
r5apex.exe!0x0189b160 ConVar shake_angleFactor_titan
r5apex.exe!0x01c5c6d0 ConVar shake_basicPitchFactor
r5apex.exe!0x01c5e4c0 ConVar shake_basicRandomRollFactor
r5apex.exe!0x0189d730 ConVar shake_offsetFactor_human
r5apex.exe!0x0189aa00 ConVar shake_offsetFactor_titan
r5apex.exe!0x01c3aca0 ConVar shake_viewmodelFactor_ads_human
r5apex.exe!0x01c3a210 ConVar shake_viewmodelFactor_ads_titan
r5apex.exe!0x018a83b0 ConVar shake_viewmodelFactor_human
r5apex.exe!0x01c3ad40 ConVar shake_viewmodelFactor_titan
r5apex.exe!0x012ebad0 ConVar showfps_enabled
r5apex.exe!0x012e8a90 ConVar showfps_heightpercent
r5apex.exe!0x012e0f20 ConVar showfps_mouse_latency
r5apex.exe!0x012eb210 ConVar showfps_smoothtime
r5apex.exe!0x012ec250 ConVar showfps_spinner
r5apex.exe!0x012e8cf0 ConVar showmem_enabled
r5apex.exe!0x012ebcb0 ConVar showmem_mode_bottom
r5apex.exe!0x012ec1b0 ConVar showmem_mode_top
r5apex.exe!0x01c5fb60 ConVar showmemnumstats
r5apex.exe!0x01c5ec20 ConVar showmemnumstatsrefresh
r5apex.exe!0x012ebe90 ConVar shownet_enabled
r5apex.exe!0x012eba30 ConVar showsnapshot_enabled
r5apex.exe!0x01c49800 ConVar sidearmSwapSelectCooldown
r5apex.exe!0x01c53c70 ConVar sidearmSwapSelectDoubleTapTime
r5apex.exe!0x0119e820 ConVar single_frame_shutdown_for_reload
r5apex.exe!0x012d7d60 ConVar singlestep
r5apex.exe!0x011a86b0 ConVar skill_arena
r5apex.exe!0x011a8110 ConVar skill_dediOnly
r5apex.exe!0x011a9890 ConVar skill_enabled
r5apex.exe!0x0119e960 ConVar skill_hostname
r5apex.exe!0x01c68c50 ConVar skip_jump_height_fraction
r5apex.exe!0x01c9b3b0 ConVar skip_jump_height_speed
r5apex.exe!0x01c9c750 ConVar skip_replenish_double_jump
r5apex.exe!0x01c66d40 ConVar skip_sounds
r5apex.exe!0x01c9c230 ConVar skip_speed_reduce
r5apex.exe!0x01c99320 ConVar skip_speed_retain
r5apex.exe!0x01c9d330 ConVar skip_time
r5apex.exe!0x011a6e20 ConVar sleep_when_meeting_framerate
r5apex.exe!0x011aa180 ConVar sleep_when_meeting_framerate_headroom_ms
r5apex.exe!0x01c98410 ConVar slide_auto_stand
r5apex.exe!0x01c65ee0 ConVar slide_max_angle_dot
r5apex.exe!0x01c68db0 ConVar slide_step_velocity_reduction
r5apex.exe!0x01cad7e0 ConVar slide_viewTiltDecreaseSpeed
r5apex.exe!0x01cadc80 ConVar slide_viewTiltIncreaseSpeed
r5apex.exe!0x01cac110 ConVar slide_viewTiltPlayerSpeed
r5apex.exe!0x01cad050 ConVar slide_viewTiltSide
r5apex.exe!0x01c92820 ConVar slide_whileInAir
r5apex.exe!0x01199fd0 ConVar slowconsolelog_old_logic
r5apex.exe!0x01cada00 ConVar smoothstairs_lunge
r5apex.exe!0x011993d0 ConVar sort_opaque_meshes
r5apex.exe!0x018ac5d0 ConVar sound_classic_music
r5apex.exe!0x01c59a30 ConVar sound_entity_seek_snap
r5apex.exe!0x01e65cb0 ConVar sound_musicReduced
r5apex.exe!0x01c44aa0 ConVar sound_num_speakers
r5apex.exe!0x01e7d670 ConVar sound_only_warn_on_missing_sound_events_in_client_script
r5apex.exe!0x0126f270 ConVar sound_printloaderrors
r5apex.exe!0x018a76f0 ConVar sound_volume
r5apex.exe!0x018aecd0 ConVar sound_volume_dialogue
r5apex.exe!0x018aebd0 ConVar sound_volume_dialogue_sp
r5apex.exe!0x018b12d0 ConVar sound_volume_music_game
r5apex.exe!0x018af2f0 ConVar sound_volume_music_game_sp
r5apex.exe!0x018ae6d0 ConVar sound_volume_music_lobby
r5apex.exe!0x0189f430 ConVar sound_volume_sfx
r5apex.exe!0x01c3b430 ConVar sound_volume_sfx_sp
r5apex.exe!0x01c3b690 ConVar sound_volume_voice
r5apex.exe!0x018b65c0 ConVar sound_without_focus
r5apex.exe!0x01c53b50 ConVar soundscape_fadetime
r5apex.exe!0x01c51fd0 ConVar soundscape_message
r5apex.exe!0x01c55010 ConVar soundscape_radius_debug
r5apex.exe!0x01897180 ConVar soundtrigger_repeat_interval
r5apex.exe!0x011a6480 ConVar sp_not_focus_pause
r5apex.exe!0x0188cb80 ConVar spam_skinning_matrices_used
r5apex.exe!0x0188cd60 ConVar spam_skinning_matrices_used_detailed
r5apex.exe!0x01c678a0 ConVar spatial_partition_deadlock_assert
r5apex.exe!0x0189e5c0 ConVar spectator_command_interval
r5apex.exe!0x01e7f7d0 ConVar speech_queue_bytes
r5apex.exe!0x012db6d0 ConVar speechtotext_audioenabled
r5apex.exe!0x012db590 ConVar speechtotext_enabled
r5apex.exe!0x012db4f0 ConVar speechtotext_forcedisabled
r5apex.exe!0x012db310 ConVar speechtotext_hostname
r5apex.exe!0x012db450 ConVar speechtotext_msg_droptimeout
r5apex.exe!0x012da350 ConVar speechtotext_path
r5apex.exe!0x012db630 ConVar speechtotext_quiettime
r5apex.exe!0x012db090 ConVar speechtotext_stats_errorspermin
r5apex.exe!0x012db3b0 ConVar speechtotext_stats_interval
r5apex.exe!0x012db1d0 ConVar speechtotext_stats_senderrors
r5apex.exe!0x012db130 ConVar speechtotext_stats_sendrequests
r5apex.exe!0x012db270 ConVar speechtotext_stats_sendsuccess
r5apex.exe!0x011a0150 ConVar speechtotexttoken_hostname
r5apex.exe!0x011abe10 ConVar speex_audio_recording
r5apex.exe!0x011ab070 ConVar speex_audio_value
r5apex.exe!0x012de270 ConVar speex_preprocess_agc_max_gain
r5apex.exe!0x012ddeb0 ConVar speex_preprocess_noise_suppress
r5apex.exe!0x012de1d0 ConVar speex_preprocess_set_agc_decrenment
r5apex.exe!0x012dde10 ConVar speex_preprocess_set_agc_increment
r5apex.exe!0x012de090 ConVar speex_preprocess_set_agc_target
r5apex.exe!0x011ab250 ConVar speex_quiet_threshold
r5apex.exe!0x011ab390 ConVar speex_quiet_window
r5apex.exe!0x012ddf50 ConVar speex_set_enh
r5apex.exe!0x012ddff0 ConVar speex_use_highpass
r5apex.exe!0x012de130 ConVar speex_use_preproser
r5apex.exe!0x01e67f50 ConVar spinner_debug_info
r5apex.exe!0x01cac860 ConVar sprint_powerdrain
r5apex.exe!0x01c3a3d0 ConVar sprint_view_shake_style
r5apex.exe!0x01c634f0 ConVar sprinttilt_accel
r5apex.exe!0x01c99ab0 ConVar sprinttilt_maxvel
r5apex.exe!0x01c667c0 ConVar sprinttilt_turnrange
r5apex.exe!0x01c5d800 ConVar ss_enable
r5apex.exe!0x01c5b690 ConVar ss_force_primary_fullscreen
r5apex.exe!0x01c521b0 ConVar ss_mimic
r5apex.exe!0x01c5db00 ConVar ss_splitmode
r5apex.exe!0x01c602a0 ConVar ss_verticalsplit
r5apex.exe!0x01c44340 ConVar ss_viewmodelfov
r5apex.exe!0x012a81d0 ConVar ss_voice_hearpartner
r5apex.exe!0x012f1980 ConVar ssao_allow_partial
r5apex.exe!0x012f1c00 ConVar ssao_blur
r5apex.exe!0x012f2060 ConVar ssao_blur_edge_sharpness
r5apex.exe!0x012f1e80 ConVar ssao_depth_max
r5apex.exe!0x012f1840 ConVar ssao_downsample
r5apex.exe!0x012f1b60 ConVar ssao_enabled
r5apex.exe!0x012f1f20 ConVar ssao_exponent
r5apex.exe!0x012f17a0 ConVar ssao_jitter_scale
r5apex.exe!0x012f21a0 ConVar ssao_max_res
r5apex.exe!0x012f1a20 ConVar ssao_max_res_threshold
r5apex.exe!0x012f1700 ConVar ssao_num_directions
r5apex.exe!0x012f2600 ConVar ssao_num_steps
r5apex.exe!0x012f2420 ConVar ssao_on_everything
r5apex.exe!0x012f1660 ConVar ssao_radius
r5apex.exe!0x012f1d40 ConVar ssao_radius_in_lobby
r5apex.exe!0x012ee5e0 ConVar ssao_show
r5apex.exe!0x012f2380 ConVar ssao_show
r5apex.exe!0x012f2740 ConVar ssao_show
r5apex.exe!0x012f18e0 ConVar ssao_snap_uv
r5apex.exe!0x012f2100 ConVar ssao_tech
r5apex.exe!0x01e64d20 ConVar ssao_tech
r5apex.exe!0x012f26a0 ConVar ssao_upsample_ranged
r5apex.exe!0x011a84b0 ConVar startButtonCommand
r5apex.exe!0x011a7130 ConVar staticProp_budget
r5apex.exe!0x018a7650 ConVar staticProp_buildlists_on_worker
r5apex.exe!0x011a8750 ConVar staticProp_debug_draw
r5apex.exe!0x011a6ce0 ConVar staticProp_earlyDepthPrepass
r5apex.exe!0x011aab50 ConVar staticProp_earlyDepthPrepassDist
r5apex.exe!0x011a8410 ConVar staticProp_earlyDepthPrepassIncludeOpaques
r5apex.exe!0x011aa540 ConVar staticProp_earlyDepthPrepassIncludeOpaquesDist
r5apex.exe!0x011a9dd0 ConVar staticProp_gather_size_weight
r5apex.exe!0x011a7270 ConVar staticProp_max_scaled_dist
r5apex.exe!0x011aa680 ConVar staticProp_no_fade_scalar
r5apex.exe!0x01e65260 ConVar staticProp_refineDrawOnWorker
r5apex.exe!0x012e30f0 ConVar static_shadow
r5apex.exe!0x0189feb0 ConVar static_shadow
r5apex.exe!0x018aea90 ConVar static_shadow_bounds_per_env
r5apex.exe!0x01e64640 ConVar static_shadow_debug_2d
r5apex.exe!0x018ad370 ConVar static_shadow_debug_dirty_rects
r5apex.exe!0x01c44500 ConVar static_shadow_depth_bias_scale
r5apex.exe!0x018aa430 ConVar static_shadow_expand_z
r5apex.exe!0x018ad2d0 ConVar static_shadow_good_merge_ratio
r5apex.exe!0x018b5ac0 ConVar static_shadow_good_merge_score
r5apex.exe!0x01c3b4d0 ConVar static_shadow_prop_min_size
r5apex.exe!0x012ebd50 ConVar static_shadow_res
r5apex.exe!0x018ab8f0 ConVar static_shadow_shrink_culler
r5apex.exe!0x012ebfd0 ConVar static_shadow_use_d16
r5apex.exe!0x018a78d0 ConVar static_shadow_uses_shadow_lod
r5apex.exe!0x0119f9d0 ConVar staticfile_hostname
r5apex.exe!0x011a1120 ConVar stats_hostname
r5apex.exe!0x01ca4760 ConVar status_effect_warning_level
r5apex.exe!0x012dbd30 ConVar steam_debug
r5apex.exe!0x012db7f0 ConVar steam_id
r5apex.exe!0x012db990 ConVar steam_name
r5apex.exe!0x012dbad0 ConVar steamlink_hostname
r5apex.exe!0x012ef750 ConVar stream_addnoise
r5apex.exe!0x012ef310 ConVar stream_bsp_bucket_bias
r5apex.exe!0x012efcf0 ConVar stream_bsp_dist_scale
r5apex.exe!0x01e954e0 ConVar stream_cache_capacity
r5apex.exe!0x01e95620 ConVar stream_cache_capacity_while_loading
r5apex.exe!0x01e951c0 ConVar stream_cache_high_priority_static_models
r5apex.exe!0x01e94e50 ConVar stream_cache_multithreaded
r5apex.exe!0x01e95040 ConVar stream_cache_preload_from_rpak
r5apex.exe!0x01e95440 ConVar stream_cache_read_buffer_cap
r5apex.exe!0x01e95580 ConVar stream_cache_read_count_cap
r5apex.exe!0x01e953a0 ConVar stream_cache_speculative_add_level
r5apex.exe!0x01e956c0 ConVar stream_cache_speculative_drop
r5apex.exe!0x012ef590 ConVar stream_drop_unused
r5apex.exe!0x012ef3b0 ConVar stream_enable
r5apex.exe!0x012e6490 ConVar stream_freeze_camera
r5apex.exe!0x012ef270 ConVar stream_load_after_drop
r5apex.exe!0x012efeb0 ConVar stream_memory
r5apex.exe!0x012ef9d0 ConVar stream_memory_ignore
r5apex.exe!0x012ef930 ConVar stream_memory_ignore_vram
r5apex.exe!0x012ef890 ConVar stream_memory_min
r5apex.exe!0x012ef7f0 ConVar stream_memory_while_loading
r5apex.exe!0x012efa70 ConVar stream_mode
r5apex.exe!0x012efbb0 ConVar stream_never_high_priority_frac
r5apex.exe!0x012eff50 ConVar stream_overlay
r5apex.exe!0x012efc50 ConVar stream_overlay_mode
r5apex.exe!0x012ef4f0 ConVar stream_pause
r5apex.exe!0x012ef450 ConVar stream_picmip
r5apex.exe!0x012f0d80 ConVar stream_resource_max_commits_per_frame
r5apex.exe!0x012f0e20 ConVar stream_resource_thread
r5apex.exe!0x012f0ce0 ConVar stream_resource_wait_copy_to_commit
r5apex.exe!0x012f0ec0 ConVar stream_resource_wait_creation_to_copy
r5apex.exe!0x012f0c40 ConVar stream_resource_wait_for_additional_gpus
r5apex.exe!0x012efd90 ConVar stream_temp_abort_old_inner_loop
r5apex.exe!0x012ef6b0 ConVar stream_temp_old_abort_all_behavior
r5apex.exe!0x012efb10 ConVar stream_temp_skip_abort_all
r5apex.exe!0x011a5a60 ConVar stringtable_alwaysrebuilddictionaries
r5apex.exe!0x011a1740 ConVar stringtable_compress
r5apex.exe!0x011a2550 ConVar stringtable_showsizes
r5apex.exe!0x0582d7b0 ConVar stryder_forceOriginUsersInvisible
r5apex.exe!0x012d5f30 ConVar stryder_security
r5apex.exe!0x01c68370 ConVar stuck_debugging
r5apex.exe!0x01c98aa0 ConVar stuck_debugging_world_only
r5apex.exe!0x01196120 ConVar studiobonecache_unlimited
r5apex.exe!0x012d5cb0 ConVar subscription_hostname
r5apex.exe!0x01c95ce0 ConVar superjump_disabled_from_water
r5apex.exe!0x01c9d200 ConVar superjump_drain_power_onfail
r5apex.exe!0x01c96fc0 ConVar superjump_fail_sound_when_jump_limit
r5apex.exe!0x01c993f0 ConVar superjump_limit
r5apex.exe!0x01c95840 ConVar superjump_limitreset_onwallrun
r5apex.exe!0x01c81fd0 ConVar superjump_max_power_use
r5apex.exe!0x01c68ae0 ConVar superjump_min_height_fraction
r5apex.exe!0x01c98340 ConVar superjump_min_power_use
r5apex.exe!0x01c94430 ConVar superjump_powerreset_onground
r5apex.exe!0x01c979d0 ConVar sv_airaccelerate
r5apex.exe!0x012d72c0 ConVar sv_allTicksFinal
r5apex.exe!0x012a7150 ConVar sv_allowSendTableTransmitToClients
r5apex.exe!0x012a8e10 ConVar sv_allowSpectatorClients
r5apex.exe!0x012a7ff0 ConVar sv_asyncSendSnapshot
r5apex.exe!0x01c675e0 ConVar sv_backspeed
r5apex.exe!0x012a8310 ConVar sv_balanceTeams
r5apex.exe!0x01c81e50 ConVar sv_bounce
r5apex.exe!0x012aa520 ConVar sv_cheats
r5apex.exe!0x012a7830 ConVar sv_checkPropBudgets
r5apex.exe!0x012a7b30 ConVar sv_compressPlaylists
r5apex.exe!0x01198fd0 ConVar sv_compressTimeValEpsilon
r5apex.exe!0x01199140 ConVar sv_compressTimeVals
r5apex.exe!0x012a73d0 ConVar sv_connectingClientDelay
r5apex.exe!0x012a7e10 ConVar sv_debug_prop_send
r5apex.exe!0x012a9cf0 ConVar sv_debugmanualmode
r5apex.exe!0x012a9890 ConVar sv_disconnectOnScriptError
r5apex.exe!0x012aa0b0 ConVar sv_disconnectOnTooManySnapshotFrames
r5apex.exe!0x0119dfe0 ConVar sv_dumpstringtables
r5apex.exe!0x012a9ed0 ConVar sv_earlyPersistenceRead
r5apex.exe!0x012d78e0 ConVar sv_everyThirdTick
r5apex.exe!0x012a94d0 ConVar sv_extra_client_connect_time
r5apex.exe!0x012a92f0 ConVar sv_fakeClientBaseId
r5apex.exe!0x01c9ed70 ConVar sv_footsteps
r5apex.exe!0x01c67800 ConVar sv_friction
r5apex.exe!0x01c9e5b0 ConVar sv_gravity
r5apex.exe!0x012a6d90 ConVar sv_hibernate_ms
r5apex.exe!0x012a7950 ConVar sv_hibernate_ms_vgui
r5apex.exe!0x012a9f70 ConVar sv_hibernate_postgame_delay
r5apex.exe!0x012a9930 ConVar sv_hibernate_when_empty
r5apex.exe!0x01cadfa0 ConVar sv_infinite_ammo
r5apex.exe!0x012a8ff0 ConVar sv_instancebaselines
r5apex.exe!0x0119bc50 ConVar sv_loadMapModelEarly
r5apex.exe!0x011a3910 ConVar sv_lobbyType
r5apex.exe!0x012a7790 ConVar sv_max_prop_data_dwords_lobby
r5apex.exe!0x012a8090 ConVar sv_max_prop_data_dwords_multiplayer
r5apex.exe!0x012a7510 ConVar sv_max_prop_data_dwords_singleplayer
r5apex.exe!0x012a7a90 ConVar sv_max_props_lobby
r5apex.exe!0x012a7470 ConVar sv_max_props_multiplayer
r5apex.exe!0x012a7290 ConVar sv_max_props_singleplayer
r5apex.exe!0x012a8eb0 ConVar sv_max_snapshots_lobby
r5apex.exe!0x012a66f0 ConVar sv_max_snapshots_multiplayer
r5apex.exe!0x012aa1f0 ConVar sv_max_snapshots_singleplayer
r5apex.exe!0x012a7cf0 ConVar sv_maxclientframes
r5apex.exe!0x012a97f0 ConVar sv_maxrate
r5apex.exe!0x011a0cf0 ConVar sv_maxroutable
r5apex.exe!0x01c9b0d0 ConVar sv_maxspeed
r5apex.exe!0x012a8cd0 ConVar sv_maxupdaterate
r5apex.exe!0x01c9b310 ConVar sv_maxvelocity
r5apex.exe!0x012a7bd0 ConVar sv_minrate
r5apex.exe!0x012a7f50 ConVar sv_minupdaterate
r5apex.exe!0x01c974f0 ConVar sv_optimizedmovement
r5apex.exe!0x012a6f70 ConVar sv_parallel_sendsnapshot
r5apex.exe!0x012a6cf0 ConVar sv_pausable
r5apex.exe!0x012a6510 ConVar sv_playerNameAppendCheater
r5apex.exe!0x01c9cd70 ConVar sv_players
r5apex.exe!0x012a9130 ConVar sv_printHighWaterMark
r5apex.exe!0x01c94570 ConVar sv_pushaway_accel
r5apex.exe!0x01c632d0 ConVar sv_pushaway_clientside
r5apex.exe!0x01ca4df0 ConVar sv_pushaway_clientside_size
r5apex.exe!0x01c9b790 ConVar sv_pushaway_debug
r5apex.exe!0x01c63ba0 ConVar sv_pushaway_dist
r5apex.exe!0x01c97060 ConVar sv_pushaway_min_player_speed
r5apex.exe!0x01c97170 ConVar sv_pushaway_player_accel
r5apex.exe!0x01c67d00 ConVar sv_pushaway_player_dist
r5apex.exe!0x012a8570 ConVar sv_rejectClientConnects
r5apex.exe!0x012a6a90 ConVar sv_rejectConnections
r5apex.exe!0x012a75b0 ConVar sv_requireOriginToken
r5apex.exe!0x012a8610 ConVar sv_resendSignonData
r5apex.exe!0x01c9c190 ConVar sv_rollangle
r5apex.exe!0x01c96110 ConVar sv_rollspeed
r5apex.exe!0x012a69f0 ConVar sv_runSpatialOptimizeInJob
r5apex.exe!0x012a8d70 ConVar sv_scarySnapDeltaPrints
r5apex.exe!0x012a9b10 ConVar sv_sendEarlyServerInfo
r5apex.exe!0x012a79f0 ConVar sv_sendReplayNetMessagesOnNoDeltaSnaps
r5apex.exe!0x012a8750 ConVar sv_separate_freq_change_prop_send
r5apex.exe!0x012a6e30 ConVar sv_showClientTickCmds
r5apex.exe!0x012a6790 ConVar sv_showLargeSnapshotSize
r5apex.exe!0x012a6470 ConVar sv_showSnapshots
r5apex.exe!0x012a6650 ConVar sv_showUserCmds
r5apex.exe!0x012a9750 ConVar sv_single_core_dedi
r5apex.exe!0x012a6950 ConVar sv_skipSendingUnnecessaryPersistence
r5apex.exe!0x01c97860 ConVar sv_skyname
r5apex.exe!0x012a8c30 ConVar sv_snapshot_uniform_interval
r5apex.exe!0x01c96a90 ConVar sv_specaccelerate
r5apex.exe!0x01c69090 ConVar sv_specnoclip
r5apex.exe!0x01c67380 ConVar sv_specspeed
r5apex.exe!0x012a63d0 ConVar sv_stats
r5apex.exe!0x01c67760 ConVar sv_stopspeed
r5apex.exe!0x012a9a70 ConVar sv_stressbots
r5apex.exe!0x012a9d90 ConVar sv_struggleCheck
r5apex.exe!0x012a70b0 ConVar sv_struggleSpam
r5apex.exe!0x012a9610 ConVar sv_struggleSpamInterval
r5apex.exe!0x012a8130 ConVar sv_tempents_send_from_delta
r5apex.exe!0x012a65b0 ConVar sv_tempents_send_from_last_sent
r5apex.exe!0x012dd730 ConVar sv_testLargeDatablock
r5apex.exe!0x01896560 ConVar sv_teststepsimulation
r5apex.exe!0x012a99d0 ConVar sv_transmitToAllPlayersMask_allBitsSet
r5apex.exe!0x012a84d0 ConVar sv_unnecessaryConnectDelay
r5apex.exe!0x012a71f0 ConVar sv_unreliableSnapMaxSize
r5apex.exe!0x012a6c50 ConVar sv_updaterate_mp
r5apex.exe!0x012a8f50 ConVar sv_updaterate_sp
r5apex.exe!0x012a7330 ConVar sv_useReputation
r5apex.exe!0x012a9090 ConVar sv_useThreadsForSnapshots
r5apex.exe!0x012a9bb0 ConVar sv_voiceEcho
r5apex.exe!0x012a9570 ConVar sv_voiceenable
r5apex.exe!0x012a76f0 ConVar sv_warnAboutCmdNumJumps
r5apex.exe!0x011a6040 ConVar sv_watchdogTimer
r5apex.exe!0x01c63190 ConVar sv_wateraccelerate
r5apex.exe!0x01c9d870 ConVar sv_waterdist
r5apex.exe!0x011a4760 ConVar sv_writePersistenceOnShutdown
r5apex.exe!0x01e7bdf0 ConVar sys_attract_mode_timeout
r5apex.exe!0x011a8bb0 ConVar system_alt_f4_closes_window
r5apex.exe!0x01c989d0 ConVar teams_unassigned_are_friendly
r5apex.exe!0x012d7fa0 ConVar telemetry_client_debug
r5apex.exe!0x012d6be0 ConVar telemetry_client_enable
r5apex.exe!0x012d7840 ConVar telemetry_client_sendInterval
r5apex.exe!0x012d6640 ConVar telemetryevent_client_enable
r5apex.exe!0x012a60b0 ConVar tencent_restricted
r5apex.exe!0x012d65a0 ConVar test_fakeTimeDays
r5apex.exe!0x01c9a060 ConVar tether_damageScale
r5apex.exe!0x01c63940 ConVar tether_dodge_damage
r5apex.exe!0x01c63590 ConVar tether_healthDrain
r5apex.exe!0x01c67a80 ConVar tether_healthDrainNPC
r5apex.exe!0x01c9d160 ConVar tether_maxvel
r5apex.exe!0x01c9c920 ConVar tether_radius
r5apex.exe!0x01c9b830 ConVar tether_strength
r5apex.exe!0x01c4da40 ConVar thirdperson_mayamode
r5apex.exe!0x0189ef00 ConVar thirdperson_override
r5apex.exe!0x01c56130 ConVar thirdperson_screenspace
r5apex.exe!0x0119b180 ConVar timeout
r5apex.exe!0x0119b220 ConVar timeout_during_load
r5apex.exe!0x0119a250 ConVar timeout_reconnect
r5apex.exe!0x01cae180 ConVar titan_sprint_sound
r5apex.exe!0x01e8df70 ConVar toggle_on_jump_to_deactivate
r5apex.exe!0x012aa840 ConVar tracehull_height_error_check
r5apex.exe!0x01c51140 ConVar tracer_debug
r5apex.exe!0x01c48280 ConVar tracer_extra
r5apex.exe!0x01e88a00 ConVar trail_optimizedRemove
r5apex.exe!0x01c63630 ConVar traversal_anim
r5apex.exe!0x01c65cc0 ConVar traversal_cooldown
r5apex.exe!0x01c967c0 ConVar traversal_enable
r5apex.exe!0x01c67ee0 ConVar traversal_hand_debug
r5apex.exe!0x01c96d50 ConVar traversal_hand_required_width
r5apex.exe!0x01e82ca0 ConVar traversal_viewLerpInDuration
r5apex.exe!0x01e86010 ConVar traversal_viewLerpOut
r5apex.exe!0x01e86c10 ConVar traversal_viewLerpOutAngle
r5apex.exe!0x01e80720 ConVar traversal_viewLerpOutDebug
r5apex.exe!0x01e87190 ConVar traversal_viewLerpOutPos
r5apex.exe!0x01c66fc0 ConVar traversal_window_duration
r5apex.exe!0x01c65d60 ConVar traversal_window_enable
r5apex.exe!0x01c67da0 ConVar traversal_window_finish_angle
r5apex.exe!0x01c95ac0 ConVar traversal_window_forward_offset
r5apex.exe!0x01c9ca90 ConVar traversal_window_hand_vertical_offset
r5apex.exe!0x01c997a0 ConVar traversal_window_sideways_offset
r5apex.exe!0x0189c7f0 ConVar traversal_window_view_pitch_max
r5apex.exe!0x0189b0c0 ConVar traversal_window_view_pitch_min
r5apex.exe!0x0189e840 ConVar traversal_window_yaw_max
r5apex.exe!0x01c64320 ConVar trigger_crowd_pusher_enabled
r5apex.exe!0x01c9aac0 ConVar trigger_ignore_nonsolids
r5apex.exe!0x012f2920 ConVar tsaa_blendfactorincreaseatmaxvelocity
r5apex.exe!0x012f29c0 ConVar tsaa_blendfactorincreasewhenunoccluded
r5apex.exe!0x012f2b00 ConVar tsaa_blendfactormaxesoutatvelocity
r5apex.exe!0x012f2ce0 ConVar tsaa_blendfactormodulationonsparklesandunocclusion
r5apex.exe!0x012f2ba0 ConVar tsaa_blendfactoroverride
r5apex.exe!0x012f2a60 ConVar tsaa_curframeblendamount
r5apex.exe!0x012f2c40 ConVar tsaa_debugresponsiveflag
r5apex.exe!0x012f27e0 ConVar tsaa_neighborhoodclamping
r5apex.exe!0x012f2880 ConVar tsaa_neighborhoodclampingsoftened
r5apex.exe!0x01c603e0 ConVar tsaa_numsamples
r5apex.exe!0x01c57010 ConVar tweak_light_shadows_every_frame
r5apex.exe!0x0126d340 ConVar twitch_check_interval
r5apex.exe!0x0126e530 ConVar twitch_prime_rewards
r5apex.exe!0x0126d9c0 ConVar twitch_shouldQuery
r5apex.exe!0x01e7bbd0 ConVar ui_fadecloud_time
r5apex.exe!0x01e7bb30 ConVar ui_fadexui_time
r5apex.exe!0x01e7ae10 ConVar ui_gameui_ctrlr_title
r5apex.exe!0x01e7b590 ConVar ui_gameui_modal
r5apex.exe!0x01e7b090 ConVar ui_loadingscreen_autotransition_time
r5apex.exe!0x01e7b770 ConVar ui_loadingscreen_fadein_time
r5apex.exe!0x01199bf0 ConVar ui_loadingscreen_fadeout_time
r5apex.exe!0x01e7b810 ConVar ui_loadingscreen_fadeout_time
r5apex.exe!0x01e7c1b0 ConVar ui_loadingscreen_mintransition_time
r5apex.exe!0x01e7b3b0 ConVar ui_loadingscreen_transition_time
r5apex.exe!0x01e7aff0 ConVar ui_lobby_jointimeout
r5apex.exe!0x01e7c110 ConVar ui_lobby_noautostart
r5apex.exe!0x01e7ba90 ConVar ui_lobby_noresults_create_msg_time
r5apex.exe!0x01caeac0 ConVar ui_posedebug_fade_in_time
r5apex.exe!0x01caea20 ConVar ui_posedebug_fade_out_time
r5apex.exe!0x01e7af50 ConVar ui_virtualnav_render
r5apex.exe!0x01c641e0 ConVar unique_entity_names
r5apex.exe!0x01e7d9b0 ConVar usePromptBaseColor
r5apex.exe!0x01e7f5f0 ConVar usePromptButtonTextColor
r5apex.exe!0x01e7d710 ConVar usePromptImageScale
r5apex.exe!0x01e7f160 ConVar usePromptImageYOffset
r5apex.exe!0x01e80200 ConVar usePromptTextColor
r5apex.exe!0x01e64b40 ConVar use_monitors
r5apex.exe!0x011aae90 ConVar use_valve_auto_gain
r5apex.exe!0x01c99180 ConVar use_vm_cloak_offset
r5apex.exe!0x012d8ea0 ConVar user_tracking_enabled
r5apex.exe!0x0119dea0 ConVar users_hostname
r5apex.exe!0x0189f250 ConVar v_centermove
r5apex.exe!0x018aa250 ConVar v_centerspeed
r5apex.exe!0x01e6ac30 ConVar variable_sights_gravity_scale_override
r5apex.exe!0x01c556e0 ConVar vehicle_predictViaPlayer
r5apex.exe!0x01e98af0 ConVar vgui_EnableFixedAspectScaling
r5apex.exe!0x01e7b8b0 ConVar vgui_drawPolyShapes
r5apex.exe!0x011a6ec0 ConVar vgui_drawfocus
r5apex.exe!0x01e97ca0 ConVar vgui_drawfocus
r5apex.exe!0x011a9090 ConVar vgui_drawkeyfocus
r5apex.exe!0x011a81b0 ConVar vgui_drawtree
r5apex.exe!0x011a9c90 ConVar vgui_drawtree_bounds
r5apex.exe!0x011aaab0 ConVar vgui_drawtree_draw_selected
r5apex.exe!0x011a8d90 ConVar vgui_drawtree_freeze
r5apex.exe!0x011a6180 ConVar vgui_drawtree_hidden
r5apex.exe!0x011a94f0 ConVar vgui_drawtree_panelalpha
r5apex.exe!0x011a7730 ConVar vgui_drawtree_panelptr
r5apex.exe!0x011a7410 ConVar vgui_drawtree_popupsonly
r5apex.exe!0x011a6220 ConVar vgui_drawtree_render_order
r5apex.exe!0x011aa220 ConVar vgui_drawtree_scheme
r5apex.exe!0x011a87f0 ConVar vgui_drawtree_visible
r5apex.exe!0x018984e0 ConVar vgui_interactive
r5apex.exe!0x01e9a5e0 ConVar vgui_noquads
r5apex.exe!0x01e9a4a0 ConVar vgui_notext
r5apex.exe!0x01e97d70 ConVar vgui_resize_on_resolution_change
r5apex.exe!0x01e991e0 ConVar vgui_show_glyph_miss
r5apex.exe!0x011a4260 ConVar vgui_simulate_during_bone_setup
r5apex.exe!0x01e7e6c0 ConVar video_menu_uiscript_reset
r5apex.exe!0x01cacba0 ConVar viewDrift
r5apex.exe!0x01cad740 ConVar viewDrift_ads_delay_debounce_time
r5apex.exe!0x01cad920 ConVar viewDrift_pitch_base1_amp
r5apex.exe!0x01cad880 ConVar viewDrift_pitch_base1_freq
r5apex.exe!0x01cac900 ConVar viewDrift_pitch_base1_phase
r5apex.exe!0x01cad230 ConVar viewDrift_pitch_base2_amp
r5apex.exe!0x01cae0e0 ConVar viewDrift_pitch_base2_freq
r5apex.exe!0x01cacb00 ConVar viewDrift_pitch_base2_phase
r5apex.exe!0x01cac660 ConVar viewDrift_pitch_scaler_amp
r5apex.exe!0x01cae2c0 ConVar viewDrift_pitch_scaler_base
r5apex.exe!0x01cac5a0 ConVar viewDrift_pitch_scaler_freq
r5apex.exe!0x01cac460 ConVar viewDrift_pitch_scaler_phase
r5apex.exe!0x01cad6a0 ConVar viewDrift_pitch_shifter_amp
r5apex.exe!0x01cad600 ConVar viewDrift_pitch_shifter_freq
r5apex.exe!0x01caddc0 ConVar viewDrift_pitch_shifter_phase
r5apex.exe!0x01cae220 ConVar viewDrift_yaw_base1_amp
r5apex.exe!0x01cadb40 ConVar viewDrift_yaw_base1_freq
r5apex.exe!0x01cabf50 ConVar viewDrift_yaw_base1_phase
r5apex.exe!0x01cacc40 ConVar viewDrift_yaw_base2_amp
r5apex.exe!0x01cad4b0 ConVar viewDrift_yaw_base2_freq
r5apex.exe!0x01cad410 ConVar viewDrift_yaw_base2_phase
r5apex.exe!0x01cac9c0 ConVar viewDrift_yaw_scaler_amp
r5apex.exe!0x01cadbe0 ConVar viewDrift_yaw_scaler_base
r5apex.exe!0x01cadf00 ConVar viewDrift_yaw_scaler_freq
r5apex.exe!0x01cadd20 ConVar viewDrift_yaw_scaler_phase
r5apex.exe!0x01cabff0 ConVar viewDrift_yaw_shifter_amp
r5apex.exe!0x01cad370 ConVar viewDrift_yaw_shifter_freq
r5apex.exe!0x01cae360 ConVar viewDrift_yaw_shifter_phase
r5apex.exe!0x01cae400 ConVar view_offset_entity_enable
r5apex.exe!0x01c593f0 ConVar viewangle_debug
r5apex.exe!0x01e93440 ConVar viewangles_simpler
r5apex.exe!0x0189f920 ConVar viewmodelShake
r5apex.exe!0x018b6340 ConVar viewmodelShake_sourceRollRange
r5apex.exe!0x01c9d740 ConVar viewmodel_attachment_fov_fix
r5apex.exe!0x01e64e60 ConVar viewmodel_bounds_draw
r5apex.exe!0x01cb0350 ConVar viewmodel_bounds_draw_lock
r5apex.exe!0x012ec070 ConVar viewmodel_selfshadow
r5apex.exe!0x01e64c80 ConVar viewmodel_selfshadow_debug_2d
r5apex.exe!0x01cb0030 ConVar viewmodel_selfshadow_tightbounds
r5apex.exe!0x018a7790 ConVar viewportscale
r5apex.exe!0x01c93f00 ConVar viewpunch_base_springConstantX
r5apex.exe!0x01c9ba70 ConVar viewpunch_base_springConstantY
r5apex.exe!0x01c9ae40 ConVar viewpunch_base_springConstantZ
r5apex.exe!0x01c82070 ConVar viewpunch_base_springDampingX
r5apex.exe!0x01c9b1d0 ConVar viewpunch_base_springDampingY
r5apex.exe!0x01c68230 ConVar viewpunch_base_springDampingZ
r5apex.exe!0x012d68e0 ConVar violence_ablood
r5apex.exe!0x01ca3ab0 ConVar violence_ablood
r5apex.exe!0x012d6ee0 ConVar violence_agibs
r5apex.exe!0x01ca5b30 ConVar violence_agibs
r5apex.exe!0x012d8180 ConVar violence_hblood
r5apex.exe!0x01ca9280 ConVar violence_hblood
r5apex.exe!0x012d74a0 ConVar violence_hgibs
r5apex.exe!0x01ca6a90 ConVar violence_hgibs
r5apex.exe!0x01e67d10 ConVar visible_ent_cone_debug_duration_client
r5apex.exe!0x011ac1d0 ConVar voice_absTriggerAmount
r5apex.exe!0x01e80600 ConVar voice_allow_mute_self
r5apex.exe!0x011ab2f0 ConVar voice_avggain
r5apex.exe!0x01caacc0 ConVar voice_clientdebug
r5apex.exe!0x011ab6b0 ConVar voice_debugAddSecondTalker
r5apex.exe!0x011abd70 ConVar voice_debugThresholds
r5apex.exe!0x011a6f60 ConVar voice_debugfeedback
r5apex.exe!0x01e7ef30 ConVar voice_decimate_at_bytes
r5apex.exe!0x01e804e0 ConVar voice_decimate_rate
r5apex.exe!0x011abaf0 ConVar voice_enabled
r5apex.exe!0x011ab7f0 ConVar voice_energyPerZeroThreshold
r5apex.exe!0x011abcd0 ConVar voice_energyThreshold
r5apex.exe!0x011abb90 ConVar voice_forcemicrecord
r5apex.exe!0x0119e780 ConVar voice_inputfromfile
r5apex.exe!0x0189f190 ConVar voice_late_update
r5apex.exe!0x011abf50 ConVar voice_loopback
r5apex.exe!0x011ac130 ConVar voice_maxgain
r5apex.exe!0x011ab930 ConVar voice_minEnergyPerZeroThreshold
r5apex.exe!0x011ac270 ConVar voice_mixer_boost
r5apex.exe!0x011ac310 ConVar voice_mixer_mute
r5apex.exe!0x011ac3b0 ConVar voice_mixer_volume
r5apex.exe!0x01cab310 ConVar voice_modenable
r5apex.exe!0x012a9c50 ConVar voice_noxplat
r5apex.exe!0x011ac090 ConVar voice_profile
r5apex.exe!0x011a0330 ConVar voice_recordtofile
r5apex.exe!0x011ab430 ConVar voice_scale
r5apex.exe!0x011aba50 ConVar voice_showchannels
r5apex.exe!0x011ab110 ConVar voice_showincoming
r5apex.exe!0x011ab1b0 ConVar voice_threshold_delay
r5apex.exe!0x011abeb0 ConVar voice_triggerCrossingRate
r5apex.exe!0x011ab610 ConVar voice_triggerRate
r5apex.exe!0x011aadf0 ConVar voice_turn_off_new_filters
r5apex.exe!0x011abc30 ConVar voice_vox
r5apex.exe!0x011aafd0 ConVar voice_writevoices
r5apex.exe!0x0119e300 ConVar voice_xsend_debug
r5apex.exe!0x011ab4d0 ConVar voice_zeroCrossingThreshold
r5apex.exe!0x01e82220 ConVar vortex_damageimpulsescale
r5apex.exe!0x012d82c0 ConVar vprof_server_spike_threshold
r5apex.exe!0x012d7220 ConVar vprof_server_thread
r5apex.exe!0x01e6a4c0 ConVar vscript_ui_do_delay_init
r5apex.exe!0x018b5550 ConVar vsm_culling
r5apex.exe!0x01c3afc0 ConVar vsm_ignore_edge_planes
r5apex.exe!0x0189fff0 ConVar vsm_ignore_face_planes
r5apex.exe!0x011a6a60 ConVar vx_do_not_throttle_events
r5apex.exe!0x01cacce0 ConVar wall_climb_pose_paramteter_hands_enabled
r5apex.exe!0x01c9a850 ConVar wallclimb_vertical_gain_reduction
r5apex.exe!0x01c9da70 ConVar wallrun_angleChangeMinCos
r5apex.exe!0x01c9d090 ConVar wallrun_avoid_wall_top_decel
r5apex.exe!0x01e82e80 ConVar wallrun_curveDebug
r5apex.exe!0x01e82390 ConVar wallrun_curveEnable
r5apex.exe!0x01c98c50 ConVar wallrun_debug
r5apex.exe!0x01c68e50 ConVar wallrun_enable
r5apex.exe!0x01c99670 ConVar wallrun_fallAwaySpeed
r5apex.exe!0x01c99090 ConVar wallrun_hangStopTime
r5apex.exe!0x01c65e40 ConVar wallrun_hangslipduration
r5apex.exe!0x01c968f0 ConVar wallrun_hangslipstarttime
r5apex.exe!0x01c952c0 ConVar wallrun_hangslipvel
r5apex.exe!0x01c93be0 ConVar wallrun_maxViewTilt
r5apex.exe!0x01c81db0 ConVar wallrun_minAngle_air
r5apex.exe!0x01c994c0 ConVar wallrun_noInputSlipFrac
r5apex.exe!0x01c676c0 ConVar wallrun_pushAwayFallOffTime
r5apex.exe!0x01c926e0 ConVar wallrun_repelEnable
r5apex.exe!0x01c95c40 ConVar wallrun_repelSoftness
r5apex.exe!0x01c9c470 ConVar wallrun_repelTimeMax
r5apex.exe!0x01c969f0 ConVar wallrun_repelTimeMin
r5apex.exe!0x01c94610 ConVar wallrun_retry_interval
r5apex.exe!0x01c94ac0 ConVar wallrun_rotateMaxRate
r5apex.exe!0x01c93c80 ConVar wallrun_sameWallDist
r5apex.exe!0x01c944d0 ConVar wallrun_sameWallDot
r5apex.exe!0x01c638a0 ConVar wallrun_sameWallSlope
r5apex.exe!0x01c9c300 ConVar wallrun_slipduration
r5apex.exe!0x01c94980 ConVar wallrun_slipslowdown
r5apex.exe!0x01c987c0 ConVar wallrun_slipstarttime
r5apex.exe!0x01c9ac00 ConVar wallrun_slipvel
r5apex.exe!0x01c67bc0 ConVar wallrun_strengthLossEnd
r5apex.exe!0x01c672e0 ConVar wallrun_strengthLossStart
r5apex.exe!0x01c96df0 ConVar wallrun_upwardAutoPush
r5apex.exe!0x01c9b270 ConVar wallrun_viewTiltPredictTime
r5apex.exe!0x01c682d0 ConVar wallrun_viewTiltSpeed
r5apex.exe!0x01c95d80 ConVar was_loaded
r5apex.exe!0x01e897e0 ConVar weaponFastHolsterScale
r5apex.exe!0x01c9ccd0 ConVar weaponSwitch3p_checkNewWeapon
r5apex.exe!0x01e8d680 ConVar weaponSwitch3p_onHolster
r5apex.exe!0x01cacfb0 ConVar weapon_auto_swap_ordnance_no_ammo
r5apex.exe!0x01e8d1e0 ConVar weapon_debugScript
r5apex.exe!0x01e8cbc0 ConVar weapon_doIdleForSurvivalMelee
r5apex.exe!0x0189e8e0 ConVar weapon_friendly_fire_prevent_ui
r5apex.exe!0x01cadaa0 ConVar weapon_meleeButtonPressProtection
r5apex.exe!0x01e928a0 ConVar weapon_parentingFixLerp
r5apex.exe!0x01cae040 ConVar weapon_pickup_allow_dupes
r5apex.exe!0x0189fba0 ConVar weapon_poseParamMaxDistance
r5apex.exe!0x01e8d580 ConVar weapon_render_with_fastpath
r5apex.exe!0x01c53990 ConVar weapon_setting_autocycle_on_empty
r5apex.exe!0x01e8d2c0 ConVar weapon_sprint_raise_delay
r5apex.exe!0x01e8dcc0 ConVar weaponx_predicting_client_only_optimization
r5apex.exe!0x01e8dd80 ConVar weaponx_smartammo_data_optimization
r5apex.exe!0x01e87370 ConVar window_hint_debug
r5apex.exe!0x01c823d0 ConVar window_hint_fov_down
r5apex.exe!0x01c9a780 ConVar window_hint_fov_horz
r5apex.exe!0x01c94750 ConVar window_hint_fov_up
r5apex.exe!0x01c98930 ConVar window_hint_keyboard_fov_horz
r5apex.exe!0x01c66060 ConVar window_hint_lookahead_time
r5apex.exe!0x01c9c9f0 ConVar window_hint_max_horz_vel_change_dot
r5apex.exe!0x01c958e0 ConVar window_hint_max_vel_change_down
r5apex.exe!0x01c9c0f0 ConVar window_hint_max_vel_change_up
r5apex.exe!0x01c691d0 ConVar window_hint_min_horz_vel
r5apex.exe!0x01c9d4a0 ConVar window_hint_permissive_max_horz_vel_change_dot
r5apex.exe!0x01c98860 ConVar window_hint_permissive_max_vel_change_down
r5apex.exe!0x01c93b40 ConVar window_hint_permissive_max_vel_change_up
r5apex.exe!0x01897660 ConVar z_ragdoll_impact_strength
r5apex.exe!0x01e8a120 ConVar zipline_check_usable_before_deploy
r5apex.exe!0x01e8fca0 ConVar zipline_cooldown_time_0
r5apex.exe!0x01e8fd40 ConVar zipline_cooldown_time_1
r5apex.exe!0x01e8fde0 ConVar zipline_cooldown_time_2
r5apex.exe!0x01e8fe80 ConVar zipline_cooldown_time_3
r5apex.exe!0x01e8ff20 ConVar zipline_cooldown_time_4
r5apex.exe!0x01e8eba0 ConVar zipline_fade_dist
r5apex.exe!0x01e8ec40 ConVar zipline_subdiv_lod_dist_base
r5apex.exe!0x01c57cf0 ConVar zipline_subdiv_slices
r5apex.exe!0x01e904a0 ConVar zipline_subdiv_slices_lod
r5apex.exe!0x01c4ffb0 ConVar zipline_subdiv_stacks
```

## ConCommands

<details>
<summary><code>+ability</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+ability_held</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+attack</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+backward</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+break</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+camdistance</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+camin</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+cammousemove</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+camout</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+campitchdown</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+campitchup</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+camyawleft</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+camyawright</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+commandermousemove</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>+csm_rot_x_neg</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+csm_rot_x_plus</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+csm_rot_y_neg</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+csm_rot_y_plus</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+displayFullscreenMap</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+dodge</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+duck</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+forward</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+graph</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+jump</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+klook</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+left</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+lookdown</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>+lookup</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>+mat_texture_list</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+melee</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+movedown</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+moveleft</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+moveright</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+moveup</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand0</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand3</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand4</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+pause_menu</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+ping</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+posedebug</code></summary>

Turn on pose debugger or add ents to pose debugger UI

flags: `0x4000`  
</details>
<details>
<summary><code>+pushtotalk</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>+reload</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+right</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+score</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand3</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand4</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand5</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand6</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand7</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand8</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand9</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+showscores</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+speed</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+strafe</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+toggle_duck</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+toggle_zoom</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+use</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+useAndReload</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+use_alt</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+use_long</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+variableScopeToggle</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+vgui_drawtree</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+voicerecord</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+walk</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+weaponCycle</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+weapon_discard</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+zoom</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-ability</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-ability_held</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-attack</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-backward</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-break</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-camdistance</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-camin</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-cammousemove</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-camout</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-campitchdown</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-campitchup</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-camyawleft</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-camyawright</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-commandermousemove</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>-csm_rot_x_neg</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-csm_rot_x_plus</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-csm_rot_y_neg</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-csm_rot_y_plus</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-displayFullscreenMap</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-dodge</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-duck</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-forward</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-graph</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-jump</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-klook</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-left</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-lookdown</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>-lookup</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>-mat_texture_list</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-melee</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-movedown</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-moveleft</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-moveright</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-moveup</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand0</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand3</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand4</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-pause_menu</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-ping</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-posedebug</code></summary>

Turn off pose debugger or hide ents from pose debugger UI

flags: `0x4000`  
</details>
<details>
<summary><code>-pushtotalk</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>-reload</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-right</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-score</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand3</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand4</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand5</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand6</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand7</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand8</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand9</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-showscores</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-speed</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-strafe</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-toggle_duck</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-toggle_zoom</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-use</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-useAndReload</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-use_alt</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-use_long</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-variableScopeToggle</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-vgui_drawtree</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-voicerecord</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-walk</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-weaponCycle</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-weapon_discard</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-zoom</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>BindToggle</code></summary>

Performs a bind <key> "increment var <cvar> 0 1 1"

flags: `0x2`  
</details>
<details>
<summary><code>CMaterialSystem_clear_loading</code></summary>



flags: `0x20002`  
</details>
<details>
<summary><code>CMaterialSystem_set_loading</code></summary>



flags: `0x20002`  
</details>
<details>
<summary><code>DebugPrintUsedTextures</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>DumpClientDataBlockReceiver</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>EADP_Dump_Invites</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>EADP_RTM_FORCE_TLS_ERROR</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>EADP_RTM_send_invite</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>EADP_RTM_test_subscribe</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>EADP_dump_friends</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>EADP_get_friend_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>EADP_is_friend_user_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>EADP_search_test2</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>EADP_unfriend_user_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>MemTrackDeltaSnapshot</code></summary>

Debug command compares two snapshots. Takes indices into the snapshot array, negative means from end

flags: `0x2`  
</details>
<details>
<summary><code>MemTrackPrintStats</code></summary>

Debug command prints current mem stats & creates a named snapshot - first param is snapshot name

flags: `0x2`  
</details>
<details>
<summary><code>RTM_SEND_PRESENCE</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>ReloadAimAssistSettings</code></summary>

Reloads aimassist config files.

flags: `0xa`  
</details>
<details>
<summary><code>adminmsg</code></summary>

Send text to the current community (if you are an admin)

flags: `0x2`  
</details>
<details>
<summary><code>aisettings_reparse_client</code></summary>

Reloads the AI settings files

flags: `0xa`  
</details>
<details>
<summary><code>alias</code></summary>

Alias a command.

flags: `0x2`  
</details>
<details>
<summary><code>applyVideoChangesDeferred</code></summary>

Workaround for applying video changes using controller buttons shortcuts.

flags: `0x40000008`  
</details>
<details>
<summary><code>bind</code></summary>

Bind a key to TAPPED.

flags: `0x40000000`  
</details>
<details>
<summary><code>bind_US_standard</code></summary>

Bind a key to TAPPED. Given a key on a standard US keyboard, this function will translate that key to the appropriate key on the user's current keyboard.

flags: `0x40000000`  
</details>
<details>
<summary><code>bind_held</code></summary>

Bind a key to HELD.

flags: `0x40000000`  
</details>
<details>
<summary><code>bind_held_US_standard</code></summary>

Bind a key to HELD.. Given a key on a standard US keyboard, this function will translate that key to the appropriate key on the user's current keyboard.

flags: `0x40000000`  
</details>
<details>
<summary><code>bind_list</code></summary>

List all current bindings.

flags: `0x2`  
</details>
<details>
<summary><code>bind_list_abilities</code></summary>

List all ability bindings and what commands they resolve to

flags: `0x2`  
</details>
<details>
<summary><code>bink_dump_precached_movies</code></summary>

Dumps information about all precached Bink movies

flags: `0x2`  
</details>
<details>
<summary><code>bot_loadout</code></summary>

Override bot loadout

flags: `0x4008`  
</details>
<details>
<summary><code>box</code></summary>

Draw a debug box.

flags: `0x4000`  
</details>
<details>
<summary><code>buildcubemaps</code></summary>

Rebuild cubemaps.

flags: `0x2`  
</details>
<details>
<summary><code>cache_print</code></summary>

cache_print [section]
Print out contents of cache memory.

flags: `0x2`  
</details>
<details>
<summary><code>cache_print_lru</code></summary>

cache_print_lru [section]
Print out contents of cache memory.

flags: `0x2`  
</details>
<details>
<summary><code>cache_print_summary</code></summary>

cache_print_summary [section]
Print out a summary contents of cache memory.

flags: `0x2`  
</details>
<details>
<summary><code>cam_command</code></summary>

Tells camera to change modes

flags: `0x4008`  
</details>
<details>
<summary><code>cancelselect</code></summary>



flags: `0x10000002`  
</details>
<details>
<summary><code>cc_emit</code></summary>

Emits a closed caption

flags: `0xa`  
</details>
<details>
<summary><code>centerview</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>changelevel</code></summary>

Change server to the specified map

flags: `0x20002`  
</details>
<details>
<summary><code>chaosmonkeydisconnect</code></summary>

Server tells us to crash. The monkey server has spoken.

flags: `0x40000008`  
</details>
<details>
<summary><code>chat</code></summary>

Send text to the current chatroom

flags: `0x2`  
</details>
<details>
<summary><code>chat_wheel</code></summary>

Opens the chat wheel

flags: `0x40000008`  
</details>
<details>
<summary><code>chatroom_adminsOnly</code></summary>

Set the chatroom to be admins-only

flags: `0x2`  
</details>
<details>
<summary><code>chatroom_away</code></summary>

Tell the chatserver you are away from the room

flags: `0x2`  
</details>
<details>
<summary><code>chatroom_freetalk</code></summary>

Set the chatroom to be free talk

flags: `0x2`  
</details>
<details>
<summary><code>chatroom_present</code></summary>

Tell the chatserver you are present in the room

flags: `0x2`  
</details>
<details>
<summary><code>chatserver</code></summary>

Connect to a chatserver

flags: `0x40000000`  
</details>
<details>
<summary><code>chroma_base</code></summary>

Transitions to a new base layer for chroma hardware

flags: `0x2`  
</details>
<details>
<summary><code>chroma_layer</code></summary>

Adds an overlay layer for chroma hardware

flags: `0x2`  
</details>
<details>
<summary><code>cl_dump_particle_stats</code></summary>

dump particle profiling info to particle_profile.csv

flags: `0x2`  
</details>
<details>
<summary><code>cl_ent_absbox</code></summary>

Displays the client's absbox for the entity under the crosshair.

flags: `0x4008`  
</details>
<details>
<summary><code>cl_ent_bbox</code></summary>

Displays the client's bounding box for the entity under the crosshair.

flags: `0x4008`  
</details>
<details>
<summary><code>cl_ent_rbox</code></summary>

Displays the client's render box for the entity under the crosshair.

flags: `0x4008`  
</details>
<details>
<summary><code>cl_find_ent</code></summary>

Find and list all client entities with classnames that contain the specified substring.
Format: cl_find_ent <substring>


flags: `0x4000`  
</details>
<details>
<summary><code>cl_find_ent_index</code></summary>

Display data for clientside entity matching specified index.
Format: cl_find_ent_index <index>


flags: `0x4000`  
</details>
<details>
<summary><code>cl_flip_visibility</code></summary>

Flips the visibilityBits of all Entities

flags: `0x4000`  
</details>
<details>
<summary><code>cl_fullupdate</code></summary>

Forces the server to send a full update packet

flags: `0x4000`  
</details>
<details>
<summary><code>cl_interpolation_report</code></summary>

Prints all entities being interpolated on the next frame

flags: `0xa`  
</details>
<details>
<summary><code>cl_panelanimation</code></summary>

Shows panel animation variables: <panelname | blank for all panels>.

flags: `0xa`  
</details>
<details>
<summary><code>cl_particles_dump_effects</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>cl_particles_dumplist</code></summary>

Dump all new particles, optional name substring.

flags: `0xa`  
</details>
<details>
<summary><code>cl_precacheinfo</code></summary>

Show precache info (client).

flags: `0x2`  
</details>
<details>
<summary><code>cl_removedecals</code></summary>

Remove the decals from the entity under the crosshair.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_showents</code></summary>

Dump entity list to console.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_soundscape_flush</code></summary>

Flushes the client side soundscapes

flags: `0x10004008`  
</details>
<details>
<summary><code>cl_trace_start_solid</code></summary>

Trace with given parameters and return start solid result

flags: `0xa`  
</details>
<details>
<summary><code>cl_trace_test_hitbox_with_non_zero_start_offset</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>cl_updatevisibility</code></summary>

Updates visibility bits.

flags: `0xa`  
</details>
<details>
<summary><code>clear_loading_progress_detente</code></summary>

Clears the detente for the load screen.

flags: `0x20002`  
</details>
<details>
<summary><code>clear_loading_progress_sp_text</code></summary>

Clears the sp text for the load screen.

flags: `0x20002`  
</details>
<details>
<summary><code>cm_query_log_record</code></summary>

Start recording a log of all queries

flags: `0x2`  
</details>
<details>
<summary><code>cm_query_log_replay</code></summary>

Play back a query log for performance testing

flags: `0x2`  
</details>
<details>
<summary><code>cmd</code></summary>

Forward command to server.

flags: `0x2`  
</details>
<details>
<summary><code>cmd1</code></summary>

sets userinfo string for split screen player in slot 1

flags: `0x40000000`  
</details>
<details>
<summary><code>cmd2</code></summary>

sets userinfo string for split screen player in slot 2

flags: `0x2`  
</details>
<details>
<summary><code>cmd3</code></summary>

sets userinfo string for split screen player in slot 3

flags: `0x2`  
</details>
<details>
<summary><code>cmd4</code></summary>

sets userinfo string for split screen player in slot 4

flags: `0x2`  
</details>
<details>
<summary><code>collision_debug</code></summary>

Sends a collision ray from player and gathers info.

flags: `0x4008`  
</details>
<details>
<summary><code>colorcorrectionui</code></summary>

Show/hide the color correction tools UI.

flags: `0x4000`  
</details>
<details>
<summary><code>community_browse</code></summary>

Browse available communities

flags: `0x2`  
</details>
<details>
<summary><code>community_getPendingJoinRequest</code></summary>

Get a random pending join request to answer

flags: `0x2`  
</details>
<details>
<summary><code>community_join</code></summary>

Join a community

flags: `0x2`  
</details>
<details>
<summary><code>community_leave</code></summary>

Leave a community

flags: `0x2`  
</details>
<details>
<summary><code>community_list</code></summary>

list my communities

flags: `0x2`  
</details>
<details>
<summary><code>community_report</code></summary>

Report a community

flags: `0x2`  
</details>
<details>
<summary><code>community_showerror</code></summary>

Get a random pending join request to answer

flags: `0x2`  
</details>
<details>
<summary><code>connect</code></summary>

Connect to specified server.

flags: `0x20002`  
</details>
<details>
<summary><code>connectAsSpectator</code></summary>

Connect to specified server as a spectator

flags: `0x20002`  
</details>
<details>
<summary><code>connectWithKey</code></summary>

Connect to specified server with an explicit encryption key.

flags: `0x20002`  
</details>
<details>
<summary><code>connectwithtoken</code></summary>

Connect to specified server with a reservation token.

flags: `0xa0000`  
</details>
<details>
<summary><code>convar_differences</code></summary>

Show all convars which are not at their default values.

flags: `0x2`  
</details>
<details>
<summary><code>convar_findByFlags</code></summary>

Find concommands by flags.

flags: `0x2`  
</details>
<details>
<summary><code>convar_list</code></summary>

Show the list of convars/concommands.

flags: `0x2`  
</details>
<details>
<summary><code>createparty</code></summary>

Create a party

flags: `0x2`  
</details>
<details>
<summary><code>createpartyifnotinone</code></summary>

Create a party if we aren't in one

flags: `0x2`  
</details>
<details>
<summary><code>csm_status</code></summary>

Usage:
   csm_status


flags: `0x2`  
</details>
<details>
<summary><code>damagedefs_reparse_client</code></summary>

Reloads the damage defs

flags: `0xa`  
</details>
<details>
<summary><code>debugModelPurge</code></summary>

Debug command to purge unused models...

flags: `0x2`  
</details>
<details>
<summary><code>devshots_nextmap</code></summary>

Used by the devshots system to go to the next map in the devshots maplist.

flags: `0x2`  
</details>
<details>
<summary><code>devshots_screenshot</code></summary>

Used by the -makedevshots system to take a screenshot. For taking your own screenshots, use the 'screenshot' command instead.

flags: `0x20002`  
</details>
<details>
<summary><code>dfs_print_flag_states</code></summary>

Prints all dfs flag states to console

flags: `0x2`  
</details>
<details>
<summary><code>dfs_print_true_flags</code></summary>

Prints all true feature flags to console

flags: `0x2`  
</details>
<details>
<summary><code>disconnect</code></summary>

Disconnect game from server.

flags: `0x48000000`  
</details>
<details>
<summary><code>display_elapsedtime</code></summary>

Displays how much time has elapsed since the game started

flags: `0x4000`  
</details>
<details>
<summary><code>dlight_debug</code></summary>

Creates a dlight in front of the player

flags: `0x4008`  
</details>
<details>
<summary><code>do_InvitePeople_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>do_Invite_friend_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>do_joinPeople_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>do_origin_test_presence</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>downloadPlaylists</code></summary>

Re-download the playlists


flags: `0x2`  
</details>
<details>
<summary><code>dumpClientStringTable</code></summary>

Dump the contents of the client's game string table to the console.

flags: `0x4000`  
</details>
<details>
<summary><code>dumpstringtables</code></summary>

Print string tables to console.

flags: `0x2`  
</details>
<details>
<summary><code>echo</code></summary>

Echo text to console.

flags: `0x10000002`  
</details>
<details>
<summary><code>echo_error</code></summary>

Echo error text to console.

flags: `0x10000002`  
</details>
<details>
<summary><code>editor_toggle</code></summary>

Disables the simulation and returns focus to the editor

flags: `0x4000`  
</details>
<details>
<summary><code>endmovie</code></summary>

Stop recording movie frames.

flags: `0x20002`  
</details>
<details>
<summary><code>entitlements_print</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>entitlements_send</code></summary>

Send client's entitlements to the server

flags: `0x2`  
</details>
<details>
<summary><code>entitlements_set_bits</code></summary>

Set entitlement bitflags on client. Needs to be set before connecting to server to have effect on server

flags: `0x2`  
</details>
<details>
<summary><code>envmap</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>escape</code></summary>

Escape key pressed.

flags: `0x40000000`  
</details>
<details>
<summary><code>exec</code></summary>

Execute script file.

flags: `0x40000000`  
</details>
<details>
<summary><code>execPlayerConfig</code></summary>

Load player settings.

flags: `0x80000`  
</details>
<details>
<summary><code>execifexists</code></summary>

Execute script file if file exists.

flags: `0x2`  
</details>
<details>
<summary><code>exit</code></summary>

Exit the engine.

flags: `0x2`  
</details>
<details>
<summary><code>eyeInfo</code></summary>

gets info about the current view


flags: `0x2`  
</details>
<details>
<summary><code>firstperson</code></summary>

Switch to firstperson camera.

flags: `0x2`  
</details>
<details>
<summary><code>flush</code></summary>

Flush unlocked cache memory.

flags: `0x4000`  
</details>
<details>
<summary><code>flush_locked</code></summary>

Flush unlocked and locked cache memory.

flags: `0x4000`  
</details>
<details>
<summary><code>force_centerview</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>fps_stats_dump</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>fps_stats_reset</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>fps_stats_start</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>fps_stats_stop</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>friends_update</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>fs_clear_open_duplicate_times</code></summary>

Clear the list of files that have been opened.

flags: `0x2`  
</details>
<details>
<summary><code>fs_dump_open_duplicate_times</code></summary>

Set fs_report_long_reads 1 before loading to use this. Prints a list of files that were opened more than once and ~how long was spent reading from them.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_cancel_prefetches</code></summary>

Cancels all the prefetches in progress.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_flush_cache</code></summary>

Flushes the FIOS HDD cache.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_prefetch_file</code></summary>

Prefetches a file: </PS3_GAME/USRDIR/filename.bin>.
The preftech is medium priority and persistent.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_prefetch_file_in_pack</code></summary>

Prefetches a file in a pack: <portal2/models/container_ride/fineDebris_part5.ani>.
The preftech is medium priority and non-persistent.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_print_prefetches</code></summary>

Displays all the prefetches currently in progress.

flags: `0x2`  
</details>
<details>
<summary><code>fs_printopenfiles</code></summary>

Show all files currently opened by the engine.

flags: `0x2`  
</details>
<details>
<summary><code>fs_warning_level</code></summary>

Set the filesystem warning level.

flags: `0x2`  
</details>
<details>
<summary><code>fx_impact_reparse</code></summary>

Reloads the weapon impact effect table files

flags: `0x4008`  
</details>
<details>
<summary><code>gameui_activate</code></summary>

Shows the game UI

flags: `0x40000000`  
</details>
<details>
<summary><code>gameui_allowescape</code></summary>

Escape key allowed to hide game UI

flags: `0x40000000`  
</details>
<details>
<summary><code>gameui_allowescapetoshow</code></summary>

Escape key allowed to show game UI

flags: `0x2`  
</details>
<details>
<summary><code>gameui_hide</code></summary>

Hides the game UI

flags: `0x40000000`  
</details>
<details>
<summary><code>gameui_preventescape</code></summary>

Escape key doesn't hide game UI

flags: `0x40000000`  
</details>
<details>
<summary><code>gameui_preventescapetoshow</code></summary>

Escape key doesn't show game UI

flags: `0x2`  
</details>
<details>
<summary><code>getNewAuthToken</code></summary>

Ask for a new auth token.

flags: `0x2`  
</details>
<details>
<summary><code>getfov</code></summary>

Gets info about the current FOV

flags: `0x2`  
</details>
<details>
<summary><code>gethttpdatacenterlist</code></summary>

Gets the list of datacenters

flags: `0x2`  
</details>
<details>
<summary><code>getpos</code></summary>

dump position and angles to the console

flags: `0xa`  
</details>
<details>
<summary><code>getpos_bind</code></summary>

Binds the given key to a setpos/setang command of your current position.

flags: `0xa`  
</details>
<details>
<summary><code>getposvec</code></summary>

dump position and angles to the console in 'Vector( x, y, z ), Vector( pitch, yaw, roll )' format

flags: `0xa`  
</details>
<details>
<summary><code>give</code></summary>

Give weapon to player.

flags: `0x4008`  
</details>
<details>
<summary><code>help</code></summary>

Find help about a convar/concommand.

flags: `0x2`  
</details>
<details>
<summary><code>hidepanel</code></summary>

Hides a viewport panel <name>

flags: `0xa`  
</details>
<details>
<summary><code>hidevideos</code></summary>

Hides video panels playing to the screen

flags: `0xa`  
</details>
<details>
<summary><code>highlight_log</code></summary>

Log Highlight

flags: `0x2`  
</details>
<details>
<summary><code>host_runofftime</code></summary>

Run off some time without rendering/updating sounds


flags: `0x2`  
</details>
<details>
<summary><code>hud_subtitles</code></summary>

Plays the Subtitles: <filename>

flags: `0xa`  
</details>
<details>
<summary><code>huffman_readProps</code></summary>

Read the huffman file and regenerate huffman trees

flags: `0x2`  
</details>
<details>
<summary><code>impulse</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>inboxmessage_report</code></summary>

Report an inbox message as abusive

flags: `0x2`  
</details>
<details>
<summary><code>incrementvar</code></summary>

Increment specified convar value.

flags: `0x20002`  
</details>
<details>
<summary><code>ingamemenu_activate</code></summary>

Shows the in-game menu

flags: `0x40080008`  
</details>
<details>
<summary><code>initMatchmaking</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>invnext</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>is_considered_sony_multiplayer</code></summary>

Checks the value for whether the game is currently telling sony it's in multiplayer

flags: `0xa`  
</details>
<details>
<summary><code>joinopeninvite</code></summary>

Join the active open invite in the chat room

flags: `0x2`  
</details>
<details>
<summary><code>joystick_initialize</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>jpeg</code></summary>

Take a jpeg screenshot:  jpeg <filename> <quality 1-100>.

flags: `0x80000`  
</details>
<details>
<summary><code>key_listboundkeys</code></summary>

(DEPRECATED. Prefer bind_list)List bound keys with bindings.

flags: `0x2`  
</details>
<details>
<summary><code>key_updatelayout</code></summary>

Updates game keyboard layout to current windows keyboard setting.

flags: `0x2`  
</details>
<details>
<summary><code>launchplaylist</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>leaveopeninvite</code></summary>

Leave the active open invite in the chat room

flags: `0x2`  
</details>
<details>
<summary><code>listClientFXScriptHandles</code></summary>

Lists all active effects tracked by script.

flags: `0xa`  
</details>
<details>
<summary><code>listmodels</code></summary>

List loaded models.

flags: `0x2`  
</details>
<details>
<summary><code>loadPlaylists</code></summary>

Reload the playlists


flags: `0x2`  
</details>
<details>
<summary><code>map</code></summary>

Start playing on specified map.

flags: `0x20002`  
</details>
<details>
<summary><code>map_background</code></summary>

Runs a map as the background to the main menu.

flags: `0x20002`  
</details>
<details>
<summary><code>maps</code></summary>

Displays list of maps.

flags: `0x2`  
</details>
<details>
<summary><code>mat_antialias_mode</code></summary>

Set antialias mode

flags: `0x2`  
</details>
<details>
<summary><code>mat_configcurrent</code></summary>

show the current video control panel config for the material system

flags: `0x2`  
</details>
<details>
<summary><code>mat_crosshair</code></summary>

Display the name of the material under the crosshair

flags: `0x4000`  
</details>
<details>
<summary><code>mat_crosshair_edit</code></summary>

open the material under the crosshair in the editor defined by mat_crosshair_edit_editor

flags: `0x4000`  
</details>
<details>
<summary><code>mat_crosshair_explorer</code></summary>

open the material under the crosshair in explorer and highlight the file

flags: `0x4000`  
</details>
<details>
<summary><code>mat_crosshair_printmaterial</code></summary>

print the material under the crosshair

flags: `0x4000`  
</details>
<details>
<summary><code>mat_crosshair_reloadmaterial</code></summary>

reload the material under the crosshair

flags: `0x4000`  
</details>
<details>
<summary><code>mat_gamma</code></summary>

Set gamma ramp

flags: `0x40000000`  
</details>
<details>
<summary><code>mat_hdr_enabled</code></summary>

Report if HDR is enabled for debugging

flags: `0x2`  
</details>
<details>
<summary><code>mat_printLiveTex</code></summary>

Print stats of all known live textures.

flags: `0x2`  
</details>
<details>
<summary><code>mat_savechanges</code></summary>

saves current video configuration

flags: `0x40000000`  
</details>
<details>
<summary><code>mat_setvideomode</code></summary>

sets the width, height, windowed state of the material system, as well as borderless state

flags: `0x40000000`  
</details>
<details>
<summary><code>mat_vsync</code></summary>

Set vsync enabled

flags: `0x2`  
</details>
<details>
<summary><code>match_abortAllSearches</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>match_showAllSearches</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>matchmake</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>matchmake_cancel</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>matchmake_cleanupforparty</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>maxplayers</code></summary>

Change the maximum number of players allowed on this server.

flags: `0x2`  
</details>
<details>
<summary><code>mem_compact</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_dump</code></summary>

Dump memory stats to text file.

flags: `0x2`  
</details>
<details>
<summary><code>mem_dump_vm</code></summary>

Dump vm allocations to console.

flags: `0x2`  
</details>
<details>
<summary><code>mem_eat</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_incremental_compact</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_leak_vm</code></summary>

Leak specified amount of virtual memory (in MB or 'oom' to deliberately run out.)

flags: `0x2`  
</details>
<details>
<summary><code>mem_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_textures</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_verify</code></summary>

Verify the validity of the heap

flags: `0x2`  
</details>
<details>
<summary><code>mem_vram</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>memory</code></summary>

Print memory stats.

flags: `0x2`  
</details>
<details>
<summary><code>migrateme</code></summary>

Ask your server to migrate you over to another server

flags: `0x10080000`  
</details>
<details>
<summary><code>miles_dump</code></summary>

Writes out milesdump file and perf CSV for current session (when CSOM_MILESDUMP_PASSIVELY_FOR_DEBUGGING is enabled)

flags: `0xa`  
</details>
<details>
<summary><code>miles_event_info</code></summary>

Shows information about a particular event.

flags: `0x2`  
</details>
<details>
<summary><code>miles_pauseui_byname</code></summary>

Pauses any sound played on the listener for this client with the given name.

flags: `0xa`  
</details>
<details>
<summary><code>miles_play</code></summary>

Plays a given alias at an optional given position

flags: `0x2`  
</details>
<details>
<summary><code>miles_reboot</code></summary>

restarts the audio engine

flags: `0x40000008`  
</details>
<details>
<summary><code>miles_record</code></summary>

Enable or disable continuous recording (including previous buffer if available) of audio output to WAV file.

flags: `0xa`  
</details>
<details>
<summary><code>miles_record_that</code></summary>

Writes audio output from the last minute or so to WAV file. (Only useful when CSOM_MILESDUMP_PASSIVE_SAMPLES is enabled)

flags: `0xa`  
</details>
<details>
<summary><code>miles_stop_all</code></summary>

stops all playing sounds

flags: `0x40000008`  
</details>
<details>
<summary><code>miles_unpauseui_byname</code></summary>

Resumes any paused sound played on the listener for this client with the given name.

flags: `0xa`  
</details>
<details>
<summary><code>miles_write_passive_dumpfile</code></summary>

Writes out milesdump file for current session (Only when CSOM_MILESDUMP_PASSIVELY_FOR_DEBUGGING is enabled)

flags: `0xa`  
</details>
<details>
<summary><code>mmdevinit</code></summary>



flags: `0x20002`  
</details>
<details>
<summary><code>multvar</code></summary>

Multiply specified convar value.

flags: `0x20002`  
</details>
<details>
<summary><code>muteroom</code></summary>

Mute the chatroom

flags: `0x2`  
</details>
<details>
<summary><code>net_channels</code></summary>

Shows net channel info

flags: `0x2`  
</details>
<details>
<summary><code>net_dumpIncomingStats</code></summary>

Dump incoming traffic stats

flags: `0x2`  
</details>
<details>
<summary><code>net_dumpOutgoingStats</code></summary>

Dump outgoing traffic stats

flags: `0x2`  
</details>
<details>
<summary><code>net_dumpStats</code></summary>

Dump all traffic stats

flags: `0x2`  
</details>
<details>
<summary><code>net_start</code></summary>

Inits multiplayer network sockets

flags: `0x2`  
</details>
<details>
<summary><code>net_status</code></summary>

Shows current network status

flags: `0x2`  
</details>
<details>
<summary><code>net_writeStatsFile</code></summary>

Write out networking info to a file


flags: `0x2`  
</details>
<details>
<summary><code>openinvite</code></summary>

Send an open invite to the chat room

flags: `0x2`  
</details>
<details>
<summary><code>openinvitecomplete</code></summary>

Open Invite is complete (we have our search results and reservation is done)

flags: `0x2`  
</details>
<details>
<summary><code>openinvitelaunch</code></summary>

Open Invite should launch

flags: `0x2`  
</details>
<details>
<summary><code>origin_friendlist_dump</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>particle_create</code></summary>

Creates the named particle effect at the location under the crosshair.

flags: `0x4000`  
</details>
<details>
<summary><code>particle_create_on_me</code></summary>

Creates a particle effect on my location

flags: `0x4000`  
</details>
<details>
<summary><code>particle_create_ss</code></summary>

Creates a screen space particle effect

flags: `0x4000`  
</details>
<details>
<summary><code>particle_dump</code></summary>

dumps particles matching provided filter (id or defname substring or *)

flags: `0xa`  
</details>
<details>
<summary><code>particle_kill</code></summary>

Destroys the particle effect created with the particle_create console command.

flags: `0x4000`  
</details>
<details>
<summary><code>particle_list</code></summary>

lists particles all, or matching optional filter (id or defname substring)

flags: `0xa`  
</details>
<details>
<summary><code>particle_recreate</code></summary>

Replays the last particle effect created with the particle_create console command.

flags: `0x4000`  
</details>
<details>
<summary><code>particle_remove_all</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_bake</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_play</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_stop</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>party_leave</code></summary>

quit the current party

flags: `0x40000000`  
</details>
<details>
<summary><code>party_serverChange</code></summary>

update the party with new server info

flags: `0x40000000`  
</details>
<details>
<summary><code>path</code></summary>

Show the engine filesystem path.

flags: `0x2`  
</details>
<details>
<summary><code>pause</code></summary>

Toggle the server pause state.

flags: `0x2`  
</details>
<details>
<summary><code>pausevideos</code></summary>

Pauses all videos playing to the screen

flags: `0xa`  
</details>
<details>
<summary><code>perfcharts_record</code></summary>

Start / Stop perfcharts recording

flags: `0x50020000`  
</details>
<details>
<summary><code>phys_objectDump</code></summary>

Dump a list of the active objects on the client.

flags: `0x2000a`  
</details>
<details>
<summary><code>phys_throw_client</code></summary>

Throws an entity of the given model where the player is looking. Model must already be loaded.

flags: `0x4008`  
</details>
<details>
<summary><code>ping</code></summary>

Display ping to server.

flags: `0x6`  
</details>
<details>
<summary><code>ping_specific_type</code></summary>

Pings a specific ping

flags: `0x40000008`  
</details>
<details>
<summary><code>pingdatacenters</code></summary>

Re-pings the datacenters

flags: `0x2`  
</details>
<details>
<summary><code>pixelvis_debug</code></summary>

Dump debug info

flags: `0xa`  
</details>
<details>
<summary><code>playerSettings_reparse</code></summary>

Reload player class settings from .set files

flags: `0x40004002`  
</details>
<details>
<summary><code>playlistdump</code></summary>

Dump contents of playlist to console (Without changing any state.)

flags: `0x2`  
</details>
<details>
<summary><code>playsoundscape</code></summary>

Forces a soundscape to play

flags: `0x4008`  
</details>
<details>
<summary><code>playvideo</code></summary>

Plays a video: <filename> [width height]

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_end_level_transition</code></summary>

Plays a video fullscreen without ability to skip (unless dev 1) and fades in: <filename> <time>

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_exitcommand</code></summary>

Plays a video and fires and exit command when it is stopped or finishes: <filename> <exit command>

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_exitcommand_nointerrupt</code></summary>

Plays a video (without interruption) and fires and exit command when it is stopped or finishes: <filename> <exit command>

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_nointerrupt</code></summary>

Plays a video without ability to skip: <filename> [width height]

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_scaled</code></summary>

Plays a video at position using coordinates scaled relative to the base screen resolution: <filename> [pinPos posX posY width height]

flags: `0xa`  
</details>
<details>
<summary><code>print_colorcorrection</code></summary>

Display the color correction layer information.

flags: `0x4000`  
</details>
<details>
<summary><code>progress_enable</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>quit</code></summary>

Exit the engine.

flags: `0x40000000`  
</details>
<details>
<summary><code>r_cheapwaterend</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>r_cheapwaterstart</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>r_cleardecals</code></summary>

Usage r_cleardecals <permanent>.

flags: `0x40000000`  
</details>
<details>
<summary><code>r_dxgi_max_frame_latency</code></summary>

Set the max number of command buffers in flight. 0 will set it to the DXGI default of 3. Make sure you are not forcing "Maximum pre-rendered frames" in the driver settings, but leave it application controlled.

flags: `0x2`  
</details>
<details>
<summary><code>r_printdecalinfo</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>readMsgs</code></summary>

Read your messages

flags: `0x2`  
</details>
<details>
<summary><code>recheck</code></summary>

Ask your server to recheck your community

flags: `0x10000002`  
</details>
<details>
<summary><code>recompute_speed</code></summary>

Recomputes clock speed (for debugging purposes).

flags: `0x4000`  
</details>
<details>
<summary><code>reconnect</code></summary>

Silently reconnect to specified server. Similar to silentconnect

flags: `0x400a0000`  
</details>
<details>
<summary><code>reload</code></summary>

Reload the game (add setpos to jump to current view position on reload).

flags: `0x2`  
</details>
<details>
<summary><code>reload_localization</code></summary>

Reloads all the localization data files

flags: `0x2`  
</details>
<details>
<summary><code>reload_script_callbacks</code></summary>

Reloads script callback function pointers for client and server.

flags: `0x4008`  
</details>
<details>
<summary><code>reset_cam_ideal_angles</code></summary>

Resets camera ideal angles to its default

flags: `0x2`  
</details>
<details>
<summary><code>restart</code></summary>

Restart the game on the same level, to the beginning of the level (add setpos to jump to current view position on restart).

flags: `0x2`  
</details>
<details>
<summary><code>restart_checkpoint</code></summary>

Restart the game on the same level, to the last checkpoint (add setpos to jump to current view position on restart).

flags: `0x2`  
</details>
<details>
<summary><code>roamingcam_setang</code></summary>

Rotate roamingcam to the specified angles.( Yaw and pitch only )

flags: `0x40004008`  
</details>
<details>
<summary><code>roamingcam_setpos</code></summary>

Move roamingCam to the specified origin.

flags: `0x40004008`  
</details>
<details>
<summary><code>roamingcam_setroll</code></summary>

Set the roll value for the roamingCam.

flags: `0x4008`  
</details>
<details>
<summary><code>rumble_print</code></summary>

Print current list of active rumbles

flags: `0xa`  
</details>
<details>
<summary><code>savePlayerConfig</code></summary>

Store player settings.

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_down</code></summary>

Select next scoreboard player

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_focus</code></summary>

Focus on scoreboard

flags: `0x2`  
</details>
<details>
<summary><code>scoreboard_mute</code></summary>

Toggle the scoreboard player's muted status

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_profile</code></summary>

Show the scoreboard player's profile

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_toggle_focus</code></summary>

Toggle scoreboard focus

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_up</code></summary>

Select previous scoreboard player

flags: `0x40000000`  
</details>
<details>
<summary><code>screenshot</code></summary>

Take a screenshot.

flags: `0x40000000`  
</details>
<details>
<summary><code>server_single_frame</code></summary>

Single step a frame for server

flags: `0x6`  
</details>
<details>
<summary><code>serverinfo</code></summary>

Request serverinfo from a remote ip and port

flags: `0x2`  
</details>
<details>
<summary><code>set</code></summary>

Change a variable in the class settings (does not save out to disk)


flags: `0x40004002`  
</details>
<details>
<summary><code>set_loading_progress_background</code></summary>

Sets the background for load screen. This is cleared to the default after each load.

flags: `0x20002`  
</details>
<details>
<summary><code>set_loading_progress_detente</code></summary>

Set the keyboard and controller strings for the detentes. This is cleared to the default after each load.

flags: `0x20002`  
</details>
<details>
<summary><code>set_loading_progress_fadeout_enabled</code></summary>

Sets whether or not to fade out of loading. This is cleared to the default after each load. (Default is <fadeoutEnabled> = true )

flags: `0x20002`  
</details>
<details>
<summary><code>set_loading_progress_sp_text</code></summary>

Set the sp text for the load sreen. This is cleared to the default after each load.

flags: `0x20002`  
</details>
<details>
<summary><code>setinfo</code></summary>

Adds a new user info value

flags: `0x40000000`  
</details>
<details>
<summary><code>settype</code></summary>

Sets a type for a Convar/ConCommand. This affects UI rendering for the convar. Examples: 'text', 'bool', 'int 0 10', 'float 0.0 100.0', 'enum apple orange banana'. Move these to code eventually.

flags: `0x2`  
</details>
<details>
<summary><code>shake_stop</code></summary>

Stops all active screen shakes.


flags: `0x4000`  
</details>
<details>
<summary><code>shake_testpunch</code></summary>

Test a punch-style screen shake.


flags: `0x4000`  
</details>
<details>
<summary><code>show_loading_progress</code></summary>

Prints all debug information regarding the state of the loading progress.

flags: `0x2`  
</details>
<details>
<summary><code>showpanel</code></summary>

Shows a viewport panel <name>

flags: `0xa`  
</details>
<details>
<summary><code>showvideos</code></summary>

Makes video panels playing to the screen visible (if they were hidden)

flags: `0xa`  
</details>
<details>
<summary><code>silentconnect</code></summary>

Silently connect to specified server, without disconnecting from our current server unless it succeeds.

flags: `0xa0000`  
</details>
<details>
<summary><code>skill_writeTrainingData</code></summary>

Write training gauntlet skill data

flags: `0x6`  
</details>
<details>
<summary><code>soundscape_dumpclient</code></summary>

Dumps the client's soundscape data.


flags: `0x4000`  
</details>
<details>
<summary><code>spawn_as_pilot</code></summary>

Spawn as Pilot

flags: `0x2`  
</details>
<details>
<summary><code>spawn_as_titan</code></summary>

Spawn as Titan

flags: `0x2`  
</details>
<details>
<summary><code>ss_map</code></summary>

Start playing on specified map with max allowed splitscreen players.

flags: `0x20002`  
</details>
<details>
<summary><code>ss_reloadletterbox</code></summary>

ss_reloadletterbox

flags: `0xa`  
</details>
<details>
<summary><code>sssss_enable</code></summary>

Enable screen-space subsurface scattering. 0 - off, 1 - enabled in lobby, 2 - always enabled

flags: `0xa`  
</details>
<details>
<summary><code>star_memory</code></summary>

Dump memory stats

flags: `0x2`  
</details>
<details>
<summary><code>startmovie</code></summary>

Start recording movie frames.

flags: `0x20002`  
</details>
<details>
<summary><code>status</code></summary>

Display map and connection status.

flags: `0x2`  
</details>
<details>
<summary><code>steam_printid</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>steam_testOverlay</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>steamlink</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>steamunlink</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>stop_transition_videos_fadeout</code></summary>

Fades out all transition videos playing to the screen: <time>

flags: `0x40000008`  
</details>
<details>
<summary><code>stopsoundscape</code></summary>

Stops all soundscape processing and fades current looping sounds

flags: `0x4008`  
</details>
<details>
<summary><code>stopvideos</code></summary>

Stops all videos playing to the screen

flags: `0xa`  
</details>
<details>
<summary><code>stopvideos_fadeout</code></summary>

Fades out all videos playing to the screen: <time>

flags: `0xa`  
</details>
<details>
<summary><code>sv_precacheinfo</code></summary>

Show precache info.

flags: `0x2`  
</details>
<details>
<summary><code>sv_showents</code></summary>

Prints the server entity list

flags: `0x2`  
</details>
<details>
<summary><code>sv_shutdown</code></summary>

Sets the server to shutdown when all games have completed

flags: `0x4004`  
</details>
<details>
<summary><code>sv_writeSendTableStreamFile</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>testCockpitJoltAngles</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>testCockpitJoltOrigin</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>test_freezeframe</code></summary>

Test the freeze frame code.

flags: `0x4000`  
</details>
<details>
<summary><code>testhudanim</code></summary>

Test a hud element animation.
	Arguments: <anim name>


flags: `0x4008`  
</details>
<details>
<summary><code>thread_test_tslist</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>thread_test_tsqueue</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>titan_loadout_select</code></summary>

Titan loadout select

flags: `0x2`  
</details>
<details>
<summary><code>toggle</code></summary>

Toggles a convar on or off, or cycles through a set of values.

flags: `0x80000`  
</details>
<details>
<summary><code>toggle_inventory</code></summary>

Toggle the inventory menu

flags: `0x40080008`  
</details>
<details>
<summary><code>toggle_map</code></summary>

Toggle the big map

flags: `0x40080008`  
</details>
<details>
<summary><code>ui_reloadscheme</code></summary>

Reloads the resource files for the active UI window

flags: `0xa`  
</details>
<details>
<summary><code>uiscript_reset</code></summary>

Resets all UI script state

flags: `0x40000000`  
</details>
<details>
<summary><code>uiscript_resolutionchanged</code></summary>

Notifies UI script that the resolution has changed

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind</code></summary>

Unbind a key's TAPPED binding.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_US_standard</code></summary>

Unbind a key's TAPPED binding. Given a key on a standard US keyboard, this function will translate that key to the appropriate key on the user's current keyboard and unbind that key.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_all_gamepad</code></summary>

Unbinds all gamepad binds

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_batch</code></summary>

Unbind all bindings (tapped/held) from all specified keys (no keyboard layout translation).

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_held</code></summary>

Unbind a key's HELD binding.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_held_US_standard</code></summary>

Unbind a key's HELD binding. Given a key on a standard US keyboard, this function will translate that key to the appropriate key on the user's current keyboard and unbind that key.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbindall</code></summary>

Unbind all keys.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbindall_ignoreGamepad</code></summary>

Unbind all keys, skip gamepad binds.

flags: `0x40000000`  
</details>
<details>
<summary><code>unload_level_loadscreen</code></summary>

Unloads the loadscreen for the current level

flags: `0x40000000`  
</details>
<details>
<summary><code>unmuteroom</code></summary>

Unmute the chatroom

flags: `0x2`  
</details>
<details>
<summary><code>unpausevideos</code></summary>

Unpauses all videos playing to the screen

flags: `0xa`  
</details>
<details>
<summary><code>use_consumable</code></summary>

Uses a specific consumable

flags: `0x80008`  
</details>
<details>
<summary><code>user</code></summary>

Show user data.

flags: `0x2`  
</details>
<details>
<summary><code>users</code></summary>

Show user info for players on server.

flags: `0x2`  
</details>
<details>
<summary><code>version</code></summary>

Print version info string.

flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_clear</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>vgui_spew_fonts</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>vgui_togglepanel</code></summary>

show/hide vgui panel by name.

flags: `0x2`  
</details>
<details>
<summary><code>voicerecord_toggle</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>vx_datacache_list</code></summary>

vx_datacache_list

flags: `0x2`  
</details>
<details>
<summary><code>vx_model_list</code></summary>

Dump models to VXConsole

flags: `0x2`  
</details>
<details>
<summary><code>weaponSelectOrdnance</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weaponSelectPrimary0</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weaponSelectPrimary1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weaponSelectPrimary2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weapon_activity</code></summary>

Play a custom animation activity on the active weapon.

flags: `0x40000000`  
</details>
<details>
<summary><code>weapon_inspect</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weapon_list</code></summary>

Lists all weapons owned by the local player

flags: `0x2`  
</details>
<details>
<summary><code>weapon_reparse</code></summary>

Reloads the weapon script files

flags: `0x4008`  
</details>
<details>
<summary><code>xlog_list</code></summary>

List all xlogs, and various stats

flags: `0x2`  
</details>
<details>
<summary><code>xlog_record</code></summary>

Start writing log file to disk (including any previously buffered data)

flags: `0x2`  
</details>
<details>
<summary><code>xlog_record_that</code></summary>

Write buffered data (if any)

flags: `0x2`  
</details>
<details>
<summary><code>xlog_stop</code></summary>

Stop writing it to disk. (resume buffering if buffering enabled on log)

flags: `0x2`  
</details>
<details>
<summary><code>xlook</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>xmove</code></summary>



flags: `0x40080000`  
</details>

### Addresses

```
r5apex.exe!0x01c45d60 ConCommand +ability
r5apex.exe!0x01c4d7e0 ConCommand +ability_held
r5apex.exe!0x01c51f50 ConCommand +attack
r5apex.exe!0x01c55dd0 ConCommand +backward
r5apex.exe!0x01c4eb80 ConCommand +break
r5apex.exe!0x01c52110 ConCommand +camdistance
r5apex.exe!0x01c4d860 ConCommand +camin
r5apex.exe!0x01c4a1e0 ConCommand +cammousemove
r5apex.exe!0x01c4a760 ConCommand +camout
r5apex.exe!0x01c55d50 ConCommand +campitchdown
r5apex.exe!0x01c45a80 ConCommand +campitchup
r5apex.exe!0x01c4ec00 ConCommand +camyawleft
r5apex.exe!0x01c4c450 ConCommand +camyawright
r5apex.exe!0x01c510c0 ConCommand +commandermousemove
r5apex.exe!0x0189f740 ConCommand +csm_rot_x_neg
r5apex.exe!0x018af9f0 ConCommand +csm_rot_x_plus
r5apex.exe!0x018ac4b0 ConCommand +csm_rot_y_neg
r5apex.exe!0x018af250 ConCommand +csm_rot_y_plus
r5apex.exe!0x01c59040 ConCommand +displayFullscreenMap
r5apex.exe!0x01c4bd30 ConCommand +dodge
r5apex.exe!0x01c568b0 ConCommand +duck
r5apex.exe!0x01c45fa0 ConCommand +forward
r5apex.exe!0x01c533b0 ConCommand +graph
r5apex.exe!0x01c57630 ConCommand +jump
r5apex.exe!0x01c51260 ConCommand +klook
r5apex.exe!0x01c55660 ConCommand +left
r5apex.exe!0x01c53530 ConCommand +lookdown
r5apex.exe!0x01c54110 ConCommand +lookup
r5apex.exe!0x012a4500 ConCommand +mat_texture_list
r5apex.exe!0x01c56e70 ConCommand +melee
r5apex.exe!0x01c53330 ConCommand +movedown
r5apex.exe!0x01c4d5a0 ConCommand +moveleft
r5apex.exe!0x01c465c0 ConCommand +moveright
r5apex.exe!0x01c4bf50 ConCommand +moveup
r5apex.exe!0x01c57a10 ConCommand +offhand0
r5apex.exe!0x01c48920 ConCommand +offhand1
r5apex.exe!0x01c563f0 ConCommand +offhand2
r5apex.exe!0x01c51e30 ConCommand +offhand3
r5apex.exe!0x01c55e50 ConCommand +offhand4
r5apex.exe!0x01c590c0 ConCommand +pause_menu
r5apex.exe!0x01c4ff30 ConCommand +ping
r5apex.exe!0x01caf3f0 ConCommand +posedebug
r5apex.exe!0x011abff0 ConCommand +pushtotalk
r5apex.exe!0x01c4a5e0 ConCommand +reload
r5apex.exe!0x01c49a40 ConCommand +right
r5apex.exe!0x01c58230 ConCommand +score
r5apex.exe!0x01c548f0 ConCommand +scriptCommand1
r5apex.exe!0x01c56ef0 ConCommand +scriptCommand2
r5apex.exe!0x01c49920 ConCommand +scriptCommand3
r5apex.exe!0x01c4c230 ConCommand +scriptCommand4
r5apex.exe!0x01c53e50 ConCommand +scriptCommand5
r5apex.exe!0x01c54190 ConCommand +scriptCommand6
r5apex.exe!0x01c57910 ConCommand +scriptCommand7
r5apex.exe!0x01c49fa0 ConCommand +scriptCommand8
r5apex.exe!0x01c53bf0 ConCommand +scriptCommand9
r5apex.exe!0x01c46860 ConCommand +showscores
r5apex.exe!0x01c45640 ConCommand +speed
r5apex.exe!0x01c58fc0 ConCommand +strafe
r5apex.exe!0x01c4a420 ConCommand +toggle_duck
r5apex.exe!0x01c466c0 ConCommand +toggle_zoom
r5apex.exe!0x01c55b90 ConCommand +use
r5apex.exe!0x01c53f70 ConCommand +useAndReload
r5apex.exe!0x01c4c820 ConCommand +use_alt
r5apex.exe!0x01c56930 ConCommand +use_long
r5apex.exe!0x01c4a9a0 ConCommand +variableScopeToggle
r5apex.exe!0x011a9fb0 ConCommand +vgui_drawtree
r5apex.exe!0x0119ef80 ConCommand +voicerecord
r5apex.exe!0x01c58930 ConCommand +walk
r5apex.exe!0x01c57f70 ConCommand +weaponCycle
r5apex.exe!0x01c542b0 ConCommand +weapon_discard
r5apex.exe!0x01c4a660 ConCommand +zoom
r5apex.exe!0x01c511e0 ConCommand -ability
r5apex.exe!0x01c4b630 ConCommand -ability_held
r5apex.exe!0x01c4dc00 ConCommand -attack
r5apex.exe!0x01c458c0 ConCommand -backward
r5apex.exe!0x01c50050 ConCommand -break
r5apex.exe!0x01c58790 ConCommand -camdistance
r5apex.exe!0x01c54ef0 ConCommand -camin
r5apex.exe!0x01c4a020 ConCommand -cammousemove
r5apex.exe!0x01c4f7a0 ConCommand -camout
r5apex.exe!0x01c53870 ConCommand -campitchdown
r5apex.exe!0x01c4ed20 ConCommand -campitchup
r5apex.exe!0x01c4d6c0 ConCommand -camyawleft
r5apex.exe!0x01c4bed0 ConCommand -camyawright
r5apex.exe!0x01c56a50 ConCommand -commandermousemove
r5apex.exe!0x018b0fa0 ConCommand -csm_rot_x_neg
r5apex.exe!0x0189fe30 ConCommand -csm_rot_x_plus
r5apex.exe!0x01c3ab80 ConCommand -csm_rot_y_neg
r5apex.exe!0x018ae810 ConCommand -csm_rot_y_plus
r5apex.exe!0x01c45e80 ConCommand -displayFullscreenMap
r5apex.exe!0x01c57c50 ConCommand -dodge
r5apex.exe!0x01c534b0 ConCommand -duck
r5apex.exe!0x01c4c330 ConCommand -forward
r5apex.exe!0x01c4c570 ConCommand -graph
r5apex.exe!0x01c4cfc0 ConCommand -jump
r5apex.exe!0x01c4b6b0 ConCommand -klook
r5apex.exe!0x01c46640 ConCommand -left
r5apex.exe!0x01c4c110 ConCommand -lookdown
r5apex.exe!0x01c45c40 ConCommand -lookup
r5apex.exe!0x012a42a0 ConCommand -mat_texture_list
r5apex.exe!0x01c50ac0 ConCommand -melee
r5apex.exe!0x01c4a300 ConCommand -movedown
r5apex.exe!0x01c55820 ConCommand -moveleft
r5apex.exe!0x01c4d040 ConCommand -moveright
r5apex.exe!0x01c535b0 ConCommand -moveup
r5apex.exe!0x01c54090 ConCommand -offhand0
r5apex.exe!0x01c56010 ConCommand -offhand1
r5apex.exe!0x01c550b0 ConCommand -offhand2
r5apex.exe!0x01c4b730 ConCommand -offhand3
r5apex.exe!0x01c46300 ConCommand -offhand4
r5apex.exe!0x01c55920 ConCommand -pause_menu
r5apex.exe!0x01c46740 ConCommand -ping
r5apex.exe!0x01caf490 ConCommand -posedebug
r5apex.exe!0x011ab9d0 ConCommand -pushtotalk
r5apex.exe!0x01c46420 ConCommand -reload
r5apex.exe!0x01c53430 ConCommand -right
r5apex.exe!0x01c561d0 ConCommand -score
r5apex.exe!0x01c46540 ConCommand -scriptCommand1
r5apex.exe!0x01c53a30 ConCommand -scriptCommand2
r5apex.exe!0x01c57990 ConCommand -scriptCommand3
r5apex.exe!0x01c57750 ConCommand -scriptCommand4
r5apex.exe!0x01c456c0 ConCommand -scriptCommand5
r5apex.exe!0x01c454a0 ConCommand -scriptCommand6
r5apex.exe!0x01c57bd0 ConCommand -scriptCommand7
r5apex.exe!0x01c49680 ConCommand -scriptCommand8
r5apex.exe!0x01c530f0 ConCommand -scriptCommand9
r5apex.exe!0x01c498a0 ConCommand -showscores
r5apex.exe!0x01c4cf40 ConCommand -speed
r5apex.exe!0x01c56370 ConCommand -strafe
r5apex.exe!0x01c4c2b0 ConCommand -toggle_duck
r5apex.exe!0x01c53db0 ConCommand -toggle_zoom
r5apex.exe!0x01c49700 ConCommand -use
r5apex.exe!0x01c455c0 ConCommand -useAndReload
r5apex.exe!0x01c55b10 ConCommand -use_alt
r5apex.exe!0x01c4e910 ConCommand -use_long
r5apex.exe!0x01c46280 ConCommand -variableScopeToggle
r5apex.exe!0x011a9930 ConCommand -vgui_drawtree
r5apex.exe!0x0119b9b0 ConCommand -voicerecord
r5apex.exe!0x01c57ff0 ConCommand -walk
r5apex.exe!0x01c4bdb0 ConCommand -weaponCycle
r5apex.exe!0x01c58a50 ConCommand -weapon_discard
r5apex.exe!0x01c4fa00 ConCommand -zoom
r5apex.exe!0x01197e60 ConCommand BindToggle
r5apex.exe!0x012efe30 ConCommand CMaterialSystem_clear_loading
r5apex.exe!0x012ef630 ConCommand CMaterialSystem_set_loading
r5apex.exe!0x012efff0 ConCommand DebugPrintUsedTextures
r5apex.exe!0x012a4fd0 ConCommand DumpClientDataBlockReceiver
r5apex.exe!0x012dd3d0 ConCommand EADP_Dump_Invites
r5apex.exe!0x012dd050 ConCommand EADP_RTM_FORCE_TLS_ERROR
r5apex.exe!0x012dd450 ConCommand EADP_RTM_send_invite
r5apex.exe!0x012dd630 ConCommand EADP_RTM_test_subscribe
r5apex.exe!0x012d60d0 ConCommand EADP_dump_friends
r5apex.exe!0x012d5fd0 ConCommand EADP_get_friend_test
r5apex.exe!0x012d6150 ConCommand EADP_is_friend_user_test
r5apex.exe!0x012d6270 ConCommand EADP_search_test2
r5apex.exe!0x012d6050 ConCommand EADP_unfriend_user_test
r5apex.exe!0x0126e110 ConCommand MemTrackDeltaSnapshot
r5apex.exe!0x0126dcb0 ConCommand MemTrackPrintStats
r5apex.exe!0x012dd5b0 ConCommand RTM_SEND_PRESENCE
r5apex.exe!0x018b6730 ConCommand ReloadAimAssistSettings
r5apex.exe!0x0126d780 ConCommand adminmsg
r5apex.exe!0x01e806a0 ConCommand aisettings_reparse_client
r5apex.exe!0x0119b780 ConCommand alias
r5apex.exe!0x01e7f2a0 ConCommand applyVideoChangesDeferred
r5apex.exe!0x011a1d90 ConCommand bind
r5apex.exe!0x0119f380 ConCommand bind_US_standard
r5apex.exe!0x011a3610 ConCommand bind_held
r5apex.exe!0x011a2df0 ConCommand bind_held_US_standard
r5apex.exe!0x0119f750 ConCommand bind_list
r5apex.exe!0x0119e660 ConCommand bind_list_abilities
r5apex.exe!0x011ac950 ConCommand bink_dump_precached_movies
r5apex.exe!0x01e6ab70 ConCommand bot_loadout
r5apex.exe!0x0126dd30 ConCommand box
r5apex.exe!0x01198db0 ConCommand buildcubemaps
r5apex.exe!0x011a0bb0 ConCommand cache_print
r5apex.exe!0x011a03d0 ConCommand cache_print_lru
r5apex.exe!0x0119be00 ConCommand cache_print_summary
r5apex.exe!0x01c51a20 ConCommand cam_command
r5apex.exe!0x01c49780 ConCommand cancelselect
r5apex.exe!0x01c48c30 ConCommand cc_emit
r5apex.exe!0x018a0350 ConCommand centerview
r5apex.exe!0x011a4300 ConCommand changelevel
r5apex.exe!0x0189e700 ConCommand chaosmonkeydisconnect
r5apex.exe!0x0126f170 ConCommand chat
r5apex.exe!0x01c611e0 ConCommand chat_wheel
r5apex.exe!0x012dcd90 ConCommand chatroom_adminsOnly
r5apex.exe!0x0126d640 ConCommand chatroom_away
r5apex.exe!0x012dcbf0 ConCommand chatroom_freetalk
r5apex.exe!0x0126e8a0 ConCommand chatroom_present
r5apex.exe!0x0126dda0 ConCommand chatserver
r5apex.exe!0x011acb10 ConCommand chroma_base
r5apex.exe!0x011aca90 ConCommand chroma_layer
r5apex.exe!0x01c62800 ConCommand cl_dump_particle_stats
r5apex.exe!0x01e926a0 ConCommand cl_ent_absbox
r5apex.exe!0x01e908a0 ConCommand cl_ent_bbox
r5apex.exe!0x01e91240 ConCommand cl_ent_rbox
r5apex.exe!0x01e91340 ConCommand cl_find_ent
r5apex.exe!0x01e91b20 ConCommand cl_find_ent_index
r5apex.exe!0x01e912c0 ConCommand cl_flip_visibility
r5apex.exe!0x0126e220 ConCommand cl_fullupdate
r5apex.exe!0x01e91d60 ConCommand cl_interpolation_report
r5apex.exe!0x01c5c650 ConCommand cl_panelanimation
r5apex.exe!0x01c5d780 ConCommand cl_particles_dump_effects
r5apex.exe!0x01c5dc40 ConCommand cl_particles_dumplist
r5apex.exe!0x0126d5c0 ConCommand cl_precacheinfo
r5apex.exe!0x01e91c40 ConCommand cl_removedecals
r5apex.exe!0x0126df60 ConCommand cl_showents
r5apex.exe!0x01c56cb0 ConCommand cl_soundscape_flush
r5apex.exe!0x01c63770 ConCommand cl_trace_start_solid
r5apex.exe!0x0189e780 ConCommand cl_trace_test_hitbox_with_non_zero_start_offset
r5apex.exe!0x01e92820 ConCommand cl_updatevisibility
r5apex.exe!0x011a4910 ConCommand clear_loading_progress_detente
r5apex.exe!0x0119ca50 ConCommand clear_loading_progress_sp_text
r5apex.exe!0x01198b60 ConCommand cm_query_log_record
r5apex.exe!0x0119b030 ConCommand cm_query_log_replay
r5apex.exe!0x01198c50 ConCommand cmd
r5apex.exe!0x011986f0 ConCommand cmd1
r5apex.exe!0x01198d40 ConCommand cmd2
r5apex.exe!0x01198230 ConCommand cmd3
r5apex.exe!0x01198e50 ConCommand cmd4
r5apex.exe!0x01e90f60 ConCommand collision_debug
r5apex.exe!0x0119b890 ConCommand colorcorrectionui
r5apex.exe!0x012d5610 ConCommand community_browse
r5apex.exe!0x012d57d0 ConCommand community_getPendingJoinRequest
r5apex.exe!0x012d5470 ConCommand community_join
r5apex.exe!0x012d5b90 ConCommand community_leave
r5apex.exe!0x012d54f0 ConCommand community_list
r5apex.exe!0x012d5b10 ConCommand community_report
r5apex.exe!0x012d51f0 ConCommand community_showerror
r5apex.exe!0x0126e780 ConCommand connect
r5apex.exe!0x0126ec60 ConCommand connectAsSpectator
r5apex.exe!0x0126d480 ConCommand connectWithKey
r5apex.exe!0x0126f390 ConCommand connectwithtoken
r5apex.exe!0x01199580 ConCommand convar_differences
r5apex.exe!0x01199510 ConCommand convar_findByFlags
r5apex.exe!0x0119b0a0 ConCommand convar_list
r5apex.exe!0x0126def0 ConCommand createparty
r5apex.exe!0x0126ea40 ConCommand createpartyifnotinone
r5apex.exe!0x01c3b1f0 ConCommand csm_status
r5apex.exe!0x01ca8700 ConCommand damagedefs_reparse_client
r5apex.exe!0x0126de80 ConCommand debugModelPurge
r5apex.exe!0x011996d0 ConCommand devshots_nextmap
r5apex.exe!0x0126e3b0 ConCommand devshots_screenshot
r5apex.exe!0x012d6b60 ConCommand dfs_print_flag_states
r5apex.exe!0x012d71a0 ConCommand dfs_print_true_flags
r5apex.exe!0x0119bab0 ConCommand disconnect
r5apex.exe!0x012d7ce0 ConCommand display_elapsedtime
r5apex.exe!0x01e90fe0 ConCommand dlight_debug
r5apex.exe!0x0582de00 ConCommand do_InvitePeople_test
r5apex.exe!0x0582d930 ConCommand do_Invite_friend_test
r5apex.exe!0x05828580 ConCommand do_joinPeople_test
r5apex.exe!0x0582d850 ConCommand do_origin_test_presence
r5apex.exe!0x011a8f70 ConCommand downloadPlaylists
r5apex.exe!0x01c67560 ConCommand dumpClientStringTable
r5apex.exe!0x011a4fc0 ConCommand dumpstringtables
r5apex.exe!0x01198f60 ConCommand echo
r5apex.exe!0x01199660 ConCommand echo_error
r5apex.exe!0x012dc640 ConCommand editor_toggle
r5apex.exe!0x0126f310 ConCommand endmovie
r5apex.exe!0x0119c2f0 ConCommand entitlements_print
r5apex.exe!0x012a5470 ConCommand entitlements_send
r5apex.exe!0x011a1f70 ConCommand entitlements_set_bits
r5apex.exe!0x011985a0 ConCommand envmap
r5apex.exe!0x0119cac0 ConCommand escape
r5apex.exe!0x0119b450 ConCommand exec
r5apex.exe!0x012d7ea0 ConCommand execPlayerConfig
r5apex.exe!0x011995f0 ConCommand execifexists
r5apex.exe!0x0119d0d0 ConCommand exit
r5apex.exe!0x0189cf50 ConCommand eyeInfo
r5apex.exe!0x01c4c5f0 ConCommand firstperson
r5apex.exe!0x0119c160 ConCommand flush
r5apex.exe!0x0119f930 ConCommand flush_locked
r5apex.exe!0x01c56250 ConCommand force_centerview
r5apex.exe!0x012e66f0 ConCommand fps_stats_dump
r5apex.exe!0x012e8830 ConCommand fps_stats_reset
r5apex.exe!0x012e6530 ConCommand fps_stats_start
r5apex.exe!0x012e8bd0 ConCommand fps_stats_stop
r5apex.exe!0x012db910 ConCommand friends_update
r5apex.exe!0x012de810 ConCommand fs_clear_open_duplicate_times
r5apex.exe!0x012dea70 ConCommand fs_dump_open_duplicate_times
r5apex.exe!0x012df1b0 ConCommand fs_fios_cancel_prefetches
r5apex.exe!0x012de770 ConCommand fs_fios_flush_cache
r5apex.exe!0x012dec30 ConCommand fs_fios_prefetch_file
r5apex.exe!0x012def80 ConCommand fs_fios_prefetch_file_in_pack
r5apex.exe!0x012def00 ConCommand fs_fios_print_prefetches
r5apex.exe!0x011999a0 ConCommand fs_printopenfiles
r5apex.exe!0x01199d80 ConCommand fs_warning_level
r5apex.exe!0x01c4b470 ConCommand fx_impact_reparse
r5apex.exe!0x011a6b00 ConCommand gameui_activate
r5apex.exe!0x011a9150 ConCommand gameui_allowescape
r5apex.exe!0x011aacb0 ConCommand gameui_allowescapetoshow
r5apex.exe!0x011a6880 ConCommand gameui_hide
r5apex.exe!0x011a62c0 ConCommand gameui_preventescape
r5apex.exe!0x011a7d90 ConCommand gameui_preventescapetoshow
r5apex.exe!0x012dcfd0 ConCommand getNewAuthToken
r5apex.exe!0x0189b820 ConCommand getfov
r5apex.exe!0x0119e5f0 ConCommand gethttpdatacenterlist
r5apex.exe!0x018a02d0 ConCommand getpos
r5apex.exe!0x018b5960 ConCommand getpos_bind
r5apex.exe!0x01c3a350 ConCommand getposvec
r5apex.exe!0x01e70f10 ConCommand give
r5apex.exe!0x01199d10 ConCommand help
r5apex.exe!0x01e93600 ConCommand hidepanel
r5apex.exe!0x01c5bb70 ConCommand hidevideos
r5apex.exe!0x012f1220 ConCommand highlight_log
r5apex.exe!0x012d7980 ConCommand host_runofftime
r5apex.exe!0x01c4e160 ConCommand hud_subtitles
r5apex.exe!0x012dd6b0 ConCommand huffman_readProps
r5apex.exe!0x01c53170 ConCommand impulse
r5apex.exe!0x012d5850 ConCommand inboxmessage_report
r5apex.exe!0x011a4e80 ConCommand incrementvar
r5apex.exe!0x01c5d660 ConCommand ingamemenu_activate
r5apex.exe!0x0119c930 ConCommand initMatchmaking
r5apex.exe!0x01c4c190 ConCommand invnext
r5apex.exe!0x018b60e0 ConCommand is_considered_sony_multiplayer
r5apex.exe!0x0126de10 ConCommand joinopeninvite
r5apex.exe!0x01c537f0 ConCommand joystick_initialize
r5apex.exe!0x0126e4c0 ConCommand jpeg
r5apex.exe!0x011a4a50 ConCommand key_listboundkeys
r5apex.exe!0x012e2ac0 ConCommand key_updatelayout
r5apex.exe!0x011a8b10 ConCommand launchplaylist
r5apex.exe!0x0126efb0 ConCommand leaveopeninvite
r5apex.exe!0x01c637f0 ConCommand listClientFXScriptHandles
r5apex.exe!0x011a0ea0 ConCommand listmodels
r5apex.exe!0x011a7550 ConCommand loadPlaylists
r5apex.exe!0x0119f4d0 ConCommand map
r5apex.exe!0x0119d7f0 ConCommand map_background
r5apex.exe!0x0119f890 ConCommand maps
r5apex.exe!0x0119ebe0 ConCommand mat_antialias_mode
r5apex.exe!0x011a5ce0 ConCommand mat_configcurrent
r5apex.exe!0x01199b50 ConCommand mat_crosshair
r5apex.exe!0x01199880 ConCommand mat_crosshair_edit
r5apex.exe!0x01198490 ConCommand mat_crosshair_explorer
r5apex.exe!0x01199c90 ConCommand mat_crosshair_printmaterial
r5apex.exe!0x0119b110 ConCommand mat_crosshair_reloadmaterial
r5apex.exe!0x011a0010 ConCommand mat_gamma
r5apex.exe!0x012ec2f0 ConCommand mat_hdr_enabled
r5apex.exe!0x0188d260 ConCommand mat_printLiveTex
r5apex.exe!0x0119f570 ConCommand mat_savechanges
r5apex.exe!0x011a28f0 ConCommand mat_setvideomode
r5apex.exe!0x011a5b00 ConCommand mat_vsync
r5apex.exe!0x011a2990 ConCommand match_abortAllSearches
r5apex.exe!0x0119e580 ConCommand match_showAllSearches
r5apex.exe!0x011a52e0 ConCommand matchmake
r5apex.exe!0x011a5560 ConCommand matchmake_cancel
r5apex.exe!0x011a30b0 ConCommand matchmake_cleanupforparty
r5apex.exe!0x012a6bd0 ConCommand maxplayers
r5apex.exe!0x012d7f20 ConCommand mem_compact
r5apex.exe!0x012d7000 ConCommand mem_dump
r5apex.exe!0x012d6d20 ConCommand mem_dump_vm
r5apex.exe!0x012d6860 ConCommand mem_eat
r5apex.exe!0x012d7c60 ConCommand mem_incremental_compact
r5apex.exe!0x012d7e00 ConCommand mem_leak_vm
r5apex.exe!0x012d7120 ConCommand mem_test
r5apex.exe!0x012ed4a0 ConCommand mem_textures
r5apex.exe!0x012d8360 ConCommand mem_verify
r5apex.exe!0x012ed980 ConCommand mem_vram
r5apex.exe!0x011a16a0 ConCommand memory
r5apex.exe!0x011a3a50 ConCommand migrateme
r5apex.exe!0x01e7eb40 ConCommand miles_dump
r5apex.exe!0x01e7ecd0 ConCommand miles_event_info
r5apex.exe!0x01e7f690 ConCommand miles_pauseui_byname
r5apex.exe!0x01e80340 ConCommand miles_play
r5apex.exe!0x01e80580 ConCommand miles_reboot
r5apex.exe!0x01e7d530 ConCommand miles_record
r5apex.exe!0x01e7e3c0 ConCommand miles_record_that
r5apex.exe!0x01e7dfd0 ConCommand miles_stop_all
r5apex.exe!0x01e7da50 ConCommand miles_unpauseui_byname
r5apex.exe!0x01e7e640 ConCommand miles_write_passive_dumpfile
r5apex.exe!0x0126d940 ConCommand mmdevinit
r5apex.exe!0x0119ba40 ConCommand multvar
r5apex.exe!0x012dcf50 ConCommand muteroom
r5apex.exe!0x0119c1d0 ConCommand net_channels
r5apex.exe!0x0126eac0 ConCommand net_dumpIncomingStats
r5apex.exe!0x0126e670 ConCommand net_dumpOutgoingStats
r5apex.exe!0x0126e920 ConCommand net_dumpStats
r5apex.exe!0x011a3150 ConCommand net_start
r5apex.exe!0x011a09f0 ConCommand net_status
r5apex.exe!0x012d7540 ConCommand net_writeStatsFile
r5apex.exe!0x0126ed80 ConCommand openinvite
r5apex.exe!0x0126e2a0 ConCommand openinvitecomplete
r5apex.exe!0x0126e070 ConCommand openinvitelaunch
r5apex.exe!0x0582d8c0 ConCommand origin_friendlist_dump
r5apex.exe!0x018aaf10 ConCommand particle_create
r5apex.exe!0x018adf10 ConCommand particle_create_on_me
r5apex.exe!0x01c3b270 ConCommand particle_create_ss
r5apex.exe!0x01c5d4a0 ConCommand particle_dump
r5apex.exe!0x018a0980 ConCommand particle_kill
r5apex.exe!0x01c60f00 ConCommand particle_list
r5apex.exe!0x018aadf0 ConCommand particle_recreate
r5apex.exe!0x01c5c0a0 ConCommand particle_remove_all
r5apex.exe!0x018a86d0 ConCommand particle_scrub_bake
r5apex.exe!0x01c44980 ConCommand particle_scrub_play
r5apex.exe!0x01c3a190 ConCommand particle_scrub_stop
r5apex.exe!0x012d9d50 ConCommand party_leave
r5apex.exe!0x012d9910 ConCommand party_serverChange
r5apex.exe!0x0119b4c0 ConCommand path
r5apex.exe!0x0119cf20 ConCommand pause
r5apex.exe!0x01c5eb00 ConCommand pausevideos
r5apex.exe!0x0188f880 ConCommand perfcharts_record
r5apex.exe!0x01c5ed60 ConCommand phys_objectDump
r5apex.exe!0x01e90820 ConCommand phys_throw_client
r5apex.exe!0x0119ddc0 ConCommand ping
r5apex.exe!0x01c5e800 ConCommand ping_specific_type
r5apex.exe!0x011a5240 ConCommand pingdatacenters
r5apex.exe!0x018a0410 ConCommand pixelvis_debug
r5apex.exe!0x01cac090 ConCommand playerSettings_reparse
r5apex.exe!0x011aa8f0 ConCommand playlistdump
r5apex.exe!0x01c4a6e0 ConCommand playsoundscape
r5apex.exe!0x01c5f600 ConCommand playvideo
r5apex.exe!0x01c5d940 ConCommand playvideo_end_level_transition
r5apex.exe!0x01c61620 ConCommand playvideo_exitcommand
r5apex.exe!0x01c61ac0 ConCommand playvideo_exitcommand_nointerrupt
r5apex.exe!0x01c62360 ConCommand playvideo_nointerrupt
r5apex.exe!0x01c5f3a0 ConCommand playvideo_scaled
r5apex.exe!0x01198cd0 ConCommand print_colorcorrection
r5apex.exe!0x011a99d0 ConCommand progress_enable
r5apex.exe!0x011a0e00 ConCommand quit
r5apex.exe!0x01cb05d0 ConCommand r_cheapwaterend
r5apex.exe!0x01e65080 ConCommand r_cheapwaterstart
r5apex.exe!0x0119eee0 ConCommand r_cleardecals
r5apex.exe!0x012f34a0 ConCommand r_dxgi_max_frame_latency
r5apex.exe!0x011a8250 ConCommand r_printdecalinfo
r5apex.exe!0x012d4f30 ConCommand readMsgs
r5apex.exe!0x011a57e0 ConCommand recheck
r5apex.exe!0x012d6f80 ConCommand recompute_speed
r5apex.exe!0x0126d520 ConCommand reconnect
r5apex.exe!0x011a2cb0 ConCommand reload
r5apex.exe!0x012a56d0 ConCommand reload_localization
r5apex.exe!0x01e65ac0 ConCommand reload_script_callbacks
r5apex.exe!0x01c4ea30 ConCommand reset_cam_ideal_angles
r5apex.exe!0x011a34f0 ConCommand restart
r5apex.exe!0x011a0d90 ConCommand restart_checkpoint
r5apex.exe!0x018ac550 ConCommand roamingcam_setang
r5apex.exe!0x01c3a490 ConCommand roamingcam_setpos
r5apex.exe!0x018b52f0 ConCommand roamingcam_setroll
r5apex.exe!0x01e7f3f0 ConCommand rumble_print
r5apex.exe!0x012d6520 ConCommand savePlayerConfig
r5apex.exe!0x01c60ae0 ConCommand scoreboard_down
r5apex.exe!0x01c5e1e0 ConCommand scoreboard_focus
r5apex.exe!0x01c61740 ConCommand scoreboard_mute
r5apex.exe!0x01c62d30 ConCommand scoreboard_profile
r5apex.exe!0x01c60980 ConCommand scoreboard_toggle_focus
r5apex.exe!0x01c5fc00 ConCommand scoreboard_up
r5apex.exe!0x0126d8c0 ConCommand screenshot
r5apex.exe!0x011a0a70 ConCommand server_single_frame
r5apex.exe!0x011a7fd0 ConCommand serverinfo
r5apex.exe!0x01cacea0 ConCommand set
r5apex.exe!0x0119eda0 ConCommand set_loading_progress_background
r5apex.exe!0x011a5c40 ConCommand set_loading_progress_detente
r5apex.exe!0x0119be70 ConCommand set_loading_progress_fadeout_enabled
r5apex.exe!0x0119bcf0 ConCommand set_loading_progress_sp_text
r5apex.exe!0x0126ebe0 ConCommand setinfo
r5apex.exe!0x0119a790 ConCommand settype
r5apex.exe!0x01c61900 ConCommand shake_stop
r5apex.exe!0x01c61d40 ConCommand shake_testpunch
r5apex.exe!0x011a5e60 ConCommand show_loading_progress
r5apex.exe!0x01e93580 ConCommand showpanel
r5apex.exe!0x01c60d40 ConCommand showvideos
r5apex.exe!0x0126dba0 ConCommand silentconnect
r5apex.exe!0x011a9590 ConCommand skill_writeTrainingData
r5apex.exe!0x0189df40 ConCommand soundscape_dumpclient
r5apex.exe!0x01c5c1c0 ConCommand spawn_as_pilot
r5apex.exe!0x01c60f80 ConCommand spawn_as_titan
r5apex.exe!0x011a06f0 ConCommand ss_map
r5apex.exe!0x01c5b2f0 ConCommand ss_reloadletterbox
r5apex.exe!0x01e643c0 ConCommand sssss_enable
r5apex.exe!0x011a7f50 ConCommand star_memory
r5apex.exe!0x0126f1f0 ConCommand startmovie
r5apex.exe!0x011a5060 ConCommand status
r5apex.exe!0x012dbdd0 ConCommand steam_printid
r5apex.exe!0x012dbcb0 ConCommand steam_testOverlay
r5apex.exe!0x012db770 ConCommand steamlink
r5apex.exe!0x012db890 ConCommand steamunlink
r5apex.exe!0x01c5e3a0 ConCommand stop_transition_videos_fadeout
r5apex.exe!0x01c56650 ConCommand stopsoundscape
r5apex.exe!0x01c5d110 ConCommand stopvideos
r5apex.exe!0x01c59ad0 ConCommand stopvideos_fadeout
r5apex.exe!0x012a6830 ConCommand sv_precacheinfo
r5apex.exe!0x012a7d90 ConCommand sv_showents
r5apex.exe!0x012a78d0 ConCommand sv_shutdown
r5apex.exe!0x012a91d0 ConCommand sv_writeSendTableStreamFile
r5apex.exe!0x01898580 ConCommand testCockpitJoltAngles
r5apex.exe!0x0189e980 ConCommand testCockpitJoltOrigin
r5apex.exe!0x01cafd10 ConCommand test_freezeframe
r5apex.exe!0x01c58070 ConCommand testhudanim
r5apex.exe!0x012d7be0 ConCommand thread_test_tslist
r5apex.exe!0x012d6460 ConCommand thread_test_tsqueue
r5apex.exe!0x01c5baf0 ConCommand titan_loadout_select
r5apex.exe!0x0119ae80 ConCommand toggle
r5apex.exe!0x01c61300 ConCommand toggle_inventory
r5apex.exe!0x01c5d230 ConCommand toggle_map
r5apex.exe!0x01e7b9f0 ConCommand ui_reloadscheme
r5apex.exe!0x01e69170 ConCommand uiscript_reset
r5apex.exe!0x01e6a060 ConCommand uiscript_resolutionchanged
r5apex.exe!0x0119dd50 ConCommand unbind
r5apex.exe!0x0119f430 ConCommand unbind_US_standard
r5apex.exe!0x011a2690 ConCommand unbind_all_gamepad
r5apex.exe!0x011a11c0 ConCommand unbind_batch
r5apex.exe!0x011a23a0 ConCommand unbind_held
r5apex.exe!0x0119ceb0 ConCommand unbind_held_US_standard
r5apex.exe!0x0119f610 ConCommand unbindall
r5apex.exe!0x0119eb40 ConCommand unbindall_ignoreGamepad
r5apex.exe!0x011a2290 ConCommand unload_level_loadscreen
r5apex.exe!0x012dcc70 ConCommand unmuteroom
r5apex.exe!0x01c5fd20 ConCommand unpausevideos
r5apex.exe!0x01c5f7c0 ConCommand use_consumable
r5apex.exe!0x012a8450 ConCommand user
r5apex.exe!0x012a7c70 ConCommand users
r5apex.exe!0x011a2440 ConCommand version
r5apex.exe!0x011a8550 ConCommand vgui_drawtree_clear
r5apex.exe!0x01e98b90 ConCommand vgui_spew_fonts
r5apex.exe!0x011a9f10 ConCommand vgui_togglepanel
r5apex.exe!0x0119d640 ConCommand voicerecord_toggle
r5apex.exe!0x0119bb30 ConCommand vx_datacache_list
r5apex.exe!0x0119de30 ConCommand vx_model_list
r5apex.exe!0x01c4dae0 ConCommand weaponSelectOrdnance
r5apex.exe!0x01c558a0 ConCommand weaponSelectPrimary0
r5apex.exe!0x01c4a880 ConCommand weaponSelectPrimary1
r5apex.exe!0x01c499a0 ConCommand weaponSelectPrimary2
r5apex.exe!0x01c536d0 ConCommand weapon_activity
r5apex.exe!0x01c45840 ConCommand weapon_inspect
r5apex.exe!0x018ab0a0 ConCommand weapon_list
r5apex.exe!0x01e6ad00 ConCommand weapon_reparse
r5apex.exe!0x0119eff0 ConCommand xlog_list
r5apex.exe!0x011a5920 ConCommand xlog_record
r5apex.exe!0x011a2310 ConCommand xlog_record_that
r5apex.exe!0x0119e6e0 ConCommand xlog_stop
r5apex.exe!0x01c51ed0 ConCommand xlook
r5apex.exe!0x01c46160 ConCommand xmove
```

## Globals

List of global variables with an associated vtable and their type name.

```
r5apex.exe!0x018903b0 .?AUCCallbackInternal_OnGetAuthTicket@SteamWrapper_CallbackHandler_s@@
r5apex.exe!0x018903c0 .?AUCCallbackInternal_OnMicroTxnAuthorization@SteamWrapper_CallbackHandler_s@@
r5apex.exe!0x018903d0 .?AUCCallbackInternal_OnOverlayActivated@SteamWrapper_CallbackHandler_s@@
r5apex.exe!0x01e9c240 .?AUSQArray@@
r5apex.exe!0x01e9c178 .?AUSQClass@@
r5apex.exe!0x01e9c128 .?AUSQClosure@@
r5apex.exe!0x01e9c010 .?AUSQFunctionProto@@
r5apex.exe!0x01e9be08 .?AUSQInstance@@
r5apex.exe!0x01e9c268 .?AUSQNativeClosure@@
r5apex.exe!0x01e9bf70 .?AUSQString@@
r5apex.exe!0x01e9c1f0 .?AUSQStructDef@@
r5apex.exe!0x01e9bfc0 .?AUSQStructInstance@@
r5apex.exe!0x01e9bfe8 .?AUSQTable@@
r5apex.exe!0x01e9bed0 .?AUSQUserData@@
r5apex.exe!0x01e9c290 .?AUSQVM@@
r5apex.exe!0x01e9c2b8 .?AUSQWeakRef@@
r5apex.exe!0x058ec808 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058ec870 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058ec878 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058ec898 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058ec900 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058ec908 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058ec928 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058ec990 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058ec998 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058ec9b8 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058eca20 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x058eca28 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x011977a8 .?AV?$CDataManager@UDataCacheItem_t@@UDataCacheItemData_t@@PEAU1@VCThreadFastMutex@@@@
r5apex.exe!0x041b13f0 .?AV?$CDataManager@VCBoneCache@@Ubonecacheparams_t@@PEAV1@VCThreadFastMutex@@@@
r5apex.exe!0x041b14a0 .?AV?$CDataManager@VCBoneCache@@Ubonecacheparams_t@@PEAV1@VCThreadFastMutex@@@@
r5apex.exe!0x0126e178 .?AV?$CPanelFactory@VCMovieDisplayScreen@@UVGuiScreenInitData_t@@@@
r5apex.exe!0x0126dc08 .?AV?$CPanelFactory@VCVGuiScreenPanel@@UVGuiScreenInitData_t@@@@
r5apex.exe!0x01e95b88 .?AV?$CParticleOperatorDefinition@VC_INIT_AgeNoise@@@@
r5apex.exe!0x01e95a68 .?AV?$CParticleOperatorDefinition@VC_INIT_ChaoticAttractor@@@@
r5apex.exe!0x01e95bd0 .?AV?$CParticleOperatorDefinition@VC_INIT_ColorLitPerParticle@@@@
r5apex.exe!0x01e91dc8 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateAlongPath@@@@
r5apex.exe!0x01e8d668 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateFromParentParticles@@@@
r5apex.exe!0x01e95be8 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateFromPlaneCache@@@@
r5apex.exe!0x01e95c60 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateInEpitrochoid@@@@
r5apex.exe!0x01e95af8 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateInHierarchy@@@@
r5apex.exe!0x01e890c8 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateOnModel@@@@
r5apex.exe!0x01e95c00 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateSequentialPath@@@@
r5apex.exe!0x01e95b40 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinBox@@@@
r5apex.exe!0x01e935e8 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinControlPointBox@@@@
r5apex.exe!0x01e95dd8 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinSphere@@@@
r5apex.exe!0x01e91328 .?AV?$CParticleOperatorDefinition@VC_INIT_CreationNoise@@@@
r5apex.exe!0x01e95d60 .?AV?$CParticleOperatorDefinition@VC_INIT_DistanceToCPInit@@@@
r5apex.exe!0x01e91b88 .?AV?$CParticleOperatorDefinition@VC_INIT_InheritFromParentParticles@@@@
r5apex.exe!0x01e95b28 .?AV?$CParticleOperatorDefinition@VC_INIT_InheritVelocity@@@@
r5apex.exe!0x01e95a80 .?AV?$CParticleOperatorDefinition@VC_INIT_InitFromParentKilled@@@@
r5apex.exe!0x01e95b10 .?AV?$CParticleOperatorDefinition@VC_INIT_InitialRepulsionVelocity@@@@
r5apex.exe!0x01e95df0 .?AV?$CParticleOperatorDefinition@VC_INIT_InitialVelocityNoise@@@@
r5apex.exe!0x01e91048 .?AV?$CParticleOperatorDefinition@VC_INIT_LifespanFromVelocity@@@@
r5apex.exe!0x01e95c48 .?AV?$CParticleOperatorDefinition@VC_INIT_ModelCull@@@@
r5apex.exe!0x01e95ac8 .?AV?$CParticleOperatorDefinition@VC_INIT_MoveBetweenPoints@@@@
r5apex.exe!0x01e95ae0 .?AV?$CParticleOperatorDefinition@VC_INIT_NormalAlignToCP@@@@
r5apex.exe!0x01e95d90 .?AV?$CParticleOperatorDefinition@VC_INIT_NormalOffset@@@@
r5apex.exe!0x01e913a8 .?AV?$CParticleOperatorDefinition@VC_INIT_OffsetVectorToVector@@@@
r5apex.exe!0x01e95b58 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionOffset@@@@
r5apex.exe!0x01e88348 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionPlaceOnGround@@@@
r5apex.exe!0x01e95b70 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionWarp@@@@
r5apex.exe!0x01e95bb8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomAlpha@@@@
r5apex.exe!0x01e90fc8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomColor@@@@
r5apex.exe!0x01e93668 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomLifeTime@@@@
r5apex.exe!0x01e805e8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRadius@@@@
r5apex.exe!0x01e90888 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRotation@@@@
r5apex.exe!0x01e7f6f8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRotationSpeed@@@@
r5apex.exe!0x01e92708 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomScalar@@@@
r5apex.exe!0x01e95028 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomSecondSequence@@@@
r5apex.exe!0x01e95dc0 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomSequence@@@@
r5apex.exe!0x01e822f8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomTrailLength@@@@
r5apex.exe!0x01e95e08 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomVector@@@@
r5apex.exe!0x01e95c30 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomVectorComponent@@@@
r5apex.exe!0x01e95d78 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomYaw@@@@
r5apex.exe!0x01e80708 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomYawFlip@@@@
r5apex.exe!0x01e912a8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapCPtoScalar@@@@
r5apex.exe!0x01e89f08 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapCPtoVector@@@@
r5apex.exe!0x01e91ca8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapInitialCPDirectionToRotation@@@@
r5apex.exe!0x01e8c148 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapInitialDirectionToCPToVector@@@@
r5apex.exe!0x01e95a50 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapParticleCountToScalar@@@@
r5apex.exe!0x01e86378 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapScalar@@@@
r5apex.exe!0x01e95c78 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapScalarToVector@@@@
r5apex.exe!0x01e803a8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapSpeedToScalar@@@@
r5apex.exe!0x01e95ca8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapWorldCPtoScreen@@@@
r5apex.exe!0x01e95da8 .?AV?$CParticleOperatorDefinition@VC_INIT_RingWave@@@@
r5apex.exe!0x01e95c18 .?AV?$CParticleOperatorDefinition@VC_INIT_SequenceFromCP@@@@
r5apex.exe!0x01e90908 .?AV?$CParticleOperatorDefinition@VC_INIT_SequenceLifeTime@@@@
r5apex.exe!0x01e95ba0 .?AV?$CParticleOperatorDefinition@VC_INIT_SetCPPosition@@@@
r5apex.exe!0x01e92888 .?AV?$CParticleOperatorDefinition@VC_INIT_SetHitboxToClosest@@@@
r5apex.exe!0x01e95ab0 .?AV?$CParticleOperatorDefinition@VC_INIT_SetHitboxToModel@@@@
r5apex.exe!0x01e95a98 .?AV?$CParticleOperatorDefinition@VC_INIT_VelocityFromCP@@@@
r5apex.exe!0x01e95c90 .?AV?$CParticleOperatorDefinition@VC_INIT_VelocityRandom@@@@
r5apex.exe!0x01e965b8 .?AV?$CParticleOperatorDefinition@VC_OP_AlphaDecay@@@@
r5apex.exe!0x01e95eb0 .?AV?$CParticleOperatorDefinition@VC_OP_AttractToControlPoint@@@@
r5apex.exe!0x01e966d0 .?AV?$CParticleOperatorDefinition@VC_OP_AxisSpin@@@@
r5apex.exe!0x01e96048 .?AV?$CParticleOperatorDefinition@VC_OP_BasicMovement@@@@
r5apex.exe!0x01e7e6a8 .?AV?$CParticleOperatorDefinition@VC_OP_BoxConstraint@@@@
r5apex.exe!0x01e95f58 .?AV?$CParticleOperatorDefinition@VC_OP_CPOffsetToPercentageBetweenCPs@@@@
r5apex.exe!0x01e968e8 .?AV?$CParticleOperatorDefinition@VC_OP_ClampScalar@@@@
r5apex.exe!0x01e96390 .?AV?$CParticleOperatorDefinition@VC_OP_ClampVector@@@@
r5apex.exe!0x01e96288 .?AV?$CParticleOperatorDefinition@VC_OP_ColorInterpolate@@@@
r5apex.exe!0x01e7ed38 .?AV?$CParticleOperatorDefinition@VC_OP_ConstrainDistance@@@@
r5apex.exe!0x01e7f308 .?AV?$CParticleOperatorDefinition@VC_OP_ConstrainDistanceToPath@@@@
r5apex.exe!0x01e95e80 .?AV?$CParticleOperatorDefinition@VC_OP_ContinuousEmitter@@@@
r5apex.exe!0x01e96700 .?AV?$CParticleOperatorDefinition@VC_OP_ControlpointLight@@@@
r5apex.exe!0x01e96090 .?AV?$CParticleOperatorDefinition@VC_OP_Cull@@@@
r5apex.exe!0x01e968d0 .?AV?$CParticleOperatorDefinition@VC_OP_DampenToCP@@@@
r5apex.exe!0x01e95fe8 .?AV?$CParticleOperatorDefinition@VC_OP_Decay@@@@
r5apex.exe!0x01e965e8 .?AV?$CParticleOperatorDefinition@VC_OP_DecayMaintainCount@@@@
r5apex.exe!0x01e96770 .?AV?$CParticleOperatorDefinition@VC_OP_DifferencePreviousParticle@@@@
r5apex.exe!0x01e96420 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceBetweenCPs@@@@
r5apex.exe!0x01e96348 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceBetweenCPsToCP@@@@
r5apex.exe!0x01e96840 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceCull@@@@
r5apex.exe!0x01e95e20 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceEmitter@@@@
r5apex.exe!0x01e962e8 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceToCP@@@@
r5apex.exe!0x01e96018 .?AV?$CParticleOperatorDefinition@VC_OP_FadeAndKill@@@@
r5apex.exe!0x01e96758 .?AV?$CParticleOperatorDefinition@VC_OP_FadeAndKillForTracers@@@@
r5apex.exe!0x01e962d0 .?AV?$CParticleOperatorDefinition@VC_OP_FadeIn@@@@
r5apex.exe!0x01e961b0 .?AV?$CParticleOperatorDefinition@VC_OP_FadeInSimple@@@@
r5apex.exe!0x01e963f0 .?AV?$CParticleOperatorDefinition@VC_OP_FadeOut@@@@
r5apex.exe!0x01e964f8 .?AV?$CParticleOperatorDefinition@VC_OP_FadeOutSimple@@@@
r5apex.exe!0x01e95ec8 .?AV?$CParticleOperatorDefinition@VC_OP_ForceBasedOnDistanceToPlane@@@@
r5apex.exe!0x01e96558 .?AV?$CParticleOperatorDefinition@VC_OP_GraphScalar@@@@
r5apex.exe!0x01e967e8 .?AV?$CParticleOperatorDefinition@VC_OP_GraphVector@@@@
r5apex.exe!0x01e96138 .?AV?$CParticleOperatorDefinition@VC_OP_InheritFromParentParticles@@@@
r5apex.exe!0x01e95e98 .?AV?$CParticleOperatorDefinition@VC_OP_InstantaneousDistanceEmitter@@@@
r5apex.exe!0x01e95e38 .?AV?$CParticleOperatorDefinition@VC_OP_InstantaneousEmitter@@@@
r5apex.exe!0x01e96858 .?AV?$CParticleOperatorDefinition@VC_OP_InterpolateRadius@@@@
r5apex.exe!0x01e96060 .?AV?$CParticleOperatorDefinition@VC_OP_LagCompensation@@@@
r5apex.exe!0x01e95fa0 .?AV?$CParticleOperatorDefinition@VC_OP_LerpEndCapScalar@@@@
r5apex.exe!0x01e96438 .?AV?$CParticleOperatorDefinition@VC_OP_LerpEndCapVector@@@@
r5apex.exe!0x01e961e0 .?AV?$CParticleOperatorDefinition@VC_OP_LerpScalar@@@@
r5apex.exe!0x01e96408 .?AV?$CParticleOperatorDefinition@VC_OP_LerpVector@@@@
r5apex.exe!0x01e96258 .?AV?$CParticleOperatorDefinition@VC_OP_LockToBone@@@@
r5apex.exe!0x01e96468 .?AV?$CParticleOperatorDefinition@VC_OP_LockToSavedSequentialPath@@@@
r5apex.exe!0x01e95e50 .?AV?$CParticleOperatorDefinition@VC_OP_MaintainEmitter@@@@
r5apex.exe!0x01e96888 .?AV?$CParticleOperatorDefinition@VC_OP_MaintainSequentialPath@@@@
r5apex.exe!0x01e963d8 .?AV?$CParticleOperatorDefinition@VC_OP_MaxVelocity@@@@
r5apex.exe!0x01e96270 .?AV?$CParticleOperatorDefinition@VC_OP_ModelCull@@@@
r5apex.exe!0x01e96740 .?AV?$CParticleOperatorDefinition@VC_OP_MoveToHitbox@@@@
r5apex.exe!0x01e96078 .?AV?$CParticleOperatorDefinition@VC_OP_MovementMaintainOffset@@@@
r5apex.exe!0x01e967a0 .?AV?$CParticleOperatorDefinition@VC_OP_MovementPlaceOnGround@@@@
r5apex.exe!0x01e96600 .?AV?$CParticleOperatorDefinition@VC_OP_MovementRotateParticleAroundAxis@@@@
r5apex.exe!0x01e962b8 .?AV?$CParticleOperatorDefinition@VC_OP_Noise@@@@
r5apex.exe!0x01e95e68 .?AV?$CParticleOperatorDefinition@VC_OP_NoiseEmitter@@@@
r5apex.exe!0x01e965d0 .?AV?$CParticleOperatorDefinition@VC_OP_NormalLock@@@@
r5apex.exe!0x01e96330 .?AV?$CParticleOperatorDefinition@VC_OP_NormalizeVector@@@@
r5apex.exe!0x01e96788 .?AV?$CParticleOperatorDefinition@VC_OP_Orient2DRelToCP@@@@
r5apex.exe!0x01e967b8 .?AV?$CParticleOperatorDefinition@VC_OP_OrientTo2dDirection@@@@
r5apex.exe!0x01e96498 .?AV?$CParticleOperatorDefinition@VC_OP_OrientTowardPlayer@@@@
r5apex.exe!0x01e96120 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateScalar@@@@
r5apex.exe!0x01e960d8 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateScalarSimple@@@@
r5apex.exe!0x01e96810 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateVector@@@@
r5apex.exe!0x01e96030 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateVectorSimple@@@@
r5apex.exe!0x01e95f10 .?AV?$CParticleOperatorDefinition@VC_OP_ParentVortices@@@@
r5apex.exe!0x01e96540 .?AV?$CParticleOperatorDefinition@VC_OP_PercentageBetweenCPs@@@@
r5apex.exe!0x01e96168 .?AV?$CParticleOperatorDefinition@VC_OP_PercentageBetweenCPsVector@@@@
r5apex.exe!0x01e7ebc8 .?AV?$CParticleOperatorDefinition@VC_OP_PlanarConstraint@@@@
r5apex.exe!0x01e96510 .?AV?$CParticleOperatorDefinition@VC_OP_PlaneCull@@@@
r5apex.exe!0x01e963a8 .?AV?$CParticleOperatorDefinition@VC_OP_PositionBetweenCPs@@@@
r5apex.exe!0x01e95fb8 .?AV?$CParticleOperatorDefinition@VC_OP_PositionLock@@@@
r5apex.exe!0x01e96718 .?AV?$CParticleOperatorDefinition@VC_OP_ProjectileArc@@@@
r5apex.exe!0x01e960a8 .?AV?$CParticleOperatorDefinition@VC_OP_RadiusDecay@@@@
r5apex.exe!0x01e965a0 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarLinear@@@@
r5apex.exe!0x01e96828 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarLinearSimple@@@@
r5apex.exe!0x01e96000 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarSpline@@@@
r5apex.exe!0x01e961f8 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarSplineSimple@@@@
r5apex.exe!0x01e95f40 .?AV?$CParticleOperatorDefinition@VC_OP_RandomForce@@@@
r5apex.exe!0x01e967d0 .?AV?$CParticleOperatorDefinition@VC_OP_RemapAverageScalarValuetoCP@@@@
r5apex.exe!0x01e95f88 .?AV?$CParticleOperatorDefinition@VC_OP_RemapBoundingVolumetoCP@@@@
r5apex.exe!0x01e96240 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPVelocityToVector@@@@
r5apex.exe!0x01e96870 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPtoScalar@@@@
r5apex.exe!0x01e96360 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPtoVector@@@@
r5apex.exe!0x01e963c0 .?AV?$CParticleOperatorDefinition@VC_OP_RemapControlPointDirectionToVector@@@@
r5apex.exe!0x01e964c8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapDirectionToCPToVector@@@@
r5apex.exe!0x01e96108 .?AV?$CParticleOperatorDefinition@VC_OP_RemapDotProductToScalar@@@@
r5apex.exe!0x01e96570 .?AV?$CParticleOperatorDefinition@VC_OP_RemapModelVolumetoCP@@@@
r5apex.exe!0x01e96528 .?AV?$CParticleOperatorDefinition@VC_OP_RemapScalar@@@@
r5apex.exe!0x01e96300 .?AV?$CParticleOperatorDefinition@VC_OP_RemapSpeed@@@@
r5apex.exe!0x01e96450 .?AV?$CParticleOperatorDefinition@VC_OP_RemapSpeedtoCP@@@@
r5apex.exe!0x01e95fd0 .?AV?$CParticleOperatorDefinition@VC_OP_RemapVelocityToVector@@@@
r5apex.exe!0x01e961c8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapWorldCPToScreen@@@@
r5apex.exe!0x01e969a0 .?AV?$CParticleOperatorDefinition@VC_OP_RenderDecal@@@@
r5apex.exe!0x01e96b80 .?AV?$CParticleOperatorDefinition@VC_OP_RenderLightSource@@@@
r5apex.exe!0x01e969e8 .?AV?$CParticleOperatorDefinition@VC_OP_RenderModels@@@@
r5apex.exe!0x01e969b8 .?AV?$CParticleOperatorDefinition@VC_OP_RenderPoints@@@@
r5apex.exe!0x01e96c50 .?AV?$CParticleOperatorDefinition@VC_OP_RenderRope@@@@
r5apex.exe!0x01e96b68 .?AV?$CParticleOperatorDefinition@VC_OP_RenderScreenVelocityRotate@@@@
r5apex.exe!0x01e969d0 .?AV?$CParticleOperatorDefinition@VC_OP_RenderScripts@@@@
r5apex.exe!0x01e96b98 .?AV?$CParticleOperatorDefinition@VC_OP_RenderSprites@@@@
r5apex.exe!0x01e96b50 .?AV?$CParticleOperatorDefinition@VC_OP_RenderSpritesTrail@@@@
r5apex.exe!0x01e964e0 .?AV?$CParticleOperatorDefinition@VC_OP_RestartAfterDuration@@@@
r5apex.exe!0x01e960c0 .?AV?$CParticleOperatorDefinition@VC_OP_RotateVector@@@@
r5apex.exe!0x01e96378 .?AV?$CParticleOperatorDefinition@VC_OP_SetCPOrientationToDirection@@@@
r5apex.exe!0x01e96180 .?AV?$CParticleOperatorDefinition@VC_OP_SetChildControlPoints@@@@
r5apex.exe!0x01e962a0 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointPositions@@@@
r5apex.exe!0x01e968a0 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointRotation@@@@
r5apex.exe!0x01e96588 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToCenter@@@@
r5apex.exe!0x01e960f0 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToImpactPoint@@@@
r5apex.exe!0x01e96228 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToPlayer@@@@
r5apex.exe!0x01e96318 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointsToParticle@@@@
r5apex.exe!0x01e96198 .?AV?$CParticleOperatorDefinition@VC_OP_SetPerChildControlPoint@@@@
r5apex.exe!0x01e966e8 .?AV?$CParticleOperatorDefinition@VC_OP_SoundMeterScalar@@@@
r5apex.exe!0x01e95f70 .?AV?$CParticleOperatorDefinition@VC_OP_Spin@@@@
r5apex.exe!0x01e96210 .?AV?$CParticleOperatorDefinition@VC_OP_SpinUpdate@@@@
r5apex.exe!0x01e964b0 .?AV?$CParticleOperatorDefinition@VC_OP_SpinYaw@@@@
r5apex.exe!0x01e968b8 .?AV?$CParticleOperatorDefinition@VC_OP_StopAfterCPDuration@@@@
r5apex.exe!0x01e95ef8 .?AV?$CParticleOperatorDefinition@VC_OP_TimeVaryingForce@@@@
r5apex.exe!0x01e95ee0 .?AV?$CParticleOperatorDefinition@VC_OP_TurbulenceForce@@@@
r5apex.exe!0x01e95f28 .?AV?$CParticleOperatorDefinition@VC_OP_TwistAroundAxis@@@@
r5apex.exe!0x01e96150 .?AV?$CParticleOperatorDefinition@VC_OP_VectorNoise@@@@
r5apex.exe!0x01e96480 .?AV?$CParticleOperatorDefinition@VC_OP_VelocityDecay@@@@
r5apex.exe!0x01e96618 .?AV?$CParticleOperatorDefinition@VC_OP_VelocityMatchingForce@@@@
r5apex.exe!0x01e7e448 .?AV?$CParticleOperatorDefinition@VC_OP_WorldCollideConstraint@@@@
r5apex.exe!0x01e7f458 .?AV?$CParticleOperatorDefinition@VC_OP_WorldTraceConstraint@@@@
r5apex.exe!0x018ae2b0 .?AV?$C_EntityClassList@VC_PointCamera@@@@
r5apex.exe!0x01c4e8e8 .?AV?$C_EntityClassList@VC_TriggerPlayerMovement@@@@
r5apex.exe!0x01ec5370 .?AVAddressV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5d80 .?AVAssignErrorV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4920 .?AVBinaryMessage@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x018b5910 .?AVCAimAssistTargets@@
r5apex.exe!0x018b5918 .?AVCAimAssistTargets@@
r5apex.exe!0x011ac840 .?AVCAvi@@
r5apex.exe!0x011a24a8 .?AVCBaseClientRenderTargets@@
r5apex.exe!0x01e9d178 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9d2e8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9d7f8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9ec68 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9eff8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f018 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f208 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f228 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f248 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f268 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f288 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f3b8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f448 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f468 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f488 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e9f4a8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea4588 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea4618 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea46a8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea4738 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea4988 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea4b58 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea4f88 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea5018 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea50a8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea50c8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea5238 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea7cb8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01ea7d78 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x04f74bc0 .?AVCBik@@
r5apex.exe!0x01e97d58 .?AVCBoolProperty@@
r5apex.exe!0x01c3b570 .?AVCCascadeLightManager@@
r5apex.exe!0x01c62940 .?AVCCenterPrint@@
r5apex.exe!0x01e9d900 .?AVCClassMap@@
r5apex.exe!0x01c5d350 .?AVCClientCollisionEvent@@
r5apex.exe!0x011a2378 .?AVCClientDLLSharedAppSystems@@
r5apex.exe!0x018b9a90 .?AVCClientEntityList@@
r5apex.exe!0x01b39ae8 .?AVCClientEntityList@@
r5apex.exe!0x0643a420 .?AVCClientLeafSystem@@
r5apex.exe!0x064332b0 .?AVCClientShadowMgr@@
r5apex.exe!0x0119aee8 .?AVCClientSound@@
r5apex.exe!0x0126f850 .?AVCClientState@@
r5apex.exe!0x0126f858 .?AVCClientState@@
r5apex.exe!0x0126f860 .?AVCClientState@@
r5apex.exe!0x0126f868 .?AVCClientState@@
r5apex.exe!0x01c45260 .?AVCClientThinkList@@
r5apex.exe!0x011963b0 .?AVCCmdLibFileLoggingListener@@
r5apex.exe!0x01196118 .?AVCCmdLibStandardLoggingListener@@
r5apex.exe!0x018a0650 .?AVCColorCorrectionMgr@@
r5apex.exe!0x012e2d80 .?AVCColorCorrectionSystem@@
r5apex.exe!0x01e97d48 .?AVCColorProperty@@
r5apex.exe!0x0188fb80 .?AVCCommandLine@@
r5apex.exe!0x058ea270 .?AVCCountedStringPool@@
r5apex.exe!0x058ea2c0 .?AVCCountedStringPool@@
r5apex.exe!0x018934d0 .?AVCCvar@@
r5apex.exe!0x011991e0 .?AVCCvarQuery@@
r5apex.exe!0x011977a0 .?AVCDataCache@@
r5apex.exe!0x01c569a8 .?AVCDebugOverlayPanel@@
r5apex.exe!0x0119cb28 .?AVCDebugTextureInfoDX11@@
r5apex.exe!0x0119de98 .?AVCDefaultAccessor@@
r5apex.exe!0x0119e658 .?AVCDefaultCvarQuery@@
r5apex.exe!0x018b32e0 .?AVCEffectsList@@
r5apex.exe!0x01892f20 .?AVCEmptyConVar@@
r5apex.exe!0x01892f60 .?AVCEmptyConVar@@
r5apex.exe!0x01e7ad70 .?AVCEmptyGameUIConVar@@
r5apex.exe!0x01e7adb0 .?AVCEmptyGameUIConVar@@
r5apex.exe!0x011a70c0 .?AVCEngine@@
r5apex.exe!0x012dc6c0 .?AVCEngineAPI@@
r5apex.exe!0x011980e8 .?AVCEngineClient@@
r5apex.exe!0x02be38d8 .?AVCEngineClient@@
r5apex.exe!0x067c71a8 .?AVCEngineClient@@
r5apex.exe!0x067cd928 .?AVCEngineClient@@
r5apex.exe!0x067f3d28 .?AVCEngineClient@@
r5apex.exe!0x0686aca8 .?AVCEngineClient@@
r5apex.exe!0x06878da8 .?AVCEngineClient@@
r5apex.exe!0x0688a4a8 .?AVCEngineClient@@
r5apex.exe!0x068e2768 .?AVCEngineClient@@
r5apex.exe!0x068fe4e8 .?AVCEngineClient@@
r5apex.exe!0x0119bd58 .?AVCEngineConsoleLoggingListener@@
r5apex.exe!0x01197908 .?AVCEngineTraceClient@@
r5apex.exe!0x011963d8 .?AVCEngineTraceClientDecals@@
r5apex.exe!0x01199d78 .?AVCEngineUniformRandomStream@@
r5apex.exe!0x011a67a0 .?AVCEngineVGui@@
r5apex.exe!0x012a50f0 .?AVCEntityReadInfo@@
r5apex.exe!0x01893640 .?AVCEventSystem@@
r5apex.exe!0x0126dd18 .?AVCExampleEffect@@
r5apex.exe!0x01c5bff0 .?AVCFPS@@
r5apex.exe!0x058ea080 .?AVCFileSystem_Stdio@@
r5apex.exe!0x058ea088 .?AVCFileSystem_Stdio@@
r5apex.exe!0x01e97e10 .?AVCFloatProperty@@
r5apex.exe!0x011a4ab8 .?AVCGameClientExports@@
r5apex.exe!0x01e7abd0 .?AVCGameUI@@
r5apex.exe!0x0119baa8 .?AVCGameUIFuncs@@
r5apex.exe!0x01e97e18 .?AVCHFontProperty@@
r5apex.exe!0x0189f570 .?AVCHLClient@@
r5apex.exe!0x02be3800 .?AVCHLClient@@
r5apex.exe!0x0126dfc8 .?AVCHudTextMessage@@
r5apex.exe!0x0126de08 .?AVCHudTextureHandleProperty@@
r5apex.exe!0x01197a50 .?AVCIVDebugOverlay@@
r5apex.exe!0x01c52250 .?AVCInput@@
r5apex.exe!0x012e0ef0 .?AVCInputStackSystem@@
r5apex.exe!0x012e0fc0 .?AVCInputSystem@@
r5apex.exe!0x0789c6d0 .?AVCInputWin32@@
r5apex.exe!0x01e97d68 .?AVCIntProperty@@
r5apex.exe!0x018b6b90 .?AVCKeyBindingListenerMgr@@
r5apex.exe!0x01894e30 .?AVCKeyValuesSystem@@
r5apex.exe!0x0119cf88 .?AVCLauncherLoggingListener@@
r5apex.exe!0x0119de28 .?AVCListOps@TSListTests@@
r5apex.exe!0x01c5dca8 .?AVCLoadingDisc@@
r5apex.exe!0x012e2be0 .?AVCLocalize@@
r5apex.exe!0x01e94ef0 .?AVCMDLCache@@
r5apex.exe!0x058eca80 .?AVCMatQueuedRenderContext@@
r5apex.exe!0x058eca88 .?AVCMatQueuedRenderContext@@
r5apex.exe!0x058ecc50 .?AVCMatRenderContext@@
r5apex.exe!0x058ecc58 .?AVCMatRenderContext@@
r5apex.exe!0x078a5570 .?AVCMatSystemSurface@@
r5apex.exe!0x078a5578 .?AVCMatSystemSurface@@
r5apex.exe!0x078a5580 .?AVCMatSystemSurface@@
r5apex.exe!0x058ec800 .?AVCMaterialSystem@@
r5apex.exe!0x01c5be00 .?AVCMessageChars@@
r5apex.exe!0x00ebfc50 .?AVCMessageListener@vgui@@
r5apex.exe!0x01198da8 .?AVCModelInfoClient@@
r5apex.exe!0x01198eb8 .?AVCModelInfoServer@@
r5apex.exe!0x011a1920 .?AVCModelLoader@@
r5apex.exe!0x011a5d80 .?AVCModelRender@@
r5apex.exe!0x01c62880 .?AVCModelRenderSystem@@
r5apex.exe!0x01c62898 .?AVCModelRenderSystem@@
r5apex.exe!0x01c60050 .?AVCMoveHelperClient@@
r5apex.exe!0x01198c18 .?AVCNetworkStringTableContainer@@
r5apex.exe!0x011a48a0 .?AVCNetworkStringTableContainer@@
r5apex.exe!0x01e9fc70 .?AVCPanelMetaClassMgrImp@@
r5apex.exe!0x01ea01a0 .?AVCParticleMgr@@
r5apex.exe!0x0119e5e8 .?AVCPhysicsCollision@@
r5apex.exe!0x01893060 .?AVCPhysicsInterface@@
r5apex.exe!0x064089f8 .?AVCPhysicsSurfaceProps@@
r5apex.exe!0x0126e0d8 .?AVCPhysicsSystem@@
r5apex.exe!0x01c69030 .?AVCPickupList@@
r5apex.exe!0x011a31b8 .?AVCPixelVisibilitySystem@@
r5apex.exe!0x01caeb60 .?AVCPoseDebuggerImpl@@
r5apex.exe!0x0119a7f8 .?AVCPrecacheSystem@@
r5apex.exe!0x01e79b80 .?AVCPrediction@@
r5apex.exe!0x01894fa0 .?AVCProcessUtils@@
r5apex.exe!0x01e97718 .?AVCProportionalFloatProperty@@
r5apex.exe!0x01e97e68 .?AVCProportionalIntProperty@@
r5apex.exe!0x01e989e0 .?AVCProportionalXPosProperty@@
r5apex.exe!0x01e97d40 .?AVCProportionalYPosProperty@@
r5apex.exe!0x0119ddb8 .?AVCQueueOps@TSListTests@@
r5apex.exe!0x011a4440 .?AVCQueuedPacketSender@@
r5apex.exe!0x01cabb50 .?AVCRagdollLRURetirement@@
r5apex.exe!0x0119b8f8 .?AVCRegistry@@
r5apex.exe!0x0188f360 .?AVCResListGenerator@@
r5apex.exe!0x011a8fd8 .?AVCRopeInitializer@@
r5apex.exe!0x01cacf08 .?AVCRunGameEngine@@
r5apex.exe!0x01199578 .?AVCSaveRestoreFileSystemPassthrough@@
r5apex.exe!0x01e98e10 .?AVCSchemeManager@@
r5apex.exe!0x0126de78 .?AVCScreenSpaceEffectManager@@
r5apex.exe!0x011963c0 .?AVCScriptLib@@
r5apex.exe!0x04f7bdc0 .?AVCServer@@
r5apex.exe!0x012ef1c0 .?AVCShaderSystem@@
r5apex.exe!0x01195808 .?AVCSimpleLoggingListener@@
r5apex.exe!0x0b2319e8 .?AVCSimpleLoggingListener@@
r5apex.exe!0x01195818 .?AVCSimpleWindowsLoggingListener@@
r5apex.exe!0x01c9fdb0 .?AVCSolidSetDefaults@@
r5apex.exe!0x0126f840 .?AVCSplitScreen@@
r5apex.exe!0x011aa840 .?AVCStaticPropMgr@@
r5apex.exe!0x011aa848 .?AVCStaticPropMgr@@
r5apex.exe!0x081c7680 .?AVCStdMemAlloc@@
r5apex.exe!0x01e97d60 .?AVCStringProperty@@
r5apex.exe!0x0188f6e0 .?AVCStudioRenderContext@@
r5apex.exe!0x01e98c78 .?AVCSurfaceDragDropTarget@@
r5apex.exe!0x01e98e70 .?AVCSystem@@
r5apex.exe!0x01c489a0 .?AVCTempEnts@@
r5apex.exe!0x01e99280 .?AVCTextureDictionary@@
r5apex.exe!0x01e97e60 .?AVCTextureIdProperty@@
r5apex.exe!0x01e93898 .?AVCTraceFilterPhysicsTunnel_Client@@
r5apex.exe!0x01198e18 .?AVCUniformRandomStream@@
r5apex.exe!0x0119e748 .?AVCUniformRandomStream@@
r5apex.exe!0x01890bd0 .?AVCUtlCStringConversion@@
r5apex.exe!0x01891df0 .?AVCUtlNoEscConversion@@
r5apex.exe!0x01199bb8 .?AVCVEfx@@
r5apex.exe!0x0119b4b8 .?AVCVEngineServer@@
r5apex.exe!0x0789c540 .?AVCVGui@@
r5apex.exe!0x011996c8 .?AVCVRenderView@@
r5apex.exe!0x01c5f1a0 .?AVCVScriptGameSystem@@
r5apex.exe!0x01c5c770 .?AVCViewEffects@@
r5apex.exe!0x01c623e0 .?AVCViewEffects@@
r5apex.exe!0x01cb0700 .?AVCViewRender@@
r5apex.exe!0x0126df58 .?AVCViewportClientSystem@@
r5apex.exe!0x01199de8 .?AVCVoiceServer@@
r5apex.exe!0x0789c340 .?AVCWin32Surface@@
r5apex.exe!0x0789c348 .?AVCWin32Surface@@
r5apex.exe!0x0789c350 .?AVCWin32Surface@@
r5apex.exe!0x0119ec48 .?AVC_BaseAnimatingGameSystem@@
r5apex.exe!0x01ca33a0 .?AVC_DataObjectAccessSystem@@
r5apex.exe!0x01e96c68 .?AVC_DefaultParticleSystemQuery@@
r5apex.exe!0x012a8828 .?AVC_DirtySpatialPartitionEntityList@@
r5apex.exe!0x01c684b0 .?AVC_GameMovement@@
r5apex.exe!0x01c81f10 .?AVC_GameRules@@
r5apex.exe!0x0675e290 .?AVC_GameStringPool@@
r5apex.exe!0x01c57cb8 .?AVC_GameTimescale@@
r5apex.exe!0x01c95b60 .?AVC_ParticleSystemQuery@@
r5apex.exe!0x01ca8648 .?AVC_PrecacheHandler@@
r5apex.exe!0x01ca3aa8 .?AVC_PrecacheRegister@@
r5apex.exe!0x01ca95d0 .?AVC_PropData@@
r5apex.exe!0x01ca8610 .?AVC_PropSurvivalList@@
r5apex.exe!0x01c51c80 .?AVC_SoundscapeSystem@@
r5apex.exe!0x01c4c7b0 .?AVC_TEBreakModel@@
r5apex.exe!0x01c4c7c0 .?AVC_TEBreakModel@@
r5apex.exe!0x01c48ac0 .?AVC_TEEffectDispatch@@
r5apex.exe!0x01c48ad0 .?AVC_TEEffectDispatch@@
r5apex.exe!0x01c48a20 .?AVC_TEExplosion@@
r5apex.exe!0x01c48a30 .?AVC_TEExplosion@@
r5apex.exe!0x011a5ca8 .?AVC_TEGibEvent@@
r5apex.exe!0x011a5cb8 .?AVC_TEGibEvent@@
r5apex.exe!0x01c457e0 .?AVC_TEPhysicsProp@@
r5apex.exe!0x01c457f0 .?AVC_TEPhysicsProp@@
r5apex.exe!0x01e882f0 .?AVC_TEProjectileTrail@@
r5apex.exe!0x01e88300 .?AVC_TEProjectileTrail@@
r5apex.exe!0x01c94930 .?AVC_TEScriptParticleSystem@@
r5apex.exe!0x01c94940 .?AVC_TEScriptParticleSystem@@
r5apex.exe!0x01c67680 .?AVC_TEScriptParticleSystemOnEntity@@
r5apex.exe!0x01c67690 .?AVC_TEScriptParticleSystemOnEntity@@
r5apex.exe!0x01c99560 .?AVC_TEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x01c99570 .?AVC_TEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x01c508f0 .?AVC_TEShatterSurface@@
r5apex.exe!0x01c50900 .?AVC_TEShatterSurface@@
r5apex.exe!0x01c4d940 .?AVC_TESoundDispatch@@
r5apex.exe!0x01c4d950 .?AVC_TESoundDispatch@@
r5apex.exe!0x011aa018 .?AVC_TempEntsSystem@@
r5apex.exe!0x01c82250 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01c82280 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01c822b0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01c822e0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01c82310 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01c82340 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01c82370 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01c823a0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01e86330 .?AVC_TurretList@@
r5apex.exe!0x01e8d620 .?AVC_WeaponXList@@
r5apex.exe!0x01ec4960 .?AVChannel@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5530 .?AVChannelMembershipChangeV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4990 .?AVChannelMessage@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4a40 .?AVChannelMuteList@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5040 .?AVChannelMuteListV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec50b0 .?AVChannelV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5580 .?AVChatChannelUpdateV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec55b0 .?AVChatChannelsRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec55d0 .?AVChatChannelsRequestV2@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5600 .?AVChatChannelsV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5650 .?AVChatConnectedV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5678 .?AVChatDisconnectedV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec50f8 .?AVChatInitiateSuccessV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec56a0 .?AVChatInitiateV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec56e8 .?AVChatLeaveV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5708 .?AVChatLeftV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5da8 .?AVChatMembersRequestErrorV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5730 .?AVChatMembersRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5770 .?AVChatMembersV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5dc8 .?AVChatTypingEventRequestErrorV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec57c8 .?AVChatTypingEventRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5830 .?AVChatTypingEventV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec57f8 .?AVChatUserMutedV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5870 .?AVChatUserUnmutedV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01287fb8 .?AVClientDataBlockReceiver@@
r5apex.exe!0x018b1530 .?AVClientModeFullscreen@@
r5apex.exe!0x01ec4a80 .?AVCommunication@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4460 .?AVCommunication@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4490 .?AVCommunicationV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec53a0 .?AVCustomMessage@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x00e87e10 .?AVDNameStatusNode@@
r5apex.exe!0x00e87e20 .?AVDNameStatusNode@@
r5apex.exe!0x00e87e30 .?AVDNameStatusNode@@
r5apex.exe!0x00e87e40 .?AVDNameStatusNode@@
r5apex.exe!0x01185d80 .?AVDenuvoTrialV2@@
r5apex.exe!0x01edaab0 .?AVDescriptorProto@protobuf@google@@
r5apex.exe!0x01edab98 .?AVDescriptorProto_ExtensionRange@protobuf@google@@
r5apex.exe!0x01edabc0 .?AVDescriptorProto_ReservedRange@protobuf@google@@
r5apex.exe!0x01edabe0 .?AVEnumDescriptorProto@protobuf@google@@
r5apex.exe!0x01edac50 .?AVEnumDescriptorProto_EnumReservedRange@protobuf@google@@
r5apex.exe!0x01edac70 .?AVEnumOptions@protobuf@google@@
r5apex.exe!0x01edacc0 .?AVEnumValueDescriptorProto@protobuf@google@@
r5apex.exe!0x01edacf0 .?AVEnumValueOptions@protobuf@google@@
r5apex.exe!0x01ec5e10 .?AVErrorV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01edad40 .?AVExtensionRangeOptions@protobuf@google@@
r5apex.exe!0x01ec58a8 .?AVFetchStickyMessagesRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01edadb0 .?AVFieldDescriptorProto@protobuf@google@@
r5apex.exe!0x01edae30 .?AVFieldOptions@protobuf@google@@
r5apex.exe!0x01ec5de8 .?AVFieldViolationV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01edae90 .?AVFileDescriptorProto@protobuf@google@@
r5apex.exe!0x01edaf68 .?AVFileDescriptorSet@protobuf@google@@
r5apex.exe!0x01edafa0 .?AVFileOptions@protobuf@google@@
r5apex.exe!0x01edb048 .?AVGeneratedCodeInfo@protobuf@google@@
r5apex.exe!0x01edb080 .?AVGeneratedCodeInfo_Annotation@protobuf@google@@
r5apex.exe!0x01ec6410 .?AVGetPreferenceRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6428 .?AVGetPreferenceResponseV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6390 .?AVGetRolesRequest@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec63a8 .?AVGetRolesResponse@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4b20 .?AVGroupMembershipChange@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x0188d420 .?AVHardwareConfigDX11@@
r5apex.exe!0x01ec4b80 .?AVHeader@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5118 .?AVHeartbeatV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4bb0 .?AVHistoryRequest@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4bf0 .?AVHistoryResponse@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x011aa818 .?AVIPredictionSystem_Client@@
r5apex.exe!0x012df620 .?AVImeTextStore@@
r5apex.exe!0x012df628 .?AVImeTextStore@@
r5apex.exe!0x012df630 .?AVImeTextStore@@
r5apex.exe!0x012df638 .?AVImeTextStore@@
r5apex.exe!0x012df640 .?AVImeTextStore@@
r5apex.exe!0x012df648 .?AVImeTextStore@@
r5apex.exe!0x012df650 .?AVImeTextStore@@
r5apex.exe!0x012df658 .?AVImeTextStore@@
r5apex.exe!0x01ec61d0 .?AVInvitation@protocol@respawn@ea@com@@
r5apex.exe!0x01ec5e70 .?AVLoginErrorV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4c40 .?AVLoginRequest@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4640 .?AVLoginRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4668 .?AVLoginRequestV2@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec46a0 .?AVLoginRequestV3@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4c70 .?AVLoginResponse@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5130 .?AVLoginV2Success@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5150 .?AVLoginV3Response@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4cd0 .?AVLogoutRequest@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x011acaf8 .?AVMapSettingsReseter@@
r5apex.exe!0x01edb0c0 .?AVMessageOptions@protobuf@google@@
r5apex.exe!0x01edb110 .?AVMethodDescriptorProto@protobuf@google@@
r5apex.exe!0x01edb150 .?AVMethodOptions@protobuf@google@@
r5apex.exe!0x01e9c748 .?AVMonitorDefaultChanges@@
r5apex.exe!0x01ec58c8 .?AVMuteUserV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5188 .?AVMutedSetV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4ce8 .?AVMutedUser@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4d18 .?AVMutedUserV1@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5458 .?AVNotificationV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01edad88 .?AVOneofDescriptorProto@protobuf@google@@
r5apex.exe!0x01edb1a0 .?AVOneofOptions@protobuf@google@@
r5apex.exe!0x01ec51b0 .?AVPersonaV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4d50 .?AVPlayer@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6208 .?AVPlayer@protocol@respawn@ea@com@@
r5apex.exe!0x01ec6308 .?AVPlayer@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6330 .?AVPlayerInfo@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec53d0 .?AVPointToPointMessageV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5f50 .?AVPresenceSubscribeV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5f98 .?AVPresenceSubscriptionErrorV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5fd0 .?AVPresenceUnsubscribeV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6018 .?AVPresenceUpdateErrorV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6040 .?AVPresenceUpdateV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6090 .?AVPresenceV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec58f8 .?AVPromoteStickyMessageRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4d78 .?AVPublishBinaryRequest@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4da0 .?AVPublishResponse@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4dd0 .?AVPublishTextRequest@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec64f0 .?AVRateLimitConfigV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec46f0 .?AVReconnectRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5930 .?AVRemoveStickyMessageRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec60f0 .?AVRichPresenceV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x012a8b70 .?AVSVC_UserMessage@@
r5apex.exe!0x01ec6518 .?AVServerConfigRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6530 .?AVServerConfigV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01edb1f0 .?AVServiceDescriptorProto@protobuf@google@@
r5apex.exe!0x01edb230 .?AVServiceOptions@protobuf@google@@
r5apex.exe!0x01ec51e0 .?AVSessionCleanupV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5480 .?AVSessionNotificationV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5200 .?AVSessionRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5218 .?AVSessionResponseV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5248 .?AVSessionV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01edb280 .?AVSourceCodeInfo@protobuf@google@@
r5apex.exe!0x01edb2b0 .?AVSourceCodeInfo_Location@protobuf@google@@
r5apex.exe!0x01ec5958 .?AVStickyMessageChangedV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5980 .?AVStickyMessageResponseV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5280 .?AVStickyMessageV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4e00 .?AVSubscribeRequest@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4e20 .?AVSubscribeResponse@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec52d0 .?AVSuccessV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4e70 .?AVTextMessage@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5310 .?AVTextMessageV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6448 .?AVTranslationPreferenceV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01edb320 .?AVUninterpretedOption@protobuf@google@@
r5apex.exe!0x01edae08 .?AVUninterpretedOption_NamePart@protobuf@google@@
r5apex.exe!0x01ec59b8 .?AVUnmuteUserV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4eb8 .?AVUnsubscribeRequest@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec4ed8 .?AVUnsubscribeResponse@protocol@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6470 .?AVUpdatePreferenceRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec54b8 .?AVUserMembershipChangeV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01e990e8 .?AVVPanelWrapper@@
r5apex.exe!0x01ec5ea0 .?AVValidationErrorV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec59e8 .?AVWorldChatAssignV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5a10 .?AVWorldChatChannelsRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5a50 .?AVWorldChatChannelsResponseV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5a38 .?AVWorldChatConfigurationRequestV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5a90 .?AVWorldChatConfigurationResponseV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5ac0 .?AVWorldChatMembershipChangeV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5b00 .?AVWorldChatResponseV1@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec5b60 .?AVWorldChatShard@protocol@rtm@antelope@eadp@ea@com@@
r5apex.exe!0x01ec6230 .?AVeadpRespawnPresence@protocol@respawn@ea@com@@
r5apex.exe!0x01ec6280 .?AVp2pPayload@protocol@respawn@ea@com@@
r5apex.exe!0x01ec4388 .?AVstl_critical_section_win7@details@Concurrency@@
r5apex.exe!0x041b15a8 .?AVstl_critical_section_win7@details@Concurrency@@
r5apex.exe!0x041b2608 .?AVstl_critical_section_win7@details@Concurrency@@
```

