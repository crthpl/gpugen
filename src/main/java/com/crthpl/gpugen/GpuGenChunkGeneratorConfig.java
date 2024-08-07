//package com.crthpl.gpugen;
//
//import com.mojang.serialization.Codec;
//import com.mojang.serialization.codecs.RecordCodecBuilder;
//import net.minecraft.registry.RegistryCodecs;
//import net.minecraft.registry.RegistryKeys;
//import net.minecraft.registry.RegistryOps;
//import net.minecraft.world.biome.Biome;
//import net.minecraft.world.biome.BiomeKeys;
//import net.minecraft.world.gen.chunk.FlatChunkGeneratorConfig;
//import net.minecraft.world.gen.chunk.FlatChunkGeneratorLayer;
//import net.minecraft.world.gen.feature.MiscPlacedFeatures;
//
//import java.util.Optional;
//import java.util.function.Function;
//
//public class GpuGenChunkGeneratorConfig {
//    public static final Codec<FlatChunkGeneratorConfig> CODEC = RecordCodecBuilder.create(
//                    instance -> instance.group(
//                                    RegistryCodecs.entryList(RegistryKeys.STRUCTURE_SET).lenientOptionalFieldOf("structure_overrides").forGetter(config -> config.structureOverrides),
//                                    FlatChunkGeneratorLayer.CODEC.listOf().fieldOf("layers").forGetter(FlatChunkGeneratorConfig::getLayers),
//                                    Codec.BOOL.fieldOf("lakes").orElse(false).forGetter(config -> config.hasLakes),
//                                    Codec.BOOL.fieldOf("features").orElse(false).forGetter(config -> config.hasFeatures),
//                                    Biome.REGISTRY_CODEC.lenientOptionalFieldOf("biome").orElseGet(Optional::empty).forGetter(config -> Optional.of(config.biome)),
//                                    RegistryOps.getEntryCodec(BiomeKeys.PLAINS),
//                                    RegistryOps.getEntryCodec(MiscPlacedFeatures.LAKE_LAVA_UNDERGROUND),
//                                    RegistryOps.getEntryCodec(MiscPlacedFeatures.LAKE_LAVA_SURFACE)
//                            )
//                            .apply(instance, FlatChunkGeneratorConfig::new)
//            )
//            .<FlatChunkGeneratorConfig>comapFlatMap(FlatChunkGeneratorConfig::checkHeight, Function.identity())
//            .stable();
//}
