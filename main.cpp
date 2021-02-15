#include <cstdint>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include "thirdparty/nlohmann_json/single_include/nlohmann/json.hpp"

using IDType = std::u8string_view;
using Json = nlohmann::json;

using DataFixer = void (*)(Json &);

struct BiomeCoordinate {
    std::uint32_t x,y,z;
};

static const std::unordered_map<IDType, std::unordered_set<IDType>> block_tag_table;
static const std::unordered_map<IDType, std::unordered_set<IDType>> entity_tag_table;
static const std::unordered_map<IDType, std::unordered_set<IDType>> item_tag_table;

static const std::unordered_map<IDType, DataFixer> block_fixer_table;
static const std::unordered_map<IDType, DataFixer> entity_fixer_table;
static const std::unordered_map<IDType, DataFixer> item_fixer_table;

struct GameMap {
    std::unordered_map<BiomeCoordinate, Json> biomes;
    Json global_data;
};

int main(){}
