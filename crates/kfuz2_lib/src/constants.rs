// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

/// KF1 packages signature byte
pub const KF_SIGNATURE: [u8; 4] = [0xC2, 0x83, 0x2A, 0x9E];
/// Size of compressed chunks - 4 bytes, 0-33096
pub const COMPRESSED_CHUNK_SIZE: usize = 33096;
/// Size of uncompressed chunks - 4 bytes, 0-32768
pub const UNCOMPRESSED_CHUNK_SIZE: usize = 32768;
/// KF1 packages default exensions.
pub const DEFAULT_EXTENSIONS: [&str; 6] = ["u", "utx", "usx", "ukx", "uax", "rom"];
/// KF1 compressed file extension.
pub const COMPRESSED_EXTENSION: &str = "uz2";

/// List of vanilla packages, will be omitted from compression
pub const KF_DEFAULT_PACKAGES: [&str; 548] = [
    // animations
    "22patchmesh.ukx",
    "foundry_anim.ukx",
    "frightyard_skm.ukx",
    "gear_anm.ukx",
    "hillbillyhorror_anim.ukx",
    "kfboss.ukx",
    "kfcharactermodels.ukx",
    "kfcharactersb.ukx",
    "kfmapobjects.ukx",
    "kfsoldiers.ukx",
    "kfvehiclemodels.ukx",
    "kfweaponmodels.ukx",
    "kf_freaks2_trip.ukx",
    "kf_freaks2_trip_circus.ukx",
    "kf_freaks2_trip_halloween.ukx",
    "kf_freaks2_trip_xmas.ukx",
    "kf_freaks_trip.ukx",
    "kf_freaks_trip_circus.ukx",
    "kf_freaks_trip_halloween.ukx",
    "kf_freaks_trip_xmas.ukx",
    "kf_gibbs_trip.ukx",
    "kf_ijc_halloween_weps3.ukx",
    "kf_ijc_halloween_weps_2.ukx",
    "kf_ijc_summer_weps1.ukx",
    "kf_mac10mpanims.ukx",
    "kf_rachelc_anim.ukx",
    "kf_ringmaster_trip.ukx",
    "kf_robotdance.ukx",
    "kf_soldier_trip.ukx",
    "kf_weapons2_trip.ukx",
    "kf_weapons3rd2_ijc.ukx",
    "kf_weapons3rd2_trip.ukx",
    "kf_weapons3rd3_ijc.ukx",
    "kf_weapons3rd3_trip.ukx",
    "kf_weapons3rd4_trip.ukx",
    "kf_weapons3rd5_trip.ukx",
    "kf_weapons3rd6_trip.ukx",
    "kf_weapons3rd_ijc.ukx",
    "kf_weapons3rd_trip.ukx",
    "kf_weapons_trip.ukx",
    "kf_wep_benelli.ukx",
    "kf_wep_claymore.ukx",
    "kf_wep_dualrevolver.ukx",
    "kf_wep_dual_mk23.ukx",
    "kf_wep_dwarf_axe.ukx",
    "kf_wep_fal_acog.ukx",
    "kf_wep_huskgun.ukx",
    "kf_wep_kriss.ukx",
    "kf_wep_ksg_shotgun.ukx",
    "kf_wep_m4.ukx",
    "kf_wep_m4m203.ukx",
    "kf_wep_m7a3.ukx",
    "kf_wep_m99_sniper.ukx",
    "kf_wep_medicnade.ukx",
    "kf_wep_mk23.ukx",
    "kf_wep_mkb42.ukx",
    "kf_wep_mp5.ukx",
    "kf_wep_revolver.ukx",
    "kf_wep_trenchgun.ukx",
    "kf_wep_vlad9000.ukx",
    "kf_wep_zedgun.ukx",
    "kf_wp3rdcamo_trip.ukx",
    "kf_wp3rdgold_trip.ukx",
    "patch2anims.ukx",
    "pier_anim.ukx",
    "yahct_anim.ukx",
    // maps
    "entry.rom",
    "kf-abusementpark.rom",
    "kf-aperture.rom",
    "kf-bedlam.rom",
    "kf-biohazard.rom",
    "kf-bioticslab.rom",
    "kf-clandestine.rom",
    "kf-crash.rom",
    "kf-departed.rom",
    "kf-evilsantaslair.rom",
    "kf-farm.rom",
    "kf-filthscross.rom",
    "kf-forgotten.rom",
    "kf-foundry.rom",
    "kf-frightyard.rom",
    "kf-hell.rom",
    "kf-hellride.rom",
    "kf-hillbillyhorror.rom",
    "kf-hospitalhorrors.rom",
    "kf-icebreaker.rom",
    "kf-icecave.rom",
    "kf-manor.rom",
    "kf-menu.rom",
    "kf-moonbase.rom",
    "kf-mountainpass.rom",
    "kf-offices.rom",
    "kf-sirensbelch.rom",
    "kf-steamland.rom",
    "kf-stronghold.rom",
    "kf-suburbia.rom",
    "kf-thrillschills.rom",
    "kf-transit.rom",
    "kf-waterworks.rom",
    "kf-westlondon.rom",
    "kf-wyre.rom",
    "kfintro.rom",
    "kfo-frightyard.rom",
    "kfo-steamland.rom",
    "kfo-transit.rom",
    "nointro.rom",
    // sounds
    "abusementparksnd.uax",
    "amboutside.uax",
    "amb_animals.uax",
    "amb_constructions.uax",
    "amb_constructionstwo.uax",
    "amb_destruction.uax",
    "amb_destruction2.uax",
    "amb_vehicles.uax",
    "amb_weather.uax",
    "amb_weather01.uax",
    "amb_weather02.uax",
    "artillery.uax",
    "freakcircus_snd_two.uax",
    "frightyard_snd.uax",
    "hellride_snd.uax",
    "hillbillyhorrorsnd.uax",
    "inf_player.uax",
    "inf_weapons.uax",
    "inf_weaponstwo.uax",
    "inf_weapons_foley.uax",
    "inf_weapons_foley2.uax",
    "inf_weapons_foley_3rd.uax",
    "kfambientsound.uax",
    "kfpawndamagesound.uax",
    "kfplayersound.uax",
    "kfportal_snd.uax",
    "kfweaponsound.uax",
    "kf_9mmsnd.uax",
    "kf_aa12snd.uax",
    "kf_ak47snd.uax",
    "kf_axesnd.uax",
    "kf_basebloat.uax",
    "kf_basebloat_circus.uax",
    "kf_basebloat_halloween.uax",
    "kf_basebloat_xmas.uax",
    "kf_baseclot.uax",
    "kf_baseclot_circus.uax",
    "kf_baseclot_halloween.uax",
    "kf_baseclot_xmas.uax",
    "kf_basecrawler.uax",
    "kf_basecrawler_circus.uax",
    "kf_basecrawler_halloween.uax",
    "kf_basecrawler_xmas.uax",
    "kf_basefleshpound.uax",
    "kf_basefleshpound_circus.uax",
    "kf_basefleshpound_halloween.uax",
    "kf_basefleshpound_xmas.uax",
    "kf_basegorefast.uax",
    "kf_basegorefast_circus.uax",
    "kf_basegorefast_halloween.uax",
    "kf_basegorefast_xmas.uax",
    "kf_basehusk.uax",
    "kf_basehusk_circus.uax",
    "kf_basehusk_halloween.uax",
    "kf_basehusk_xmas.uax",
    "kf_basepatriarch.uax",
    "kf_basepatriarchtwo.uax",
    "kf_basepatriarch_circus.uax",
    "kf_basepatriarch_halloween.uax",
    "kf_basepatriarch_xmas.uax",
    "kf_basescrake.uax",
    "kf_basescrake_circus.uax",
    "kf_basescrake_halloween.uax",
    "kf_basescrake_xmas.uax",
    "kf_basesiren.uax",
    "kf_basesiren_circus.uax",
    "kf_basesiren_halloween.uax",
    "kf_basesiren_xmas.uax",
    "kf_basestalker.uax",
    "kf_basestalker_circus.uax",
    "kf_basestalker_halloween.uax",
    "kf_basestalker_xmas.uax",
    "kf_bimpactsnd.uax",
    "kf_bullpupsnd.uax",
    "kf_chainsawsnd.uax",
    "kf_chainsawsnd_xmas.uax",
    "kf_claymoresnd.uax",
    "kf_darvoice.uax",
    "kf_doublesgsnd.uax",
    "kf_dwarfaxesnd.uax",
    "kf_enemiesfinalsnd.uax",
    "kf_enemiesfinalsnd_circus.uax",
    "kf_enemiesfinalsnd_halloween.uax",
    "kf_enemiesfinalsnd_xmas.uax",
    "kf_enemyglobalsnd.uax",
    "kf_enemyglobalsndtwo.uax",
    "kf_envambientsnd.uax",
    "kf_envambientsnd2.uax",
    "kf_femalevoiceone.uax",
    "kf_flamethrowersnd.uax",
    "kf_fnfalsnd.uax",
    "kf_foundrysnd.uax",
    "kf_fy_blowerthrowersnd.uax",
    "kf_fy_sealsquealsnd.uax",
    "kf_fy_seekersixsnd.uax",
    "kf_fy_zedv2snd.uax",
    "kf_grenadesnd.uax",
    "kf_handcannonsnd.uax",
    "kf_huskgunsnd.uax",
    "kf_ijc_halloweensnd.uax",
    "kf_indooramb1.uax",
    "kf_interfacesnd.uax",
    "kf_inventorysnd.uax",
    "kf_jumpsnd.uax",
    "kf_katanasnd.uax",
    "kf_knifesnd.uax",
    "kf_krisssnd.uax",
    "kf_ksgsnd.uax",
    "kf_lawsnd.uax",
    "kf_m14ebrsnd.uax",
    "kf_m32snd.uax",
    "kf_m4riflesnd.uax",
    "kf_m4shotgunsnd.uax",
    "kf_m79snd.uax",
    "kf_m7a3snd.uax",
    "kf_m99snd.uax",
    "kf_mac10mpsnd.uax",
    "kf_machetesnd.uax",
    "kf_malevoiceone.uax",
    "kf_malevoicetwo.uax",
    "kf_medicgrenadesnd.uax",
    "kf_menusnd.uax",
    "kf_mk23snd.uax",
    "kf_mkb42snd.uax",
    "kf_mp5snd.uax",
    "kf_mp7snd.uax",
    "kf_mumblevoice.uax",
    "kf_nailshotgun.uax",
    "kf_outdooramb1.uax",
    "kf_pipesnd.uax",
    "kf_playerglobalsnd.uax",
    "kf_playerm1voc.uax",
    "kf_pumpsgsnd.uax",
    "kf_revolversnd.uax",
    "kf_riflesnd.uax",
    "kf_rs_thompsonsnd.uax",
    "kf_scarsnd.uax",
    "kf_shotgundragonsbreathsnd.uax",
    "kf_sp_longmusketsnd.uax",
    "kf_sp_orcasnd.uax",
    "kf_sp_thompsonsnd.uax",
    "kf_sp_zedthrowersnd.uax",
    "kf_swansong_snd.uax",
    "kf_trader.uax",
    "kf_transit_dialog_snd.uax",
    "kf_xbowsnd.uax",
    "kf_zedgunsnd.uax",
    "miscsounds.uax",
    "patchsounds.uax",
    "projectilesounds.uax",
    "romenusounds.uax",
    "steamland_snd.uax",
    "summerboardwalkdialogue.uax",
    "vehicle_engines.uax",
    "vehicle_weapons.uax",
    "woodbreakfx.uax",
    // static meshes
    "22patch.usx",
    "asylum_sm.usx",
    "baksanvalleysm.usx",
    "civilvehicles_sm.usx",
    "debugobjects.usx",
    "departedstatics.usx",
    "detailsm.usx",
    "effectssm.usx",
    "fallenherossm.usx",
    "filthscross_sm.usx",
    "foundry_sm.usx",
    "freakcircus_sm_one.usx",
    "freakcircus_sm_two.usx",
    "frightyard2_sm.usx",
    "frightyard_sm.usx",
    "furnituresm.usx",
    "gkstaticmeshes.usx",
    "hedgehogsm.usx",
    "hellride_sm.usx",
    "hemispheressm.usx",
    "hemispheressm2.usx",
    "hillbillyhorror_sm.usx",
    "icebreaker_sm.usx",
    "ijcweaponpackstaticsw2.usx",
    "industrysm.usx",
    "industrysm2.usx",
    "kesselsm.usx",
    "kfmuzzleflashes.usx",
    "kfportal_sm.usx",
    "kf_enginetest.usx",
    "kf_fx_char_sm.usx",
    "kf_generic_sm.usx",
    "kf_gore_trip_sm.usx",
    "kf_gore_trip_sm_circus.usx",
    "kf_gore_trip_sm_halloween.usx",
    "kf_gore_trip_sm_two.usx",
    "kf_gore_trip_sm_xmas.usx",
    "kf_icetunnel_sm.usx",
    "kf_ijc_halloween_weps.usx",
    "kf_ijc_halloween_weps2.usx",
    "kf_ijc_summer_weps.usx",
    "kf_pickups2_trip.usx",
    "kf_pickups3_trip.usx",
    "kf_pickups4_trip.usx",
    "kf_pickups5_trip.usx",
    "kf_pickups6_trip.usx",
    "kf_pickupscamo_trip.usx",
    "kf_pickupsgold_trip.usx",
    "kf_pickups_trip.usx",
    "kf_sirensbelch_sm.usx",
    "kf_swansong_sm.usx",
    "killingfloorlabstatics.usx",
    "killingfloormanorstatics.usx",
    "killingfloorstatics.usx",
    "konigsplatzsm.usx",
    "krasnyism.usx",
    "krasnyism2.usx",
    "landscapesm.usx",
    "levelspecificsm.usx",
    "levelspecificsm2.usx",
    "menustatics.usx",
    "moonbase_sm.usx",
    "moonbase_sm_two.usx",
    "mountainpass_sm.usx",
    "mrsfoster_dlc_gore_sm.usx",
    "mrsfoster_steampunk_dlc_gore_sm.usx",
    "newpatchsm.usx",
    "officestatics.usx",
    "patchstatics.usx",
    "pier_sm.usx",
    "potato_s.usx",
    "props_objectivemode.usx",
    "religioussm.usx",
    "ruralsm.usx",
    "specialeffectssm.usx",
    "stronghold_sm.usx",
    "vegetationsm.usx",
    "vegetationsm_two.usx",
    "waterworks_sm.usx",
    "workshop_sm.usx",
    "wyrestatics.usx",
    "zed_fx_sm.usx",
    "zed_pieces.usx",
    // u files
    "core.u",
    "editor.u",
    "engine.u",
    "fire.u",
    "frightscript.u",
    "gameplay.u",
    "goodkarma.u",
    "gui2k4.u",
    "ipdrv.u",
    "kfchar.u",
    "kfgui.u",
    "kfmod.u",
    "kfmutators.u",
    "kfstorygame.u",
    "kfstoryui.u",
    "old2k4.u",
    "roeffects.u",
    "roengine.u",
    "rointerface.u",
    "sideshowscript.u",
    "unrealed.u",
    "unrealgame.u",
    "utv2004c.u",
    "utv2004s.u",
    "uweb.u",
    "xadmin.u",
    "xgame.u",
    "xinterface.u",
    "xvoting.u",
    "xwebadmin.u",
    // textures
    "20credits.utx",
    "22chartex.utx",
    "25tex.utx",
    "2k4menus.utx",
    "architecture_t.utx",
    "architecture_t2.utx",
    "architecture_t3.utx",
    "asylum_t.utx",
    "baksanvalley_t.utx",
    "cellexample.utx",
    "characters_tex.utx",
    "civilvehicles_smt.utx",
    "crash_t.utx",
    "danzig_t.utx",
    "danzig_t2.utx",
    "danzig_t3.utx",
    "departedtextures.utx",
    "detailsmt.utx",
    "detailtextures_t.utx",
    "dreamtex.utx",
    "effects_tex.utx",
    "effects_tex_steampunk.utx",
    "fallenheros2_t.utx",
    "filthscross_t.utx",
    "foundry_t.utx",
    "freakcircus_t_one.utx",
    "freakcircus_t_two.utx",
    "frightyard2_t.utx",
    "frightyard_t.utx",
    "furnituresmt.utx",
    "g15lcd.utx",
    "g15lcdfonts.utx",
    "gear_tex.utx",
    "gktextures.utx",
    "hedgehog_t.utx",
    "hellride_t.utx",
    "hemispheres_smt.utx",
    "hemispheres_smt2.utx",
    "hillbillyhorror_t.utx",
    "icebreaker_t.utx",
    "ijcfonts.utx",
    "industrysmt.utx",
    "industrysmt2.utx",
    "interfaceart2_tex.utx",
    "interfaceart_tex.utx",
    "interfacecontent.utx",
    "kessel_t.utx",
    "kfcharacters.utx",
    "kffonts.utx",
    "kfgui.utx",
    "kfinterfacecontent.utx",
    "kfkillmenow.utx",
    "kflevelpreviews.utx",
    "kfmapendtextures.utx",
    "kfmaterials.utx",
    "kfpatch2.utx",
    "kfportal_t.utx",
    "kfportraits.utx",
    "kfstorygame_tex.utx",
    "kfthumbs.utx",
    "kfurbanskin.utx",
    "kfx.utx",
    "kfzed_fx_t.utx",
    "kf_dlc.utx",
    "kf_fx_char_t.utx",
    "kf_fx_trip_t.utx",
    "kf_generic_t.utx",
    "kf_gore_trip_t_two.utx",
    "kf_icetunnel_t.utx",
    "kf_ijc_halloween_weapons.utx",
    "kf_ijc_halloween_weapons2.utx",
    "kf_ijc_halloween_weapons2_3rd.utx",
    "kf_ijc_halloween_weapons_3rd.utx",
    "kf_ijc_hud.utx",
    "kf_ijc_summer_weapons.utx",
    "kf_ijc_summer_weapons_3rd.utx",
    "kf_interfaceart_tex.utx",
    "kf_mac10mptex.utx",
    "kf_rachelc_mat.utx",
    "kf_sirensbelch_t.utx",
    "kf_soldier11_trip_t.utx",
    "kf_soldier12_trip_t.utx",
    "kf_soldier13_trip_t.utx",
    "kf_soldier2_trip_t.utx",
    "kf_soldier3_trip_t.utx",
    "kf_soldier4_trip_t.utx",
    "kf_soldier5_trip_t.utx",
    "kf_soldier6_trip_t.utx",
    "kf_soldiermagmaneon_trip_t.utx",
    "kf_soldier_trip_t.utx",
    "kf_solider10_trip_t.utx",
    "kf_solider7_trip_t.utx",
    "kf_solider8_trip_t.utx",
    "kf_solider9_trip_t.utx",
    "kf_specimens_trip_circus_t.utx",
    "kf_specimens_trip_halloween_t.utx",
    "kf_specimens_trip_t.utx",
    "kf_specimens_trip_t_two.utx",
    "kf_specimens_trip_xmas_t.utx",
    "kf_specimens_trip_xmas_t_two.utx",
    "kf_swansong_tex.utx",
    "kf_weapons2_trip_t.utx",
    "kf_weapons3rd2_trip_t.utx",
    "kf_weapons3rd3_trip_t.utx",
    "kf_weapons3rd4_trip_t.utx",
    "kf_weapons3rd5_trip_t.utx",
    "kf_weapons3rd6_trip_t.utx",
    "kf_weapons3rd_camo_trip_t.utx",
    "kf_weapons3rd_gold_t.utx",
    "kf_weapons3rd_trip_t.utx",
    "kf_weapons3_trip_t.utx",
    "kf_weapons4_trip_t.utx",
    "kf_weapons5_scopes_trip_t.utx",
    "kf_weapons5_trip_t.utx",
    "kf_weapons6_trip_t.utx",
    "kf_weapons7_trip_t.utx",
    "kf_weapons8_trip_t.utx",
    "kf_weapons9_trip_t.utx",
    "kf_weapons_camo_trip_t.utx",
    "kf_weapons_gold_t.utx",
    "kf_weapons_neon_trip_t.utx",
    "kf_weapons_trip_t.utx",
    "killingfloor2hud.utx",
    "killingfloorhud.utx",
    "killingfloorhud_halloween.utx",
    "killingfloorhud_summer.utx",
    "killingfloorhud_xmas.utx",
    "killingfloorlabtextures.utx",
    "killingfloormanortextures.utx",
    "killingfloorofficetextures.utx",
    "killingfloortextures.utx",
    "killingfloorweapons.utx",
    "konigsplatz_t.utx",
    "krasnyioktyabr_t.utx",
    "krasnyioktyabr_t2.utx",
    "krasnyi_cubemaps.utx",
    "landscapesmt.utx",
    "landscape_t.utx",
    "levelspecificsmt.utx",
    "levelspecificsmt2.utx",
    "menubackground.utx",
    "militaryaxissmt.utx",
    "moonbase_t.utx",
    "moonbase_t_two.utx",
    "mountainpass_t.utx",
    "mrsfoster_dlc_1stp.utx",
    "mrsfoster_dlc_3rdp.utx",
    "mrsfoster_steampunk_dlc_1stp.utx",
    "mrsfoster_steampunk_dlc_3rdp.utx",
    "objectivemode_t.utx",
    "patchtex.utx",
    "pier_t.utx",
    "pitomnik_t.utx",
    "placeholder_t.utx",
    "potato_t.utx",
    "religioussmt.utx",
    "ringmaster_lockheart_t.utx",
    "rofonts.utx",
    "rofontstwo.utx",
    "rofonts_rus.utx",
    "rointerfaceart.utx",
    "rsdlc_1stp.utx",
    "rsdlc_3rdp.utx",
    "ruralsmt.utx",
    "scopeshaders.utx",
    "specialeffects.utx",
    "stronghold_t.utx",
    "vegetationsmt.utx",
    "vegetationsmt_two.utx",
    "waterworks_t.utx",
    "weapons1st_tex.utx",
    "workshop_t.utx",
    "wyretex.utx",
];