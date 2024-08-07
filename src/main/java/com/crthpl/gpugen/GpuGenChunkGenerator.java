package com.crthpl.gpugen;

import com.crthpl.gpugen.rust.generate_chunk$callback;
import com.crthpl.gpugen.rust.lib_h;
import com.mojang.serialization.MapCodec;
import com.mojang.serialization.codecs.RecordCodecBuilder;
import net.minecraft.block.BlockState;
import net.minecraft.block.Blocks;
import net.minecraft.registry.DynamicRegistryManager;
import net.minecraft.registry.entry.RegistryEntry;
import net.minecraft.structure.StructureTemplateManager;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.ChunkPos;
import net.minecraft.world.ChunkRegion;
import net.minecraft.world.HeightLimitView;
import net.minecraft.world.Heightmap;
import net.minecraft.world.StructureWorldAccess;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.biome.source.BiomeAccess;
import net.minecraft.world.biome.source.FixedBiomeSource;
import net.minecraft.world.chunk.Chunk;
import net.minecraft.world.gen.GenerationStep;
import net.minecraft.world.gen.StructureAccessor;
import net.minecraft.world.gen.chunk.*;
import net.minecraft.world.gen.chunk.placement.StructurePlacementCalculator;
import net.minecraft.world.gen.noise.NoiseConfig;

import java.lang.foreign.Arena;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.List;
import java.util.concurrent.CompletableFuture;

public class GpuGenChunkGenerator extends ChunkGenerator {
	public static final MapCodec<GpuGenChunkGenerator> CODEC = RecordCodecBuilder.mapCodec(
			instance -> instance
					.group(Biome.REGISTRY_CODEC.fieldOf("biome").forGetter(config -> config.biome))
					.apply(instance, instance.stable(GpuGenChunkGenerator::new))
			// FlatChunkGeneratorConfig.CODEC.fieldOf("settings").forGetter(FlatChunkGenerator::getConfig)
	);

	private final RegistryEntry<Biome> biome;
	private MemorySegment generator = MemorySegment.ofArray(new int[500]);

	/*
	 * you can add whatever fields you want to this constructor, as long as they're
	 * added to the codec as well
	 */
	public GpuGenChunkGenerator(RegistryEntry<Biome> biome) {
		super(new FixedBiomeSource(biome));
		GpuGen.LOGGER.info("java.library.path is " + System.getProperty("java.library.path"));
		this.generator = lib_h.new_generator(0, 16, 8);
		// print address in hex
		GpuGen.LOGGER.info("generator address is " + Long.toHexString(generator.address()));
		this.biome = biome;
	}

	/*
	 * the method that creates non-noise caves (x.e., all the caves we had before
	 * the caves and cliffs update)
	 */
	@Override
	public void carve(ChunkRegion chunkRegion, long seed, NoiseConfig noiseConfig, BiomeAccess biomeAccess,
					  StructureAccessor structureAccessor, Chunk chunk, GenerationStep.Carver carverStep) {
	}

	/*
	 * the method that places grass, dirt, and other things on top of the world, as
	 * well as handling the bedrock and deepslate layers,
	 * as well as a few other miscellaneous things. without this method, your world
	 * is just a blank stone (or whatever your default block is) canvas (plus any
	 * ores, etc)
	 */
	@Override
	public void buildSurface(ChunkRegion region, StructureAccessor structures, NoiseConfig noiseConfig,
							 Chunk chunk) {

	}
	/*
	 * the method that paints biomes on top of the already-generated terrain. if you
	 * leave this method alone, the entire world will be a River biome.
	 * note that this does not mean that the world will all be water; but drowned
	 * and salmon will spawn.
	 */
	// @Override
	// public CompletableFuture<Chunk> populateBiomes(Registry<Biome> biomeRegistry,
	// Executor executor, NoiseConfig noiseConfig, Blender blender,
	// StructureAccessor structureAccessor, Chunk chunk) {
	// return super.populateBiomes(noiseConfig, blender, structureAccessor, chunk);
	// }

	/* this method spawns entities in the world */
	@Override
	public void populateEntities(ChunkRegion region) {
	}

	/*
	 * the distance between the highest and lowest points in the world. in vanilla,
	 * this is 384 (64+320)
	 */
	@Override
	public int getWorldHeight() {
		return lib_h.get_height(generator);
	}

	/*
	 * this method builds the shape of the terrain. it places stone everywhere,
	 * which will later be overwritten with grass, terracotta, snow, sand, etc
	 * by the buildSurface method. it also is responsible for putting the water in
	 * oceans. it returns a CompletableFuture-- you'll likely want this to be
	 * delegated to worker threads.
	 */
	@Override
	public CompletableFuture<Chunk> populateNoise(Blender blender, NoiseConfig noiseConfig,
												  StructureAccessor structureAccessor, Chunk chunk) {
		CompletableFuture<Chunk> chunkFuture = new CompletableFuture<Chunk>();
		ChunkPos pos = chunk.getPos();
		int height = getWorldHeight();
		Arena arena = Arena.global();
		MemorySegment cb = generate_chunk$callback.allocate(
				(completedChunk) -> {
					GpuGen.LOGGER.info("ABCDEF is " + Long.toHexString(generator.address()));
					int[] blocks = com.crthpl.gpugen.rust.Chunk.blocks$get(completedChunk).asSlice(0, 4* 16*16*getWorldHeight()).toArray(ValueLayout.JAVA_INT);
					for (int x = 0; x < 16; x++) {
						for (int z = 0; z < 16; z++) {
							for (int y = 0; y < height; y++) {
								int block = blocks[x + z * 16 + y * 16 * height];
								BlockState blockState = block == 0 ?
									Blocks.AIR.getDefaultState() :Blocks.STONE.getDefaultState();

								chunk.setBlockState(new BlockPos(x + (pos.x << 4), y, z + (pos.z << 4)), blockState, false);
							}
						}
					}
					chunkFuture.complete(chunk);
				}, arena);
		GpuGen.LOGGER.info("2.generator address is " + Long.toHexString(generator.address()));
		lib_h.generate_chunk(generator, cb, pos.x, pos.z);
		return chunkFuture;
	}

	@Override
	public int getSeaLevel() {
		return lib_h.get_sea_level(generator);
	}

	/*
	 * the lowest value that blocks can be placed in the world. in a vanilla world,
	 * this is -64.
	 */
	@Override
	public int getMinimumY() {
		return lib_h.get_min_height(generator);
	}

	/*
	 * this method returns the height of the terrain at a given coordinate. it's
	 * used for structure generation
	 */
	@Override
	public int getHeight(int x, int z, Heightmap.Type heightmap, HeightLimitView world, NoiseConfig noiseConfig) {
		return lib_h.get_height(generator);
	}

	/*
	 * this method returns a "core sample" of the world at a given coordinate. it's
	 * used for structure generation
	 */
	@Override
	public VerticalBlockSample getColumnSample(int x, int z, HeightLimitView world, NoiseConfig noiseConfig) {
		BlockState[] blocks = new BlockState[0];
		return new VerticalBlockSample(0, blocks);
	}

	@Override
	public void generateFeatures(StructureWorldAccess world, Chunk chunk, StructureAccessor structureAccessor) {
	}

	@Override
	public void setStructureStarts(
			DynamicRegistryManager registryManager,
			StructurePlacementCalculator placementCalculator,
			StructureAccessor structureAccessor,
			Chunk chunk,
			StructureTemplateManager structureTemplateManager
	) {}

	/*
	 * this method adds text to the f3 menu. for NoiseChunkGenerator, it's the
	 * NoiseRouter line
	 */
	@Override
	public void getDebugHudText(List<String> text, NoiseConfig noiseConfig, BlockPos pos) {

	}

	@Override
	protected MapCodec<? extends ChunkGenerator> getCodec() {
		return CODEC;
	}
}