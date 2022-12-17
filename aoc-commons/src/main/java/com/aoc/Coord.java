package com.aoc;

public class Coord {
    private int x;
    private int y;

    public Coord(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() {
        return x;
    }

    public void setX(int x) {
        this.x = x;
    }

    public int getY() {
        return y;
    }

    public void setY(int y) {
        this.y = y;
    }

    @Override
    public String toString() {
        return "("+x+","+y+")";
    }

    @Override
    public boolean equals(Object obj) {
        if(obj instanceof Coord) {
            return (obj == this) || ((Coord)obj).getX() == x && ((Coord) obj).getY() == y;
        }

        return false;
    }

    @Override
    public int hashCode() {
        return toString().hashCode();
    }
}