#include <string_view>
#include <vector>
#include <map>
#include <memory>

struct Block {
    std::u8string_view type_id;
    std::unordered_map<char8_t *, void *> *interface_table;
    virtual std::vector<std::uint8_t> serialize() const = 0;
    virtual ~Block() {};
}
static const std::map<std::u8string_view, std::unique_ptr<Block> (*)(const std::span<std::uint8_t>)> block_desrialize_func_table;

struct Entity {
    std::u8string_view type_id;
    double x,y,z;
    std::unordered_map<char8_t *, void *> *interface_table;
    virtual std::vector<std::uint8_t> serialize() const = 0;
    virtual ~Entity() {};
}
static const std::map<std::u8string_view, std::unique_ptr<Entity> (*)(const std::span<std::uint8_t>)> entity_desrialize_func_table;

struct Item {
    std::u8string_view type_id;
    std::unordered_map<char8_t *, void *> *interface_table;
    virtual std::vector<std::uint8_t> serialize() const = 0;
    virtual ~Item() {};
}
static const std::map<std::u8string_view, std::unique_ptr<Item> (*)(const std::span<std::uint8_t>)> item_desrialize_func_table;

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
}