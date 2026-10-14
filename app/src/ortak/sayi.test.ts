import { describe, expect, it } from "vitest";
import { kisalt, sure, yuzde, yuzdeTam } from "./sayi";

describe("sayi yardimcilari", () => {
  it("sayilari kisaltir", () => {
    expect(kisalt(999)).toBe("999");
    expect(kisalt(41_234)).toBe("41.2K");
    expect(kisalt(1_234_567)).toBe("1.23M");
    expect(kisalt(1_234_567_890)).toBe("1.23B");
  });

  it("orani yuzdeye cevirir", () => {
    expect(yuzde(0.4441)).toBe("%44.4");
    expect(yuzde(0.4441, "en")).toBe("44.4%");
  });

  it("yuzde degerini tam sayiya cevirir", () => {
    expect(yuzdeTam(54.4)).toBe("%54");
    expect(yuzdeTam(54.4, "en")).toBe("54%");
  });

  it("sureyi saat ve dakika olarak gosterir", () => {
    expect(sure(61_000)).toBe("1:01");
    expect(sure(3_661_000)).toBe("1:01:01");
  });
});
