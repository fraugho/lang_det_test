#include <iostream>
#include <ratio>
#include <string>
#include "lang_det.h"
#include <chrono>

int main() {
    //std::string text = "Hello, how are you?";
    std::string text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?";
    int times = 100;
    auto start = std::chrono::high_resolution_clock::now();
    for(int i = 0; i < times; ++i){
        LanguageEnum lang = whichlang_process_string(text.c_str());
    }
    /*
    switch(lang) {
        case EN:
            std::cout << "Detected: English" << std::endl;
            break;
        case ES:
            std::cout << "Detected: Spanish" << std::endl;
            break;
        case FR:
            std::cout << "Detected: French" << std::endl;
            break;
        default:
            std::cout << "Unknown language" << std::endl;
    }
    */
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = end - start;
    std::cout << "average processing time: " << std::chrono::duration_cast<std::chrono::microseconds>(duration).count() / times << "\n";
    
    return 0;
}
