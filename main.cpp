#include <string_view>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <memory>
#include "3rd\nlohmann_json\include\nlohmann\json.hpp"
#include "3rd\nlohmann_json_schema_validator\src\nlohmann\json-schema.hpp"

using IDType = std::u8string_view;
using DataType = nlohmann::json;
using Validator = nlohmann::json_schema::json_validator;
using UUID = std::tuple<std::uint64_t, std::uint64_t>;
struct BlockCoordinate {
    std::uint64_t x,y,z;
}
struct EntityCoordinate {
    std::uint64_t x,y,z;
}

static const std::unordered_map<IDType, std::unordered_set<IDType>> block_tag_table;
static const std::unordered_map<IDType, std::unordered_set<IDType>> entity_tag_table;
static const std::unordered_map<IDType, std::unordered_set<IDType>> item_tag_table;

static const std::unordered_map<IDType, Validator> block_validator_table;
static const std::unordered_map<IDType, Validator> entity_validator_table;
static const std::unordered_map<IDType, Validator> item_validator_table;

struct Entity {
    EntityCoordinate pos;
    DataType data;
}

struct Biome {
    enum class Weather {
        Sunny,
        Rainy,
        ThunderStorm,
        Snowy
    };
    enum class Difficulty {
        Easy,
        Normal,
        Hard
    };
    std::u8string_view type_id;
    Weather weather;
    Difficulty diffculty;
    DataType blocks;
}

struct GameMap {
    std::unordered_map<BlockCoordinate, Biome> biomes;
    std::unordered_map<UUID, Entity> entities;
    std::unordered_map<IDType, DataType> global_data;
}