package com.yuta;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Main {
    private static Map<String, HeavenlyBody> solarSystem = new HashMap<>();
    private static Set<HeavenlyBody> planets = new HashSet<>();

    public static void main(String[] args) {
        HeavenlyBody planet = new HeavenlyBody("Mercury", 365);
        solarSystem.put(planet.getName(), planet);
        planets.add(planet);

        planet = new HeavenlyBody("Venus", 365);
        solarSystem.put(planet.getName(), planet);
        planets.add(planet);

        planet = new HeavenlyBody("Earth", 365);
        planet.addMoon(new HeavenlyBody("Moon", 30));
        solarSystem.put(planet.getName(), planet);
        planets.add(planet);

        planet = new HeavenlyBody("Mars", 365);
        planet.addMoon(new HeavenlyBody("Phobos", 30));
        planet.addMoon(new HeavenlyBody("Deimos", 30));
        solarSystem.put(planet.getName(), planet);
        planets.add(planet);

        planet = new HeavenlyBody("Jupyter", 365);
        planet.addMoon(new HeavenlyBody("Io", 30));
        planet.addMoon(new HeavenlyBody("Europa", 30));
        planet.addMoon(new HeavenlyBody("Ganymede", 30));
        planet.addMoon(new HeavenlyBody("Callisto", 30));
        solarSystem.put(planet.getName(), planet);
        planets.add(planet);

        planet = new HeavenlyBody("Saturn", 365);
        solarSystem.put(planet.getName(), planet);
        planets.add(planet);

        planet = new HeavenlyBody("Uranus", 365);
        solarSystem.put(planet.getName(), planet);
        planets.add(planet);

        planet = new HeavenlyBody("Neptune", 365);
        solarSystem.put(planet.getName(), planet);
        planets.add(planet);

        planet = new HeavenlyBody("Neptune", 3650);
        solarSystem.put(planet.getName(), planet);
        planets.add(planet);

        // list satellites of a planet
        HeavenlyBody tmp = solarSystem.get("Mars");
        for (HeavenlyBody body: tmp.getSatellites()) {
            System.out.println(body.getName());
        }

        // list all satellites
        System.out.println("----- All planets -----");
        Set<HeavenlyBody> satellites = new HashSet<>();
        for (HeavenlyBody body: solarSystem.values()) {
            System.out.println(body.getName() + ": orbital period " + body.getOrbitalPeriod());
            satellites.addAll(body.getSatellites());
        }

        System.out.println("----- All satellites -----");
        for (HeavenlyBody body: satellites) {
            System.out.println(body.getName());
        }

        System.out.println("----- All satellites in planets hashset -----");
        for (HeavenlyBody body: planets) {
            System.out.println(body.getName() + ": orbital period " + body.getOrbitalPeriod());
        }
    }
}
