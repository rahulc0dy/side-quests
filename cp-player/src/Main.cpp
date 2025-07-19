#include <iostream>
#include <ostream>

#include "Player.hpp"

int main() {
    try {
        Player player;
        player.start();
    } catch (std::runtime_error& error) {
        std::cerr << "Error: " << error.what() << std::endl;
    } catch (...) {
        std::cerr << "Unknown error." << std::endl;
    }

    return 0;
}
