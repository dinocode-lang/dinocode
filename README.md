# 🦖 DinoCode

[![English](https://img.shields.io/badge/Read%20in-English-blue?style=flat-square)](./README.en.md)

**DinoCode** es un lenguaje de programación basado en la **inferencia de la intención** del programador, diseñado específicamente para eliminar la fricción sintáctica mediante el paradigma de la **Regla de Oro**.

> [!IMPORTANT]
> DinoCode es el núcleo técnico de un proyecto de titulación. Actualmente se distribuye en **binarios compilados** para pruebas de usabilidad. El código fuente permanece privado hasta la sustentación del grado, donde se pretende liberarlo.
---

## La Regla de Oro

### Pilares principales:
- **Inferencia de intención:** El motor de DinoCode interpreta la topografía del código para deducir la estructura lógica.
- **Regla de Oro:** La sintaxis es una propiedad emergente de la intención del programador, no una imposición del compilador.
- **Arquitectura optimizada:** La implementación actual está escrita en Rust y traduce el código fuente directamente a una máquina virtual con complejidad lineal $O(n)$, sin pasar por un AST (Abstract Syntax Tree) convencional.

---

## Cómo probar DinoCode

**Opción 1: DinoIDE**

[![Descargar DinoIDE](https://img.shields.io/badge/DinoIDE-para%20Windows-008080?style=for-the-badge&logo=windows)](https://github.com/BlassGO/DinoIDE)

**Opción 2: Línea de comandos (Linux/Windows)**
1. **Descarga el binario:** Ve a la sección de [Releases](https://github.com/dinocode-lang/dinocode/releases).
2. **Ejecuta un script**
   ```bash
    ./dinocode programa.dino
   ```

## ¡Tu opinión cuenta!

Si ya probaste el lenguaje, me encantaría conocer tu experiencia. Ayúdame llenando esta encuesta para la validación de la usabilidad del lenguaje, me ayudaría un montón:

 **[Encuesta de usabilidad de DinoCode](https://forms.gle/ZcNhLoLjDSZ6FDnQ9)**

### Ejemplo de sintaxis

```dinocode

:suma a b
    return a + b

:main args
    print "Hola mundo!"

    x = 10
    y = 1.5

    print [   # Nueva lista

        # Suma
        x+y

        # Multiplicación
        x*y
    ]
    
    for arg in args
        print arg

```

## Otros ejemplos

[![Ir a Pruebas de Benchmark](https://img.shields.io/badge/Ir%20a-Pruebas%20de%20Benchmark-green?style=flat-square)](./ejemplos/3_avanzado/4_benchmarking.dino)

1. [`ejemplos/`](./ejemplos/)
   1. [`Regla de Oro`](./ejemplos/1_regla_de_oro/)
      1. [`Introducción`](./ejemplos/1_regla_de_oro/1_introduccion)
      2. [`Continuidad Operativa`](./ejemplos/1_regla_de_oro/2_continuidad_operativa.dino)
      3. [`Inferencia de Intención`](./ejemplos/1_regla_de_oro/3_inferencia_de_intencion.dino)
      4. [`Consideraciones`](./ejemplos/1_regla_de_oro/4_consideraciones.dino)
      5. [`En la Práctica`](./ejemplos/1_regla_de_oro/5_en_la_practica.dino)
   2. [`Sintaxis Básica`](./ejemplos/2_basico/)
      1. [`Aritmética`](./ejemplos/2_basico/1_aritmetica.dino)
      2. [`Sintaxis Flexible`](./ejemplos/2_basico/2_sintaxis_flexible.dino)
      3. [`Interpolación`](./ejemplos/2_basico/3_interpolacion.dino)
      4. [`Strings`](./ejemplos/2_basico/4_strings.dino)
      5. [`Control de Flujo`](./ejemplos/2_basico/5_control_de_flujo.dino)
      6. [`Funciones`](./ejemplos/2_basico/6_funciones.dino)
      7. [`Función Main`](./ejemplos/2_basico/7_la_funcion_main.dino)
      8. [`Matrices`](./ejemplos/2_basico/8_matrices.dino)
      9. [`Objetos`](./ejemplos/2_basico/9_objetos.dino)
      10. [`Objetos Nuevos`](./ejemplos/2_basico/10_objetos_nuevos.dino)
      11. [`Dollar Call`](./ejemplos/2_basico/11_dollar_call.dino)
      12. [`Templates`](./ejemplos/2_basico/12_templates.dino)
   3. [`Avanzados`](./ejemplos/3_avanzado/)
      1. [`BigIntegers`](./ejemplos/3_avanzado/1_bigintegers.dino)
      2. [`Otros Números`](./ejemplos/3_avanzado/2_otros_numeros.dino)
      3. [`Métodos de Arrays`](./ejemplos/3_avanzado/3_metodos_de_arrays.dino)
      4. [`Métodos de Objetos`](./ejemplos/3_avanzado/4_metodos_de_objetos.dino)
      5. [`Benchmarking`](./ejemplos/3_avanzado/5_benchmarking.dino)
      6. [`Calculadora`](./ejemplos/3_avanzado/calculadora.dino)
      7. [`Consola Library`](./ejemplos/3_avanzado/console_library.dino)
      8. [`Fibonacci`](./ejemplos/3_avanzado/fibonacci.dino)

---

## ⚖️ Autoría y licencia

DinoCode es una obra tecnológica original. La arquitectura técnica, el diseño del motor de inferencia y la implementación íntegra en Rust son propiedad exclusiva del autor. Ningún tercero ha participado en la investigación técnica, la lógica del motor ni en su código fuente.
* **Autor:** Ismael Quiroz ([@BlassGO](https://github.com/BlassGO))

### Términos de uso

Actualmente, el proyecto se distribuye bajo la licencia **Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0)**.

> [!NOTE]
> Esta licencia restrictiva se mantiene vigente mientras el proyecto atraviesa su fase de validación académica y sustanciación de tesis. El autor tiene la intención de transicionar hacia una **licencia más permisiva** una vez concluido el proceso de titulación y liberado el código fuente.

Para más detalles, consulte el archivo [LICENSE](./LICENSE) de este repositorio.

---
