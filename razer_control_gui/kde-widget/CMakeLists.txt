cmake_minimum_required(VERSION 3.16)

project(razercontrol-widget)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

find_package(ECM REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})

find_package(Qt6 REQUIRED COMPONENTS Core Gui DBus Widgets)
find_package(Plasma REQUIRED)
find_package(KF6 REQUIRED COMPONENTS 
    I18n
    CoreAddons
    Config
    ConfigWidgets
)

add_library(razercontrol-widget MODULE
    src/razercontrolwidget.cpp
    src/daemoncommunicator.cpp
    src/resources.qrc
)

target_link_libraries(razercontrol-widget
    Qt6::Core
    Qt6::Gui
    Qt6::DBus
    Qt6::Widgets
    KF6::I18n
    KF6::CoreAddons
    KF6::ConfigCore
    KF6::ConfigWidgets
    Plasma::Plasma
)

install(TARGETS razercontrol-widget DESTINATION ${KDE_INSTALL_PLUGINDIR}/plasma/applets)
install(DIRECTORY package/ DESTINATION ${KDE_INSTALL_DATADIR}/plasma/plasmoids/com.github.encomjp.razercontrol)
