package com.crthpl.gpugen.mixin;

import com.crthpl.gpugen.GpuGen;
import com.crthpl.gpugen.GpuGenChunkGenerator;
import net.fabricmc.fabric.api.biome.v1.NetherBiomes;
import net.minecraft.registry.*;
import net.minecraft.registry.entry.RegistryEntry;
import net.minecraft.util.Identifier;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.biome.BiomeKeys;
import net.minecraft.world.biome.BuiltinBiomes;
import net.minecraft.world.dimension.DimensionOptions;
import net.minecraft.world.gen.WorldPreset;
import net.minecraft.world.gen.WorldPresets;
import net.minecraft.world.gen.chunk.ChunkGenerator;
import net.minecraft.world.gen.chunk.FlatChunkGenerator;
import net.minecraft.world.gen.chunk.FlatChunkGeneratorConfig;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

import java.util.Optional;

@Mixin(WorldPresets.Registrar.class)
public abstract class WorldPresetsMixin {

	@Shadow protected abstract void register(RegistryKey<WorldPreset> key, DimensionOptions dimensionOptions);
	@Shadow protected abstract DimensionOptions createOverworldOptions(ChunkGenerator chunkGenerator);
	@Shadow protected  RegistryEntryLookup<Biome> biomeLookup;

	@Inject(at = @At("RETURN"), method = "Lnet/minecraft/world/gen/WorldPresets$Registrar;bootstrap()V")
	private void init(CallbackInfo info) {
//		Registries.REGISTRIES.get(RegistryKeys
		this.register(
				GpuGen.OVERWORL,
				this.createOverworldOptions(
						new GpuGenChunkGenerator(biomeLookup.getOrThrow(BiomeKeys.PLAINS))
				)
		);
		// This code is injected into the start of MinecraftServer.loadWorld()V
	}
}