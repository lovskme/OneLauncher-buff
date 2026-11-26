import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;

/** Exercises the real MCP/Calamus composition without loading game or client code. */
public final class OrnitheMappingRegressionTest {
    public static void main(String[] args) throws Exception {
        Path resources = java.nio.file.Paths.get(args[0]);
        Path result = Files.createTempFile("ornithe-regression-", ".srg");
        try {
            GenerateOrnitheSrg.main(new String[]{resources.resolve("notch-joined.srg").toString(),
                    resources.resolve("ornithe-intermediary.tiny").toString(), result.toString()});
            Map<String, String> methods = new HashMap<>();
            for (String line : Files.readAllLines(result, StandardCharsets.UTF_8)) {
                String[] p = line.split(" +");
                if (p.length == 5 && p[0].equals("MD:")) methods.put(p[3] + " " + p[4], p[1]);
            }
            expect(methods, "net/minecraft/client/Minecraft/func_152345_ab ()Z",
                    "net/minecraft/client/Minecraft/m_33616981");
            expect(methods, "net/minecraft/client/renderer/texture/AbstractTexture/func_110552_b ()I",
                    "net/minecraft/unmapped/C_52359403/m_94041999");
            expect(methods, "net/minecraft/client/entity/EntityPlayerSP/func_145747_a (Lnet/minecraft/util/IChatComponent;)V",
                    "net/minecraft/unmapped/C_57778754/m_28323918");
            expect(methods, "net/minecraft/client/Minecraft/func_147108_a (Lnet/minecraft/client/gui/GuiScreen;)V",
                    "net/minecraft/client/Minecraft/m_52715402");
            if (!java.util.Arrays.equals(Files.readAllBytes(result),
                    Files.readAllBytes(resources.resolve("ornithe-joined.srg")))) {
                throw new AssertionError("Bundled Ornithe mapping differs from generated mapping");
            }
            System.out.println("Ornithe interface aliases, GUI entry point and bundled snapshot: PASS");
        } finally {
            Files.deleteIfExists(result);
        }
    }

    private static void expect(Map<String, String> methods, String key, String expected) {
        if (!expected.equals(methods.get(key))) {
            throw new AssertionError(key + ": expected " + expected + ", got " + methods.get(key));
        }
    }
}
