package com.crthpl.gpugen;

import net.fabricmc.api.ModInitializer;

import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.registry.RegistryKey;
import net.minecraft.registry.RegistryKeys;
import net.minecraft.util.Identifier;
import net.minecraft.world.dimension.DimensionOptions;
import net.minecraft.world.gen.WorldPreset;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;

public class GpuGen implements ModInitializer {
    // This logger is used to write text to the console and the log file.
    // It is considered best practice to use your mod id as the logger's name.
    // -----
    public static final Logger LOGGER = LoggerFactory.getLogger("gpugen");
    public static final String MOD_ID = "gpugen";
    public static final RegistryKey<WorldPreset> OVERWORL = RegistryKey.of(RegistryKeys.WORLD_PRESET, Identifier.of(GpuGen.MOD_ID, "overworl"));

    @Override
    public void onInitialize() {
        Registry.register(Registries.CHUNK_GENERATOR, Identifier.of(MOD_ID, "overworl"), GpuGenChunkGenerator.CODEC);

        // This code runs as soon as Minecraft is in a mod-load-ready state.
        // However, some things (like resources) may still be uninitialized.
        // Proceed with mild caution.OVERWORLD
//        ChunkGeneratorSettings OverworldSettings = (ChunkGeneratorSettings)Registries.REGISTRIES.get(RegistryKeys.CHUNK_GENERATOR_SETTINGS.getRegistry()).get(ChunkGeneratorSettings.OVERWORLD.getRegistry());
//        DimensionOptions overworldOptions = (DimensionOptions)Registries.REGISTRIES.get(RegistryKeys.DIMENSION.getRegistry()).get(DimensionOptions.OVERWORLD.getRegistry());
//        WorldPreset overworldPreset =  new WorldPreset(
//                Map.of(DimensionOptions.OVERWORLD, overworldOptions)
//        );
////          , Identifier.of(MOD_ID, "overworl"), overworldPreset);
//        RegistryKeys.WORLD_PRESET.getRegistryRef()
//        LOGGER.info(Registries.REGISTRIES.get(RegistryKeys.WORLD_PRESET.getRegistry()).toString());
//        LOGGER.info("GPUGen Loaded");
    }
}